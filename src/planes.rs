use std::{rc::Rc, str::FromStr};

use crate::points::{Point, PointRef};
pub struct Plane {
    pub point:Rc<PointRef>,
    pub altitude: i32,
    pub speed: u32,
    pub direction: f32,
}

impl Plane {
    pub fn new(p: Point, altitude: i32, speed: u32, dir: u32, index: usize) -> Self {
        let direction = (dir as f32).to_radians();
        Plane {
            point: Rc::new(PointRef { point: p, idx: index }),
            altitude,
            speed,
            direction,
        }
    }

    /// Move the plane based on its speed and direction
    /// /// # Arguments
    /// /// * `time` - The time in seconds for which the plane should move
    pub fn move_plane(&mut self, time: f32) {
        let distance = self.speed as f32 * time;
        let mut x_coeff = self.direction.cos();
        let mut y_coeff = self.direction.sin();
        if x_coeff < f32::EPSILON {x_coeff = 0.0}
        if y_coeff < f32::EPSILON{y_coeff = 0.0}
        Rc::get_mut(&mut self.point).unwrap().point.x += distance * x_coeff;
        Rc::get_mut(&mut self.point).unwrap().point.y += distance * y_coeff;
    }
    fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
    }
}
impl FromStr for Plane {
    type Err = String;
 
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = Plane::strip_trailing_newline(s).split(' ').collect();

        if parts.len() < 6 {
            return Err("Invalid plane format".to_string());
        }
        let idx = parts[0].parse().unwrap();
        let p = Point::from_str(&format!("{} {}", parts[1], parts[2])).map_err(|_| "Invalid point format".to_string())?;
        let altitude = parts[3].parse().map_err(|_| "Invalid altitude".to_string())?;
        let speed = parts[4].parse().map_err(|_| "Invalid speed".to_string())?;
        let direction = parts[5].parse().map_err(|_| "Invalid direction".to_string())?;

        Ok(Plane::new(p, altitude, speed, direction, idx))
    }
}
