use crate::planes::{Plane};
use crate::points::{Point, PointRef, PointSet};
use core::f32;
use std::rc::Rc;
use std::collections::HashMap;

pub struct State {
    pub miss_distance: f32,
    pub miss_altitude: i32,
    pub current_time: u32,
    pub contents: HashMap<String, (u32,PointsResult)>,
    planes: Vec<Plane>,
}
impl State {
    pub fn new(distance: f32, altitude: i32, planes: Vec<Plane>) -> Self {
        Self {
            miss_distance: distance,
            miss_altitude: altitude,
            contents: HashMap::new(),
            planes,
            current_time: 0,
        }
    }
    pub fn try_push_points(&mut self, p1: Rc<PointRef>, p2: Rc<PointRef>) {

        let key = format!("{}-{}", p1.idx, p2.idx);
        let value = PointsResult::new(Point { x: p1.point.x, y: p1.point.y }, Point { x: p2.point.x, y: p2.point.y });
        if value.distance > self.miss_distance ||
             self.planes[p1.idx].altitude - self.planes[p2.idx].altitude > self.miss_altitude  {
            return;
        }
        // println!("({}, {}) - ({}, {})", value.p1.x, value.p1.y, value.p2.x, value.p2.y);
        if !self.contents.contains_key(&key) {
            self.contents.insert(key, (self.current_time,value));
        }
    }
    pub fn move_next_state(&mut self) {
        for p in &mut self.planes {
            p.move_plane(20.0);
        } 
    }
    pub fn as_point_refs(&self) -> Vec<Rc<PointRef>> {
        let mut points= Vec::with_capacity(20); 
        for p in &self.planes {
           points.push(p.point.clone()) 
        }
        points
    }
}
impl ToString for State {
    fn to_string(&self) -> String {
        let mut res = String::new();
        res.push_str(&format!("total {} near misses\n",self.contents.len()));
        for (key, value) in &self.contents {
            res.push_str(&format!("planes {} have a near miss at time {} and points: {} - {}\n", key, value.0, value.1.p1.to_string(), value.1.p2.to_string()));
        }
        res
    }
}
pub struct PointsResult {
    pub p1: Point,
    pub p2: Point,
    pub distance: f32,
}
impl PointsResult {
    pub fn new(p1: Point, p2: Point) -> Self {
        let distance = p1.distance(&p2);
        Self { p1, p2, distance }
    }
    pub fn from_no_point() -> Self {
        let p2 = Point {
            x: f32::MAX,
            y: f32::MAX,
        };
        let p1 = Point {
            x: f32::MIN,
            y: f32::MIN
        };
        Self::new(p1, p2)
    }
    pub fn from_single_point(p1: Point) -> Self {
        let p2 = Point {
            x: f32::MAX,
            y: f32::MAX,
        };
        Self::new(p1, p2)
    }
}
pub fn find_all_near_misses(state: &mut State, time:usize){
    for _i in 0..(time/20) {
        {
         let points = state.as_point_refs(); 
         let point_set = PointSet::from_point_refs(points);
         compute_cpp(&point_set, state);
        }
       state.move_next_state();
    } 
} 
fn compute_cpp(points: &PointSet, results: &mut State) -> PointsResult {
    if points.px.len() == 0 {
        return PointsResult::from_no_point();
    } 
    if points.px.len() == 1 { 
        return PointsResult::from_single_point(points.px[0].point.clone());
    }
    if points.px.len() == 2 {
        results.try_push_points(points.px[0].clone(), points.px[1].clone());
        return PointsResult::new(points.px[0].point.clone(), points.px[1].point.clone());
    }
    let middle_idx = points.px.len() / 2;
    let (first_half, second_half) = points.split(points.px[middle_idx].point.x);
    let first_res = compute_cpp(&first_half, results);
    let second_res = compute_cpp(&second_half, results);
    let min_res = if first_res.distance < second_res.distance {
        first_res
    } else {
        second_res
    };
    merge(first_half, second_half, min_res, results)
}
//theta(3n)
fn merge(set1: PointSet, set2: PointSet, min_res: PointsResult, results: &mut State) -> PointsResult {
    // construct the strip
    let mut res = min_res;
    let middle = (set1.px.last().unwrap().point.x + set2.px.first().unwrap().point.x) / 2.0;
    let (_, right) = set1.split(middle - res.distance);
    let (left, _) = set2.split(middle + res.distance);
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
        for i in k..k+8 {
            let t_dist = strip[k].point.distance(&strip[i].point);
            // println!("({}, {}) , ({}, {})", strip[k].point.x, strip[k].point.y, strip[i].point.x, strip[i].point.y);
            results.try_push_points(strip[k].clone(), strip[i].clone());
            if t_dist < res.distance { res = PointsResult {
                p1: strip[k].point.clone(),
                p2: strip[i].point.clone(),
              distance: t_dist,  
            };}
        }
    }
    res
}
