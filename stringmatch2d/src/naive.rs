/*
 * Naive implementation of 2-dimensional string match.
 * Returns: result; a list of left-upper most coordinates in (y, x) form
 */
pub fn run(
    text_vec: &Vec<Vec<char>>,
    query_vec: &Vec<Vec<char>>,
    result: &mut Vec<(usize, usize)>,
) {
    let thei: usize = text_vec.len();
    let twid: usize = text_vec[0].len(); // Assumes that there is at least a character in text_vec;
    let qhei: usize = query_vec.len();
    let qwid: usize = query_vec[0].len();

    for i in 0..thei + 1 - qhei {
        for j in 0..twid + 1 - qwid {
            let mut found: bool = true;

            for ii in 0..qhei {
                for jj in 0..qwid {
                    if text_vec[i + ii][j + jj] != query_vec[ii][jj] {
                        found = false;
                        break;
                    }
                }
                if !found {
                    break;
                }
            }
            if found {
                result.push((i, j));
            }
        }
    }
}
