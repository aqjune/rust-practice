use std::mem::transmute;

pub struct Lz78State {
    pub trie: Vec<[usize; 256]>,  // a trie
    pub trie_idx: usize, // currently it is at trie[trie_idx]
}

pub fn lz78_state_new() -> Lz78State{
    let mut state:Lz78State;
    // Init trie
    state.trie.clear();
    // Invalid node is 0
    state.trie.push([0; 26]);
    // Root node is 1
    state.trie.push([0; 26]);
    // Set the current node to the root
    state.trie_idx = 1;
    // Return state
    return state
}

pub fn lz78_encode(data: &Vec<u8>, encoded: &mut Vec<(usize, u8)>,
                   state: &mut Lz78State) {
    let state:Lz78State = lz78_state_new()

    for d in data {
        if encoded.trie[encoded.trie_idx][d] == 0 {
            // okay, done!
            // As the root of trie is 1, encoded.trie_idx - 1 is the index of
            // entry in the dictionary.
            encoded.push((entry.trie_idx - 1, d))
            // Add a new node to trie
            encoded.trie[encoded.trie_idx][d] = encoded.size()
            encoded.trie.push([0; 26])
            // Now, go to the root
            encoded.trie_idx = 1
        } else {
            encoded.trie_idx = encoded.trie[encoded.trie_idx][d]
        }
    }
}

pub enum Lz78EncodeProc {
    Naive
}

// Compresses the encoded data further
pub fn lz78_encode_postprocess(encoded: &Vec<(usize, u8)>, encoded_postproc:&mut Vec<u8>,
                               postproc_method: Lz78EncodeProc) {
    match postproc_method {
        Lz78EncodeProc::Naive =>
            for (idx, chr) in encoded {
               let bytes: [u8; 8] = unsafe { transmute(idx.to_le()) }; // little endian
                encoded_postproc.extend_from_slice(&bytes);
                encoded_postproc.push(chr);
            },
    }
}


/*
 * DECODE
 */

pub fn lz78_decode_preproc(bytes: &Vec<u8>, encoded: &mut Vec<usize, u8>,
                           preproc_method: Lz78EncodeProc) {
    match preproc_method {
        Lz78EncodeProc::Naive =>
            let mut idx = 0;
            assert!(bytes.size() % 9 == 0);
            while idx < bytes.size() {
                let idx_bytes = bytes[idx:idx + 7];
                let chr = bytes[idx + 8];
                let idx:usize = { transmute::<[u8; 8], usize>(idx) };
                encoded.push((idx, chr))
                idx = idx + 9;
            }
    }
}

pub fn lz78_decode(encoded:&Vec<usize, u8>, decoded: &mut Vec<u8>) {

}