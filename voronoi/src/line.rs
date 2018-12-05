pub struct Line {
  pub xbeg: f64,
  pub ybeg: f64,
  pub xend: f64,
  pub yend: f64,
  // If finite = false, line ends at (xend, yend)
  // Otherwise, line goes past (xend, yend)
  pub finite: bool
}