/*
Input: a vector of points (x, y)
output: a vector of lines (xbeg, ybeg, xend, yend)
*/
use std::cmp::Ordering;
use std::collections::HashSet;
use line;
use parabola;

// 
fn intersection(p1: parabola::Point, p2: parabola::Point, dirx:f64) 
    -> (option parabola::Point, option parabola::Point)  {

    let mut result: Parabola::Point;
    let mut result2: Parabola::Point;
    let mut tmp: Parabola::Point = p1;
    // get intersection "y" => put x in tmp's eqn
    // TODO: fix it
    if p2._y == dirx {
        result._x = p2._x;
        result._y = (tmp._y * tmp._y) + (tmp._x - result._x) * (tmp._x - result._x) - dirx * dirx) / (2 * tmp._y - 2 * dirx);
        return (Some result, None)
    } else if p1._y == dirx {
        result._x = p1._x;
        tmp = p2;
        result._y = (tmp._y * tmp._y) + (tmp._x - result._x) * (tmp._x - result._x) - dirx * dirx) / (2 * tmp._y - 2 * dirx);
        return (Some result, None)
    } else {    
        let z1:f64 = 2 * (p1._y - dirx);
        let z2:f64 = 2 * (p2._y - dirx);

        let a:f64 = 1/z1 - 1/z2;
        let b:f64 = -2 * (p1._x / z1 - p2._x / z2);
        let c:f64 = (p1._x * p1._x + p1._y * p1._y - dirx * dirx) / z1 - (p2._x * p2._x + p2._y * p2._y - dirx * dirx) / z2;
        let d:f64 = b * b - 4 * a * c;

        if d > 0 {
            result._x = (- b - sqrt(b * b - 4 * a * c)) / (2 * a);
            result2._x = (- b + sqrt(b * b + 4 * a * c)) / (2 * a);
            result._y = (tmp._y * tmp._y) + (tmp._x - result._x) * (tmp._x - result._x) - dirx * dirx) / (2 * tmp._y - 2 * dirx);
            result2._y = (tmp._y * tmp._y) + (tmp._x - result2._x) * (tmp._x - result2._x) - dirx * dirx) / (2 * tmp._y - 2 * dirx);
            return (Some result, Some result2)
        } else if d == 0 {
            result._x = (- b - sqrt(b * b - 4 * a * c)) / (2 * a);
            result._y = (tmp._y * tmp._y) + (tmp._x - result._x) * (tmp._x - result._x) - dirx * dirx) / (2 * tmp._y - 2 * dirx);
            return (Some result, None)
        } else {
            return (None, None)
        }
    }   
}

pub fn run(input:&vec<(f64, f64)>, output:&mut vec<Line>) {
    
    let mut voronoi_points:Vec<(Parabola::Point)> = Vec::new();
    let mut voronoi_vertices:Vec<Vec<usize> > = Vec::new();
    let mut visited_points:HashSet<(usize, usize, usize)> = HashSet::new();


}
