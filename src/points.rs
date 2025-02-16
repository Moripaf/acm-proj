use std::{cmp::Ordering, rc::Rc};

pub struct Point{
    pub x:i32,
    pub y:i32
}
pub struct Pointy{
    point: Rc<Point>
}
pub struct Pointx{
    point: Rc<Point>
}
pub trait PointRef {
    // Default implementation for from_point
    fn from_point(p: &Rc<Point>) -> Self
    where
        Self: Sized,
    {
        Self::from_point_impl(p)
    }

    // Default implementation for from_vec
    fn from_vec(points: &Vec<Rc<Point>>) -> Vec<Self>
    where
        Self: Sized,
    {
        points.iter().map(|p| Self::from_point(p)).collect()
    }

    // Internal method to be implemented by the struct
    fn from_point_impl(p: &Rc<Point>) -> Self;
}

impl PointRef for Pointx {
    fn from_point_impl(p: &Rc<Point>) -> Self {
        Pointx {
            point: Rc::clone(p),
        }
    }
}
impl PointRef for Pointy {
    fn from_point_impl(p: &Rc<Point>) -> Self {
        Pointy {
            point: Rc::clone(p),
        }
    }
}
impl Ord for Pointx {
    fn cmp(&self, other: &Self) -> Ordering{
        self.point.x.cmp(&other.point.x) 
    }
}
impl PartialOrd for Pointx {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.point.x.partial_cmp(&other.point.x) 
    }
}
impl PartialEq for Pointx {
    fn eq(&self, other: &Self) -> bool {
        self.point.x == other.point.x && self.point.y == other.point.y
    }
}
impl Eq for Pointx {}

impl Ord for Pointy {
    fn cmp(&self, other: &Self) -> Ordering{
        self.point.y.cmp(&other.point.y) 
    }
}
impl PartialOrd for Pointy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.point.y.partial_cmp(&other.point.y) 
    }
}
impl PartialEq for Pointy {
    fn eq(&self, other: &Self) -> bool {
        self.point.y == other.point.y && self.point.y == other.point.y
    }
}
impl Eq for Pointy {}
