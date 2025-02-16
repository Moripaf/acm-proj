use std::rc::Rc;

use points::*;

use crate::points::Point;

pub mod points;
///preprocess the points and call the algorithm
pub fn find_closest_pair(points: Vec<Point>) -> (Point,Point,u32) {
    let points_ref : Vec<Rc<Point>>= points.into_iter().map(|p| -> Rc<Point> {Rc::new(p)}).collect();
    let mut px = Pointx::from_vec(&points_ref);
    let mut py = Pointy::from_vec(&points_ref);
    px.sort();
    py.sort();
    compute_cpp(px, py)
}
///main recursive function
fn compute_cpp(px: Vec<Pointx>,py: Vec<Pointy>) -> (Point,Point,u32) {

    (Point {x:0,y:0},Point {x:0,y:0},0)
}