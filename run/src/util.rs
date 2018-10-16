pub fn strip(chrs:&mut Vec<char>) {
  loop {
    match chrs.last().cloned() {
      None => break,
      Some (c) => {
        if !c.is_whitespace()
        { break; }
        chrs.pop();
      }
    }
  }
}
