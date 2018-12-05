use std::collections::HashSet;
use lines;

// Returns true if a and b are equal in small absolute/relative error.
fn eq_error(a:f64, b:f64) {
  let err:f64 = 0.0000001;
  if (a - b).abs() < err {
    true
  }

  (if b != 0.0 && b != -0.0 { a / b < 1.0 + err } else { false }) &&
  (if a != 0.0 && a != -0.0 { b / a < 1.0 + err } else { false })
}

fn normalize(p:(f64, f64)) -> (f64, f64) {
  let d = p.0.hypot(p.1);
  assert!(!eq_error(d, 0));
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
  if crossprod == 0 {
    None
  }
  // Get circumcenter of input[i, j, k]
  // and the circumcenter is (x, y), radius is r.
  // (xi - x)^2 + (yi - y)^2 = r^2  -- (1)
  // (xj - x)^2 + (yj - y)^2 = r^2  -- (2)
  // (xk - x)^2 + (yk - y)^2 = r^2  -- (3)
  //
  // (1) - (2) makes:
  // (2 * xj - 2 * xi)x + (2 * yj - 2 * yi)y = xi^2 - xj^2 + yi^2 - yj^2
  // (1) - (3) makes:
  // (2 * xk - 2 * xi)x + (2 * yk - 2 * yi)y = xi^2 - xk^2 + yi^2 - yk^2
  //
  // Represent this using matrix multiplication Ax = b:
  // |(2*xj - 2*xi)  (2*yj - 2*yi)| | x |   |xi^2 - xj^2 + yi^2 - yj^2|
  // |(2*xk - 2*xi)  (2*yk - 2*yi)| | y | = |xi^2 - xk^2 + yi^2 - yk^2|
  let a11 = 2.0 * xj - 2.0 * xi,
      a12 = 2.0 * yj - 2.0 * yi,
      a21 = 2.0 * xk - 2.0 * xi,
      a22 = 2.0 * yk - 2.0 * yi;
  let b1 = xi.powi(2) - xj.powi(2) + yi.powi(2) - yj.powi(2),
      b2 = xi.powi(2) - xk.powi(2) + yi.powi(2) - yk.powi(2);
  //
  // Inverse of the left 2 x 2 matrix is
  // |(2*yk - 2*yi)  (2*yi - 2*yj)|
  // |(2*xi - 2*xk)  (2*xj - 2*xi)| * 1/det
  let det = a11 * a22 - a12 * a21;
  let inv11 = a22 / det,
      inv12 = -a12 / det,
      inv21 = -a21 / det,
      inv22 = a11 / det;
  let x = inv11 * b1 + inv12 * b2,
      y = inv21 * b1 + inv22 * b2;
  (x, y)
}


/*
output: a vector of lines (xbeg, ybeg, xend, yend)
*/
fn run(input:&vec<(f64, f64)>, output:&mut vec<Line>) {
  let mut voronoiVertices:vec<vec<usize> > = Vec::new();
  let mut visitedPoints = HashSet::new();

  for i in 0...input.len() {
    for j in i+1...input.len() {
      for k in j+1...input.len() {
        if visitedPoints.contains((i, j, k)):
          continue;

        let (x, y) = get_circumcenter(input[i], input[j], input[k]);
        let (xi, yi) = input[i];
        let (xj, yj) = input[j];
        let (xk, yk) = input[k];
        let r = (x - xi).hypot(y - yi);

        assert!(eq_error(r, (x - xj).hypot(y - yj)));
        assert!(eq_error(r, (x - xk).hypot(y - yk)));

        // Check whether there exists input[l] s.t. input[l]
        // is closer to (x,y) than input[i, j, k]
        let mut hasCloserPnt = false;
        for l in 0...input.len() {
          let (xl, yl) = input[l];
          let d = (xl - x).hypot(yl - y);
          if d < r && !eq_error(r, d) {
            hasCloserPnt = true;
            break;
          }
        }
        
        if hasCloserPnt {
          continue;
        }

        // Check all other points which are on the same circle.
        let mut contributingPoints:vec<usize> = Vec::new();
        contributingPoints.push(i);
        contributingPoints.push(j);
        contributingPoints.push(k);

        for l in k+1...input.len() {
          let (xl, yl) = input[l];
          let d = (xl - x).hypot(yl - y);

          if eq_error(r, d) {
            // l is on the circle as well!
            contributing_points.push(l);
            visitedPoints.insert((i, j, l));
            visitedPoints.insert((i, k, l));
            visitedPoints.insert((j, k, l));
          }
        }
        // Add a new voronoi vertex, which is represented by
        // its surrounding input points.
        voronoiVertices.push(contributingPoints);
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
  
  // Let's start by sorting voronoiVertices[i][0..n] by angle..!
  for i in 0..voronoiVertices.len() {
    let (px, py) = get_circumcenter(voronoiVertices[i][0], voronoiVertices[i][1],
                             voronoiVertices[i][2]);
    // To sort by angle: we use cross product again.
    voronoiVertices[i].sort_by(|j, k|
      let (xj, yj) = normalize(input[j] - (px, py));
      let (xk, yk) = normalize(input[k] - (px, py));
      if xj * xk < 0.0 {
        // if xj is negative & xk is positive, we should swap it
        xk.cmp(xj)
      } else {
        // if (xj, yj) x (xk, yk) > 0, we should swap it
        let crossprod = xj * yk - yj * xk;
        crossprod.cmp(0.0)
      }
    );
  }
  // Now, let's make lines.
  for i in 0..voronoiVertices.len() {
    let (px, py) = get_circumcenter(voronoiVertices[i][0], voronoiVertices[i][1],
                             voronoiVertices[i][2]);
    for j in 0..voronoiVertices[i].len() {
      let idx1 = voronoiVertices[i][j];
      let idx2 = voronoiVertices[i][(j + 1) % voronoiVertices[i].len()];
      
      let connectedVV:Option<usize> = None;
      for i2 in i+1..voronoiVertices.len() {
        // check whether i and i2 can be connected!
        for j2 in 0..voronoiVertices[i2].len() {
          if voronoiVertices[i2][j2] == idx1 &&
             voronoiVertices[i2][(j2 + 1) % voronoiVertices[i2].len()] == idx2 {
            assert!(connectedVV == None);
            connectedVV = Some(i2);
            // For debugging, just fall throguh (don't do break)
          }
        }
      }

      match connectedVV {
        None => // Infinite line!
          let xmid (inputs[idx1].0 + inputs[idx2].0) / 2,
              ymid = (inputs[idx1].1 + inputs[idx2].1) / 2;
          output.push(
            Line { xbeg: px, ybeg: py, xend: xmid, yend: ymid,
                   finite: false }
          ),
        Some(i2) => // Finite line.
          let (qx, qy) = inputs[i2];
          output.push(
            Line { xbeg: px, ybeg: py, xend: qx, yend: qy,
                   finite: true }
          ),
      }
    }
  }
}