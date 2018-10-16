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

pub fn char_to_u16 (c:&char) -> u16 {
  let c2:char = *c;
  return c2 as u16;
}
