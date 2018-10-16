// Builds prefix-suffix array.
// query_vec: a string of query
// pfxsfx: the output
pub fn build_pfxsfx(query_vec:&Vec<u16>, pfxsfx:&mut Vec<usize>) {
  // pfxsfx:The length of longest prefix-suffix match
  assert! (pfxsfx.len() == query_vec.len(), "pfxsfx.len() == query_vec.len()");
  pfxsfx[0] = 0;
  for idx in 1..(query_vec.len()) {
    let mut common_prefix_len = pfxsfx[idx - 1];
    loop {
      if query_vec[common_prefix_len] == query_vec[idx] {
        // found.
        pfxsfx[idx] = common_prefix_len + 1;
        break;
      } else {
        if common_prefix_len == 0 {
          pfxsfx[idx] = 0;
          break;
        }
        common_prefix_len = pfxsfx[common_prefix_len - 1];
      }
    }
  }
}

pub fn run(text_vec:&Vec<u16>, query_vec:&Vec<u16>, pfxsfx:&Vec<usize>, result:&mut Vec<usize>) {
  // Now, find the match
  let mut qidx = 0;
  for idx in 0..(text_vec.len()) {
    loop {
      if text_vec[idx] == query_vec[qidx] {
        qidx = qidx + 1;
        if qidx == query_vec.len() {
          result.push(idx + 1 - qidx);
          qidx = pfxsfx[qidx - 1];
        }
        break;
      } else {
        if qidx == 0 { break };
        qidx = pfxsfx[qidx - 1];
      }
    }
  }
}
