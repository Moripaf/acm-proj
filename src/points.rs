use std::{rc::Rc, str::FromStr};

pub struct Point {
    pub x: f32,
    pub y: f32,
}
impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            return Err("Invalid point format".to_string());
        }
        let x = parts[0].parse::<f32>().map_err(|_| "Invalid x coordinate".to_string())?;
        let y = parts[1].parse::<f32>().map_err(|_| "Invalid y coordinate".to_string())?;
        Ok(Point { x, y })
    }
    
}
impl ToString for Point {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
    
}
impl Point {
    pub fn new(x: f32, y: f32) -> Point { Point { x: x, y: y } }
    pub fn distance(&self, other: &Point) -> f32 {
        let diff_x = (self.x - other.x) as f64;
        let diff_y = (self.y - other.y) as f64;
        (diff_x.powf(2.0) + diff_y.powf(2.0)).sqrt() as f32
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
        return PointSet::from_point_refs(references);
        
    }
    pub fn from_point_refs(prf: Vec<Rc<PointRef>>) -> Self {
        let clone_vec_rc = |p: &Rc<PointRef>| -> Rc<PointRef> { p.clone() };
        let mut xs: Vec<Rc<PointRef>> = prf.iter().map(clone_vec_rc).collect();
        xs.sort_by(|a,b| a.point.x.total_cmp(&b.point.x));
        let mut ys: Vec<Rc<PointRef>> = prf.iter().map(clone_vec_rc).collect();
        ys.sort_by(|a,b| a.point.y.total_cmp(&b.point.y));
        Self {
            points: prf,
            px: xs,
            py: ys,
        }       
    }
    pub fn get_middle_x(&self) -> f32 {
        let idx = self.px.len() / 2;
        let midpoint = &self.px[idx].point;
        midpoint.x
    }
    pub fn split(&self, x: f32) -> (PointSet, PointSet) {
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