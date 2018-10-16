
fn as_idx(c:char) -> usize {
  return ((c as u8) - ('a' as u8)) as usize;
}

fn stringmatch(text_vec:&Vec<char>, queries_vec:&Vec<Vec<char>>,
               trie:&Vec<[usize; 26]>, failure:&Vec<usize>, output:&Vec<Vec<usize>>,
               results:&mut Vec<Vec<usize>>) {
  // Initialize results
  for _ in 0..queries_vec.len() {
    results.push(Vec::new());
  }

  let mut p = 1; // start at the root node!
  for idx in 0..text_vec.len() {
    let ic:usize = as_idx(text_vec[idx]);

    // while condition: only root node's failure is 0
    // other nodes' failure is bigger than 0
    while p != 0 {
      if trie[p][ic] == 0 {
        // no corresponding child!
        // follow failure
        p = failure[p];
      } else {
        // found the node!
        break;
      }
    }
    if p == 0 {
      // start at root again
      p = 1;
    } else {
      // point to the child (next character)
      p = trie[p][ic];
      assert!(p != 0, "trie[{}][{}] ( == {}) != 0", p, ic, trie[p][ic]);
    }

    // If there is a match..
    // Follow failure links to aggregate outputs!
    // For example, if queries "abab", "ab", "b" are given,
    // matching "abab" means "ab" and "b" are also matched!
    let mut op = p;
    while op != 1 {
      for oidx in 0..output[op].len() {
        // query queries_vec[qidx] matches!
        let qidx = output[op][oidx];
        results[qidx].push(idx + 1 - queries_vec[qidx].len());
      }
      op = failure[op];
    }
  }
}

pub fn run(text_vec:&Vec<char>, queries_vec:&Vec<Vec<char>>, results:&mut Vec<Vec<usize>>) {
  let mut trie:Vec<[usize; 26]> = Vec::new(); // a trie
  let mut failure:Vec<usize> = Vec::new(); // failure function (trie node -> trie node)
  let mut output:Vec<Vec<usize>> = Vec::new(); // output function (trie node -> Vec<query idx>)

  // Build a trie.
  trie.push([0; 26]); // node 0 is an invalid node.
  failure.push(0);
  output.push(Vec::new());
  trie.push([0; 26]); // node 1 is a root node.
  failure.push(0); // failure of root is 0
  output.push(Vec::new());

  for qidx in 0..queries_vec.len() {
    let mut p:usize = 1;
    for idx in 0..queries_vec[qidx].len() {
      // Process character queries_vec[qidx][idx].
      let ic:usize = as_idx(queries_vec[qidx][idx]);

      if trie[p][ic] == 0 {
        // create a new node & define it as a child!
        let newp = trie.len();
        trie.push([0; 26]);
        trie[p][ic] = newp; // becomes child

        // calculate failure
        let mut f = failure[p];
        while f != 0 {
          if trie[f][ic] != 0 {
            // found!
            f = trie[f][ic];
            break;
          } else {
            // rewind
            f = failure[f];
          }
        }
        if f == 0 {
          // assign root node for failure
          f = 1;
        }
        failure.push(f);

        // output will be updated later. :)
        output.push(Vec::new());
      }
      // update p
      assert!(p < trie.len(), "trie[{}][{}] ( == {}) < {}", p, ic, trie[p][ic], trie.len());
      p = trie[p][ic];
    }
    output[p].push(qidx);
    //println!("Inserting {} to output[{}]", qidx, p);
  }

  // Okay, building trie / failure function / output function is done!
  // Let's start matching!
  stringmatch(text_vec, queries_vec, &trie, &failure, &output, results);
  for idx in 0..results.len() {
    results[idx].dedup()
  }
}
