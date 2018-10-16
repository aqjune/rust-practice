
pub struct ahc_trie {
  pub trie:Vec<[usize; 26]>, // a trie
  pub failure:Vec<usize>, // failure function (trie node -> trie node)
  pub output:Vec<Vec<usize>>, // output function (trie node -> Vec<query idx>)
}

fn as_idx(c:char) -> usize {
  return ((c as u8) - ('a' as u8)) as usize;
}

pub fn build_trie(queries_vec:&Vec<Vec<char>>, t:&mut ahc_trie) {
  // Build a trie.
  t.trie.push([0; 26]); // node 0 is an invalid node.
  t.failure.push(0);
  t.output.push(Vec::new());
  t.trie.push([0; 26]); // node 1 is a root node.
  t.failure.push(0); // failure of root is 0
  t.output.push(Vec::new());

  for qidx in 0..queries_vec.len() {
    let mut p:usize = 1;
    for idx in 0..queries_vec[qidx].len() {
      // Process character queries_vec[qidx][idx].
      let ic:usize = as_idx(queries_vec[qidx][idx]);

      if t.trie[p][ic] == 0 {
        // create a new node & define it as a child!
        let newp = t.trie.len();
        t.trie.push([0; 26]);
        t.trie[p][ic] = newp; // becomes child

        // calculate failure
        let mut f = t.failure[p];
        while f != 0 {
          if t.trie[f][ic] != 0 {
            // found!
            f = t.trie[f][ic];
            break;
          } else {
            // rewind
            f = t.failure[f];
          }
        }
        if f == 0 {
          // assign root node for failure
          f = 1;
        }
        t.failure.push(f);

        // output will be updated later. :)
        t.output.push(Vec::new());
      }
      // update p
      assert!(p < t.trie.len(), "trie[{}][{}] ( == {}) < {}", p, ic, t.trie[p][ic], t.trie.len());
      p = t.trie[p][ic];
    }
    t.output[p].push(qidx);
    //println!("Inserting {} to output[{}]", qidx, p);
  }
}

pub fn run(text_vec:&Vec<char>, queries_vec:&Vec<Vec<char>>,
           t:&ahc_trie, results:&mut Vec<Vec<usize>>) {
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
      if t.trie[p][ic] == 0 {
        // no corresponding child!
        // follow failure
        p = t.failure[p];
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
      p = t.trie[p][ic];
      assert!(p != 0, "trie[{}][{}] ( == {}) != 0", p, ic, t.trie[p][ic]);
    }

    // If there is a match..
    // Follow failure links to aggregate outputs!
    // For example, if queries "abab", "ab", "b" are given,
    // matching "abab" means "ab" and "b" are also matched!
    let mut op = p;
    while op != 1 {
      for oidx in 0..t.output[op].len() {
        // query queries_vec[qidx] matches!
        let qidx = t.output[op][oidx];
        results[qidx].push(idx + 1 - queries_vec[qidx].len());
      }
      op = t.failure[op];
    }
  }

  for idx in 0..results.len() {
    results[idx].dedup()
  }
}
