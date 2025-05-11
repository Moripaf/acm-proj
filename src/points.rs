use std::rc::Rc;

pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point { Point { x: x, y: y } }
    pub fn distance(&self, other: &Point) -> f64 {
        let diff_x = (self.x - other.x) as f64;
        let diff_y = (self.y - other.y) as f64;
        (diff_x.powf(2.0) + diff_y.powf(2.0)).sqrt()
    }
}
impl Clone for Point {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}
pub struct PointRef {
    pub point: Point,
    pub idx: usize,
}
impl PointRef {
    pub fn new(p: Point, i: usize) -> Self {
        Self { point: p, idx: i }
    }
    pub fn extract_by_idxs(points: &Vec<Rc<PointRef>>, idxs: &Vec<usize>) -> Vec<Rc<PointRef>> {
        points
            .iter()
            .filter(|p| -> bool { idxs.contains(&p.idx) })
            .map(|p| -> Rc<PointRef> { p.clone() })
            .collect()
    }
    pub fn extract_not_in_idxs(points: &Vec<Rc<PointRef>>, idxs: &Vec<usize>) -> Vec<Rc<PointRef>> {
        points
            .iter()
            .filter(|p| -> bool { !idxs.contains(&p.idx) })
            .map(|p| -> Rc<PointRef> { p.clone() })
            .collect()
    }
}
pub struct PointSet {
    points: Vec<Rc<PointRef>>,
    pub px: Vec<Rc<PointRef>>,
    pub py: Vec<Rc<PointRef>>,
}
impl PointSet {
    pub fn from_points(point_vec: Vec<Point>) -> Self {
        let mut i: usize = 0;
        let references: Vec<Rc<PointRef>> = point_vec
            .into_iter()
            .map(|p| -> Rc<PointRef> {
                i = i + 1;
                Rc::new(PointRef::new(p, i - 1))
            })
            .collect();
        let clone_vec_rc = |p: &Rc<PointRef>| -> Rc<PointRef> { p.clone() };
        let mut xs: Vec<Rc<PointRef>> = references.iter().map(clone_vec_rc).collect();
        xs.sort_by_key(|p| -> i32 { p.point.x });
        let mut ys: Vec<Rc<PointRef>> = references.iter().map(clone_vec_rc).collect();
        ys.sort_by_key(|p| -> i32 { p.point.y });
        Self {
            points: references,
            px: xs,
            py: ys,
        }
    }
    pub fn get_middle_x(&self) -> i32 {
        let idx = self.px.len() / 2;
        let midpoint = &self.px[idx].point;
        midpoint.x
    }
    pub fn split(&self, x: i32) -> (PointSet, PointSet) {
        let idxs: Vec<usize> = self
            .px
            .iter()
            .filter_map(|p| -> Option<usize> {
                if p.point.x < x {
                    Some(p.idx)
                } else {
                    None
                }
            })
            .collect();
        let first_half = Self {
            points: PointRef::extract_by_idxs(&self.points, &idxs),
            px: self
                .px
                .iter()
                .filter_map(|p| -> Option<Rc<PointRef>> {
                    if p.point.x < x {
                        Some(p.clone())
                    } else {
                        None
                    }
                })
                .collect(),
            py: PointRef::extract_by_idxs(&self.py, &idxs),
        };
        let second_half = Self {
            points: PointRef::extract_not_in_idxs(&self.points, &idxs),
            px: self
                .px
                .iter()
                .filter_map(|p| -> Option<Rc<PointRef>> {
                    if p.point.x > x {
                        Some(p.clone())
                    } else {
                        None
                    }
                })
                .collect(),
            py: PointRef::extract_not_in_idxs(&self.py, &idxs),
        };
        (first_half, second_half)
    }
}
// pub trait PointRef {
//     // Default implementation for from_point
//     fn from_point(p: &Rc<Point>) -> Self
//     where
//         Self: Sized,
//     {
//         Self::from_point_impl(p)
//     }
//
//     // Default implementation for from_vec
//     fn from_vec(points: &Vec<Rc<Point>>) -> Vec<Self>
//     where
//         Self: Sized,
//     {
//         points.iter().map(|p| Self::from_point(p)).collect()
//     }
//
//     // Internal method to be implemented by the struct
//     fn from_point_impl(p: &Rc<Point>) -> Self;
// }
//
// impl PointRef for Pointx {
//     fn from_point_impl(p: &Rc<Point>) -> Self {
//         Pointx {
//             point: Rc::clone(p),
//         }
//     }
// }
// impl PointRef for Pointy {
//     fn from_point_impl(p: &Rc<Point>) -> Self {
//         Pointy {
//             point: Rc::clone(p),
//         }
//     }
// }
// impl Ord for Pointx {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.point.x.cmp(&other.point.x)
//     }
// }
// impl PartialOrd for Pointx {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.point.x.partial_cmp(&other.point.x)
//     }
// }
// impl PartialEq for Pointx {
//     fn eq(&self, other: &Self) -> bool {
//         self.point.x == other.point.x && self.point.y == other.point.y
//     }
// }
// impl Eq for Pointx {}
//
// impl Ord for Pointy {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.point.y.cmp(&other.point.y)
//     }
// }
// impl PartialOrd for Pointy {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.point.y.partial_cmp(&other.point.y)
//     }
// }
// impl PartialEq for Pointy {
//     fn eq(&self, other: &Self) -> bool {
//         self.point.y == other.point.y && self.point.y == other.point.y
//     }
// }
// impl Eq for Pointy {}
