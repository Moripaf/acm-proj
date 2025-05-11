use points::*;
use std::rc::Rc;

use crate::points::Point;

pub mod points;
pub struct Result {
    pub p1: Point,
    pub p2: Point,
    pub distance: f64,
}
impl Result {
    pub fn new(p1: Point, p2: Point) -> Self {
        let distance = p1.distance(&p2);
        Self { p1, p2, distance }
    }
    pub fn from_single_point(p1: Point) -> Self {
        let p2 = Point {
            x: i32::MAX,
            y: i32::MAX,
        };
        Self::new(p1, p2)
    }
}
///preprocess the points and call the algorithm
pub fn find_closest_pair(ps: Vec<Point>) -> Result {
    let points = PointSet::from_points(ps);
    compute_cpp(&points)
}
///main recursive function
fn compute_cpp(points: &PointSet) -> Result {
    if points.px.len() == 1 { 
        return Result::from_single_point(points.px[0].point.clone());
    }
    if points.px.len() == 2 {
        return Result::new(points.px[0].point.clone(), points.px[1].point.clone());
    }
    let middle_idx = points.px.len() / 2;
    let (first_half, second_half) = points.split(points.px[middle_idx].point.x);
    let first_res = compute_cpp(&first_half);
    let second_res = compute_cpp(&second_half);
    let min_res = if first_res.distance < second_res.distance {
        first_res
    } else {
        second_res
    };
    merge(first_half, second_half, min_res)
}
fn merge(set1: PointSet, set2: PointSet, min_res: Result) -> Result {
    // construct the strip
    let mut res = min_res;
    let middle = (set1.px.last().unwrap().point.x + set2.px.first().unwrap().point.x) / 2;
    let (_, right) = set1.split(middle - res.distance.ceil() as i32);
    let (left, _) = set2.split(middle + res.distance.floor() as i32);
    let size = left.py.len() + right.py.len();
    let mut strip: Vec<Rc<PointRef>> = Vec::with_capacity(size);
    let mut i: usize = 0;
    let mut j: usize = 0;
    for k in 0..size {
        strip[k] = if left.py[i].point.y > right.py[j].point.y {
            i = i + 1;
            left.py[i - 1].clone()
        } else {
            j = j + 1;
            right.py[j - 1].clone()
        };
    }
    for k in 0..size {
        for i in k..k+7 {
            let t_dist = strip[k].point.distance(&strip[i].point);
            if t_dist < res.distance { res = Result {
                p1: strip[k].point.clone(),
                p2: strip[i].point.clone(),
              distance: t_dist,  
            };}
        }
    }
    res
}
