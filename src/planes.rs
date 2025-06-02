use std::str::FromStr;

use crate::points::Point;
pub struct Plane {
    pub p:Point,
    pub altitude: u32,
    pub speed: u32,
    pub direction: f32,
}

impl Plane {
    pub fn new(p: Point, altitude: u32, speed: u32, direction: f32) -> Self {
        Plane {
            p,
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
        self.p.x += distance * self.direction.cos();
        self.p.y += distance * self.direction.sin();
    }
}
impl FromStr for Plane {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 4 {
            return Err("Invalid plane format".to_string());
        }
        let p = Point::from_str(&format!("{} {}", parts[0], parts[1])).map_err(|_| "Invalid point format".to_string())?;
        if parts.len() != 5 {
            return Err("Invalid plane format".to_string());
        }
        let altitude = parts[2].parse().map_err(|_| "Invalid altitude".to_string())?;
        let speed = parts[3].parse().map_err(|_| "Invalid speed".to_string())?;
        let direction = parts[4].parse().map_err(|_| "Invalid direction".to_string())?;

        Ok(Plane::new(p, altitude, speed, direction))
    }
}