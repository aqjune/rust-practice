extern crate stringmatch;

/*
 * Baker-bird algorithm for 2-dimensional string match.
 * Time complexity: O(Nm^2 + n^2)
 *   (N = # of alphabets, m = pattern width (assuming that the pattern is a square),
 *    n = text width (assuming that the text is a square))
 * Space complexity: O(Nm^2 + n)
 */
pub fn run(
    text_vec: &Vec<Vec<char>>,
    query_vec: &Vec<Vec<char>>,
    result: &mut Vec<(usize, usize)>,
) {
    let nh = text_vec.len();
    let nw = text_vec[0].len();
    let mh = query_vec.len();
    let _mw = query_vec[0].len();

    // Build a trie for aho-corasick algorithm
    let mut ahc_t: stringmatch::ahocorasick::AhcTrie = stringmatch::ahocorasick::AhcTrie {
        trie: Vec::new(),
        failure: Vec::new(),
        output: Vec::new(),
    };
    stringmatch::ahocorasick::build_trie(query_vec, &mut ahc_t);

    // Now, let's identify duplicated rows using the built trie!
    // qu_idx[i] == j means that
    // query_vec[i] == query_vec[j] where query_vec[j] is the
    // "representative" of the equivalence class.
    let mut qu_idx: Vec<u16> = vec![65535; mh];
    for i in 0..query_vec.len() {
        // res[i][j]: if i'th query is matched, it is matched at text's index res[i][j].
        let mut res: Vec<Vec<usize>> = Vec::new();
        stringmatch::ahocorasick::run(&query_vec[i], &query_vec, &ahc_t, &mut res);

        // Get the 'minimum' node from res
        let mut found = false;
        for j in 0..query_vec.len() {
            if res[j].len() > 0 {
                assert!(res[j].len() == 1, "Only one match should be available");
                assert!(res[j][0] == 0, "Match should be at the beginning");
                found = true;
                qu_idx[i] = j as u16;
                break;
            }
        }
        assert!(found, "Should be found, because query matches query itself");
    }

    // Also build a prefix-suffix array for KMP
    let mut kmp_pfxsfx: Vec<usize> = vec![0; mh];
    stringmatch::kmp::build_pfxsfx(&qu_idx, &mut kmp_pfxsfx);

    // KMP matched index for each column of the text!
    let mut kmp_matchidx: Vec<usize> = vec![0; nw];

    // Now, visit each row of the text
    for i in 0..nh {
        let mut res: Vec<Vec<usize>> = Vec::new();
        let mut res_row: Vec<u16> = vec![65535; nw];
        stringmatch::ahocorasick::run(&text_vec[i], &query_vec, &ahc_t, &mut res);
        assert!(
            res.len() == mh,
            "|res| should be the height of the pattern map"
        );
        // Convert res => res_row
        // res[j]: result of matchings with j'th row in the pattern
        for j in 0..mh {
            if qu_idx[j] != j as u16 {
                // Not 'representative' row.
                // can ignore this match!
                continue;
            }
            for k in 0..res[j].len() {
                assert!(
                    res_row[res[j][k]] == 65535,
                    "There must be only one pattern which matches here"
                );
                res_row[res[j][k]] = j as u16;
            }
        }

        // Now run KMP for this row!
        // j is the idx of column of the text.
        for j in 0..nw {
            loop {
                if res_row[j] == qu_idx[kmp_matchidx[j]] {
                    // match!
                    kmp_matchidx[j] += 1;
                    if kmp_matchidx[j] == mh {
                        // 2d match done!
                        result.push((i + 1 - mh, j));
                        kmp_matchidx[j] = kmp_pfxsfx[kmp_matchidx[j] - 1];
                    }
                    break;
                } else {
                    if kmp_matchidx[j] == 0 {
                        break;
                    }
                    kmp_matchidx[j] = kmp_pfxsfx[kmp_matchidx[j] - 1];
                }
            }
        }
    }
}
