use std::{collections::HashMap, rc::Rc, str::FromStr};

use crate::points::{Point, PointRef};
pub struct Plane {
    pub point:Rc<PointRef>,
    pub altitude: u32,
    pub speed: u32,
    pub direction: u32,
}

impl Plane {
    pub fn new(p: Point, altitude: u32, speed: u32, direction: u32, index: usize) -> Self {
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
        // Rc::get_mut(&mut self.point).unwrap().point.x += distance * (self.direction as f32).cos();
        // Rc::get_mut(&mut self.point).unwrap().point.y += distance * (self.direction as f32).sin();
    }
}
impl FromStr for Plane {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

            print!("{:?}", parts);
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
