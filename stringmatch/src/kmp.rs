pub fn run(text_vec:&Vec<char>, query_vec:&Vec<char>, result:&mut Vec<usize>) {
  // prefix:The length of longest prefix-suffix match
  let mut prefix:Vec<usize> = vec![0; query_vec.len()];
  prefix[0] = 0;
  for idx in 1..(query_vec.len()) {
    let mut common_prefix_len = prefix[idx - 1];
    loop {
      if query_vec[common_prefix_len] == query_vec[idx] {
        // found.
        prefix[idx] = common_prefix_len + 1;
        break;
      } else {
        if common_prefix_len == 0 {
          prefix[idx] = 0;
          break;
        }
        common_prefix_len = prefix[common_prefix_len - 1];
      }
    }
  }
  // Now, find the match
  let mut qidx = 0;
  for idx in 0..(text_vec.len()) {
    loop {
      if text_vec[idx] == query_vec[qidx] {
        qidx = qidx + 1;
        if qidx == query_vec.len() {
          result.push(idx + 1 - qidx);
          qidx = prefix[qidx - 1];
        }
        break;
      } else {
        if qidx == 0 { break };
        qidx = prefix[qidx - 1];
      }
    }
  }
}