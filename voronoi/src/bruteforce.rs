use std::cmp::Ordering;
use std::collections::HashSet;
use line;

// Returns true if a and b are equal in small absolute/relative error.
fn eq_error(a:f64, b:f64) -> bool {
  let err:f64 = 0.00001;
  if (a - b).abs() < err {
    true
  } else {
    (if err < b.abs() { 1.0 - err < a / b && a / b < 1.0 + err } else { false }) &&
    (if err < a.abs() { 1.0 - err < b / a && b / a < 1.0 + err } else { false })
  }
}

fn sorted_tuple(i:usize, j:usize, k:usize) -> (usize, usize, usize) {
  let mut a:[usize; 3] = [i, j, k];
  a.sort();
  return (a[0], a[1], a[2]);
}

fn psub(p1:(f64, f64), p2:(f64, f64)) -> (f64, f64) {
  return (p1.0 - p2.0, p1.1 - p2.1)
}

fn normalize(p:(f64, f64)) -> (f64, f64) {
  let d = p.0.hypot(p.1);
  assert!(!eq_error(d, 0.0));
  return (p.0 / d, p.1 / d);
}

fn get_circumcenter(pi: (f64, f64), pj: (f64, f64), pk: (f64, f64))
    -> Option<(f64, f64)> {
  let (xi, yi) = pi;
  let (xj, yj) = pj;
  let (xk, yk) = pk;
  // First, they should not be on a line.
  // Check this using cross product :
  // |(xj - xi, yj - yi) x (xk - xi, yk - yi)| = 0
  // (xj - xi) * (yk - yi) - (yj - yi) * (xk - xi) = 0
  let crossprod = (xj - xi) * (yk - yi) - (yj - yi) * (xk - xi);
  if crossprod == 0.0 {
    None
  } else {
    // Get circumcenter of input[i, j, k]
    // and the circumcenter is (x, y), radius is r.
    // (xi - x)^2 + (yi - y)^2 = r^2  -- (1)
    // (xj - x)^2 + (yj - y)^2 = r^2  -- (2)
    // (xk - x)^2 + (yk - y)^2 = r^2  -- (3)
    //
    // (1) - (2) makes:
    // (2 * xj - 2 * xi)x + (2 * yj - 2 * yi)y = xj^2 - xi^2 + yj^2 - yi^2
    // (1) - (3) makes:
    // (2 * xk - 2 * xi)x + (2 * yk - 2 * yi)y = xk^2 - xi^2 + yk^2 - yi^2
    //
    // Represent this using matrix multiplication Ax = b:
    // |(2*xj - 2*xi)  (2*yj - 2*yi)| | x |   |xj^2 - xi^2 + yj^2 - yi^2|
    // |(2*xk - 2*xi)  (2*yk - 2*yi)| | y | = |xk^2 - xi^2 + yk^2 - yi^2|
    let a11 = 2.0 * xj - 2.0 * xi;
    let a12 = 2.0 * yj - 2.0 * yi;
    let a21 = 2.0 * xk - 2.0 * xi;
    let a22 = 2.0 * yk - 2.0 * yi;
    let b1 = xj.powi(2) - xi.powi(2) + yj.powi(2) - yi.powi(2);
    let b2 = xk.powi(2) - xi.powi(2) + yk.powi(2) - yi.powi(2);
    //
    // Inverse of the left 2 x 2 matrix is
    // |(2*yk - 2*yi)  (2*yi - 2*yj)|
    // |(2*xi - 2*xk)  (2*xj - 2*xi)| * 1/det
    let det = a11 * a22 - a12 * a21;
    let inv11 = a22 / det;
    let inv12 = -a12 / det;
    let inv21 = -a21 / det;
    let inv22 = a11 / det;
    let x = inv11 * b1 + inv12 * b2;
    let y = inv21 * b1 + inv22 * b2;
/*
    println!("<get_circumcenter>");
    println!("inputs: ({}, {}), ({}, {}), ({}, {})", xi, yi, xj, yj, xk, yk);
    println!("x = {}, y = {}", x, y);
*/
    let r = (x - xi).hypot(y - yi);

    assert!(eq_error(r, (x - xj).hypot(y - yj)));
    assert!(eq_error(r, (x - xk).hypot(y - yk)));

    Some((x, y))
  }
}


/*
output: a vector of lines (xbeg, ybeg, xend, yend)
*/
pub fn run(input:&Vec<(f64, f64)>, output:&mut Vec<line::Line>) {
  let mut voronoi_points:Vec<(f64, f64)> = Vec::new();
  let mut voronoi_vertices:Vec<Vec<usize> > = Vec::new();
  let mut visited_points:HashSet<(usize, usize, usize)> = HashSet::new();

  for i in 0..input.len() {
    for j in i+1..input.len() {
      for k in j+1..input.len() {
        if visited_points.contains(&(i, j, k)) {
          continue;
        }

        match get_circumcenter(input[i], input[j], input[k]) {
          None => continue,
          Some ((x, y)) => {
            let (xi, yi) = input[i];
            let r = (x - xi).hypot(y - yi);

            // Check whether there exists input[l] s.t. input[l]
            // is closer to (x,y) than input[i, j, k]
            let mut has_closer_pnt = false;
            for l in 0..input.len() {
              let (xl, yl) = input[l];
              let d = (xl - x).hypot(yl - y);
              if d < r && !eq_error(r, d) {
                has_closer_pnt = true;
                break;
              }
            }
            
            if has_closer_pnt {
              continue;
            }

            // Check all other points which are on the same circle.
            let mut contributing_points:Vec<usize> = Vec::new();
            contributing_points.push(i);
            contributing_points.push(j);
            contributing_points.push(k);

            for l in 0..input.len() {
              if l == i || l == j || l == k
              { continue; }

              let (xl, yl) = input[l];
              let d = (xl - x).hypot(yl - y);

              if eq_error(r, d) {
                // l is on the circle as well!
                contributing_points.push(l);
                visited_points.insert(sorted_tuple(i, j, l));
                visited_points.insert(sorted_tuple(i, k, l));
                visited_points.insert(sorted_tuple(j, k, l));
              }
            }
            // Add a new voronoi vertex, which is represented by
            // its surrounding input points.
            voronoi_vertices.push(contributing_points);
            voronoi_points.push((x, y));
          },
        }
      }
    }
  }

  // Now connect these dots!
  // Let's assume that x1, x2, .., xn are input vertices
  // which made voronoi vertex p (in other words: x1, x2, .., xn are
  // on the circle centered at p)
  // (Assume here that x1, x2, .. xn are sorted by its angle)
  // For 0 <= i < n,
  // (1) if there exists another voronoi vertex q which also has
  //     xi, xi+1 as its contributing points. then p and q are connected.
  // (2) If there's no such vertex, then there is an half-infinite line
  //     starting at p and crossing line (xi, xi+1).
  
  // Let's start by sorting voronoi_vertices[i][0..n] by angle..!
  for i in 0..voronoi_vertices.len() {
    let (px, py) = voronoi_points[i];
    // To sort by angle: we use cross product again.
    voronoi_vertices[i].sort_by(|j, k| {
        let (xj, yj) = normalize(psub(input[*j], (px, py)));
        let (xk, yk) = normalize(psub(input[*k], (px, py)));
        if xj * xk < 0.0 {
          // if xj is negative & xk is positive, we should swap it
          // f64 only has partial_cmp.
          xk.partial_cmp(&xj).unwrap_or(Ordering::Equal)
        } else {
          // if (xj, yj) x (xk, yk) > 0, we should swap it
          let crossprod = xj * yk - yj * xk;
          crossprod.partial_cmp(&0.0).unwrap_or(Ordering::Equal)
        }
      }
    );
    /*
    println!("Voronoi vertex {}, {}: After sorting:", px, py);
    for j in 0..voronoi_vertices[i].len() {
      print!("{} ", voronoi_vertices[i][j]);
    }
    println!("");
    */
  }
  // Now, let's make lines.
  for i in 0..voronoi_vertices.len() {
    let (px, py) = voronoi_points[i];
    for j in 0..voronoi_vertices[i].len() {
      let idx1 = voronoi_vertices[i][j];
      let idx2 = voronoi_vertices[i][(j + 1) % voronoi_vertices[i].len()];
      
      let mut connected_vv:Option<usize> = None;
      for i2 in 0..voronoi_vertices.len() {
        if i2 == i { continue; }
        // check whether i and i2 can be connected!
        for j2 in 0..voronoi_vertices[i2].len() {
          if voronoi_vertices[i2][j2] == idx2 &&
             voronoi_vertices[i2][(j2 + 1) % voronoi_vertices[i2].len()] == idx1 {
            assert!(connected_vv == None);
            connected_vv = Some(i2);
            // For debugging, just fall throguh (don't do break)
          }
        }
      }

      match connected_vv {
        None => { // Infinite line!
          let xdir = input[idx2].0 - input[idx1].0;
          let ydir = input[idx2].1 - input[idx1].1;
          // rotate (xdir, ydir) by -pi/2 & add it to (px,py)
          output.push(
            line::Line { xbeg: px, ybeg: py,
                         xend: px - ydir, yend: py + xdir,
                         finite: false }
          )
        },
        Some(i2) => { // Finite line.
          if i < i2 { // no duplication
            let (qx, qy) = voronoi_points[i2];
            output.push(
              line::Line { xbeg: px, ybeg: py, xend: qx, yend: qy,
                          finite: true }
            )
          }
        },
      }
    }
  }
}