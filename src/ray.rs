use nannou::{prelude::*, geom::point};

pub struct Ray<'a>{
    // pub angle: f32,
    // pub step: f64,
    pub done: bool,
    pub all_lines: &'a Vec<Line>,
    point: Vec2,
    
}

impl<'a> Ray<'a>{
    pub fn from(lines: Vec<Line>) -> Self{
        Self { done: false, point: Vec2::ZERO, all_lines: &lines }
    }
    /// returns true if it hits something.
    /// to access the hit point do Ray.point
    pub fn trace(&mut self, angle: f32, step: f32) -> bool{
        let dir = vec2(angle.sin() * step, angle.cos() * step);
        // move loop
        for _ in 0..(100.0/step) as usize {
            // move the point in step size and angle
            self.point += dir;
            // check all lines
            for line in self.all_lines {
                // find if a line intersects with a point
                if line.intersect(self.point) {
                    return true;
                }
            }
        }
        // if you did not intersect with any line return false
        return false;
    }
}

pub struct Line{
    pub point1: Vec2,
    pub point2: Vec2,
    pub thickness: f32,
}

impl Line{
    pub fn from(point1: Vec2, point2: Vec2, thickness: f32) -> Self{
        Line {point1, point2, thickness}
    }
    /// point is the points you want to find the intersection with
    /// 
    /// thickness is the minimum distance an intersect will trigger
    pub fn intersect(&self, point: Vec2) -> bool{
        
        let line_m = (self.point1.y - self.point2.y) / (self.point1.x - self.point2.x);
        // y = mx + b
        // b = y - mx
        let line_b = self.point1.y - line_m*self.point1.x;

        let distance = (line_m*point.x + 1.0*point.y + line_b).abs() / (line_m*line_m + 1.0).sqrt();

        distance <= self.thickness
    }
    
    /// find the distance between this line and a point
    pub fn distance(&self, point: Vec2) -> f32{
        let line_m = (self.point1.y - self.point2.y) / (self.point1.x - self.point2.x);
        // y = mx + b
        // b = y - mx
        let line_b = self.point1.y - line_m*self.point1.x;

        let distance = (line_m*point.x + 1.0*point.y + line_b).abs() / (line_m*line_m + 1.0).sqrt();

        distance
    }
}