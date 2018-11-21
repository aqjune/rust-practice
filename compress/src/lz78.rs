use std::mem::transmute;
use std::str;

pub struct State {
    pub trie: Vec<[usize; 256]>,  // a trie
    pub trie_parent: Vec<(usize, u8)>,  // trie_parent[n] : parent of trie node n & char
    pub trie_idx: usize, // currently it is at trie[trie_idx]
}

fn state_new() -> State{
    let mut state:State = State {
        trie: Vec::new(),
        trie_parent: Vec::new(),
        trie_idx: 0,
    };
    // Invalid node is 0
    state.trie.push([0; 256]);
    state.trie_parent.push((0, 0));
    // Root node is 1
    state.trie.push([0; 256]);
    state.trie_parent.push((0, 0));
    // Set the current node to the root
    state.trie_idx = 1;
    // Return state
    return state
}

fn addnode<'a>(parent:usize, chr:u8, encoded:&'a mut State) {
    encoded.trie[encoded.trie_idx][chr as usize] = encoded.trie.len();
    encoded.trie.push([0; 256]);
    encoded.trie_parent.push((parent, chr));
}

fn encode(data: &Vec<u8>, encoded: &mut Vec<(usize, u8)>) {
    let mut state:State = state_new();

    for d0 in data {
        let d:u8 = *d0;
        if state.trie[state.trie_idx][d as usize] == 0 {
            // okay, done!
            // As the root of trie is 1, encoded.trie_idx - 1 is the index of
            // entry in the dictionary.
            encoded.push((state.trie_idx - 1, d));
            // Add a new node to trie
            addnode(state.trie_idx, d, &mut state);
            // Now, go to the root
            state.trie_idx = 1
        } else {
            state.trie_idx = state.trie[state.trie_idx][d as usize];
        }
    }
}

pub enum Lz78EncodeProc {
    Naive,
    // uses high 1 bit of char & high 1 bit of index to encode the range of index of dictionary entry
    // 0mmmmmmm nnnnnnnn : char m, index n (n < 2^8), 
    // 1mmmmmmm 0nnnnnnn nnnnnnnn : char m, index n (2^8 <= n < 2^15)
    // 1mmmmmmm 1nnnnnnn nnnnnnnn nnnnnnnn : char m, index n (2^15 <= n < 2^23)
    UseCharHighBit
}

// Compresses the encoded data further
fn encode_postprocess(encoded: &Vec<(usize, u8)>, encoded_postproc:&mut Vec<u8>,
                               postproc_method: Lz78EncodeProc) {
    match postproc_method {
        Lz78EncodeProc::Naive =>
            for (idx, chr) in encoded {
               let bytes: [u8; 8] = unsafe { transmute(idx.to_le()) }; // little endian
                encoded_postproc.extend_from_slice(&bytes);
                encoded_postproc.push(*chr);
            },
        Lz78EncodeProc::UseCharHighBit =>
            for (idx, chr) in encoded {
                if *idx < usize::pow(2, 8) {
                    encoded_postproc.push(*chr);
                    encoded_postproc.push(*idx as u8);
                } else if *idx < usize::pow(2, 15) {
                    encoded_postproc.push(128 | *chr);
                    encoded_postproc.push(((*idx & 0xFF00) >> 8) as u8);
                    encoded_postproc.push((*idx & 0xFF) as u8);
                } else if *idx < usize::pow(2, 23) {
                    encoded_postproc.push(128 | *chr);
                    encoded_postproc.push(128 | (((*idx & 0xFF0000) >> 16) as u8));
                    encoded_postproc.push(((*idx & 0xFF00) >> 8) as u8);
                    encoded_postproc.push((*idx & 0xFF) as u8);
                } else {
                    // Too big..!
                    assert!(*idx < usize::pow(2, 23));
                }
            },
    }
}


pub fn compress(data0:&Vec<u8>, compressed:&mut Vec<u8>, encmethod:Lz78EncodeProc) {
    let mut encoded:Vec<(usize, u8)> = Vec::new();
    let mut data:Vec<u8> = data0.to_vec();
    data.push(127u8);
    encode(&data, &mut encoded);

    encode_postprocess(&encoded, compressed, encmethod);
}


/*
 * DECODE
 */

fn decode_preprocess(bytes: &Vec<u8>, encoded: &mut Vec<(usize, u8)>,
                           preproc_method: Lz78EncodeProc) {
    match preproc_method {
        Lz78EncodeProc::Naive => {
            let mut idx = 0;
            assert!(bytes.len() % 9 == 0);
            while idx < bytes.len() {
                let mut idx_bytes: [u8; 8] = Default::default();
                idx_bytes.copy_from_slice(&bytes[idx..idx + 8]);
                let chr = bytes[idx + 8];
                let trienode:usize = unsafe { transmute::<[u8; 8], usize>(idx_bytes) };
                encoded.push((trienode, chr));
                idx = idx + 9;
            }
        },
        Lz78EncodeProc::UseCharHighBit => {
            let mut idx = 0;
            while idx < bytes.len() {
                let mut chr:u8 = bytes[idx];
                let mut trienode:usize;
                idx = idx + 1;

                if chr & 128 == 0 {
                    // use one byte to encode the index of dictionary entry
                    trienode = bytes[idx] as usize;
                    idx = idx + 1;
                } else {
                    chr = chr & 127;
                    if bytes[idx] & 128 == 0 {
                        // use two bytes to encode
                        trienode = ((bytes[idx] as usize) << 8) + (bytes[idx + 1] as usize);
                        idx = idx + 2;
                    } else {
                        // use three bytes
                        trienode = (((bytes[idx] & 127) as usize) << 16) +
                                   ((bytes[idx + 1] as usize) << 8) + (bytes[idx + 2] as usize);
                        idx = idx + 3;
                    }
                }
                encoded.push((trienode, chr));
            }
        },
    }
}

fn decode(encoded:&Vec<(usize, u8)>, decoded: &mut Vec<u8>) {
    let mut st:State = state_new();
    for (nodeidx0, chr) in encoded {
        let mut nodeidx = nodeidx0 + 1;
        let mut the_str:Vec<u8> = Vec::new();
        while nodeidx != 1 {
            the_str.push(st.trie_parent[nodeidx].1);
            nodeidx = st.trie_parent[nodeidx].0;
        }
        the_str.reverse();
        decoded.append(&mut the_str);
        decoded.push(*chr);
        addnode(nodeidx0 + 1, *chr, &mut st);
    }
}

pub fn decompress(compressed:&Vec<u8>, data:&mut Vec<u8>, encmethod:Lz78EncodeProc) {
    let mut encoded:Vec<(usize, u8)> = Vec::new();
    decode_preprocess(compressed, &mut encoded, encmethod);

    decode(&encoded, data);
    data.pop();
}
