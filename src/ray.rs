use nannou::{prelude::*, geom::point};

pub struct Ray{
    // pub angle: f32,
    // pub step: f64,
    pub pos: Vec2,
    pub done: bool,
    pub point: Vec2,
    
}

impl Ray{
    pub fn new() -> Self{
        Self { done: false, point: Vec2::ZERO, pos: Vec2::ZERO }
    }
    /// returns true if it hits something.
    /// to access the hit point do Ray.point
    pub fn trace(&mut self, angle: f32, step: f32, lines: &Vec<Line>) -> bool{
        self.point = self.pos;
        let dir = vec2(angle.cos() * step, angle.sin() * step);
        // move loop
        for _ in 0..(1000.0/step) as usize {
            // move the point in step size and angle
            self.point += dir;
            // check all lines
            for line in lines {
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
    /// written by chat GPT
    pub fn intersect(&self, point: Vec2) -> bool {
        let v1 = vec2 (
            self.point2.x - self.point1.x,
            self.point2.y - self.point1.y,
        );
        let v2 = vec2 (
            point.x - self.point1.x,
            point.y - self.point1.y,
        );

        let dot_product = v1.x * v2.x + v1.y * v2.y;
        let squared_length_v1 = v1.x * v1.x + v1.y * v1.y;

        // Calculate the t value along the line segment
        let t = dot_product / squared_length_v1;


        // Check if the intersection point is within the bounds of the line segment
        if t >= 0.0  && t <= 1.0 {
            // Calculate the point of intersection on the line
            let intersection_point = vec2(
                self.point1.x + t * v1.x,
                self.point1.y + t * v1.y,
            );

            // Calculate the distance between the intersection point and the input point
            let distance = (intersection_point.x - point.x).hypot(intersection_point.y - point.y);

            return distance <= self.thickness;
        }

        false
    }

    
    /// find the distance between this line and a point
    pub fn distance_to_point(&self, point: &Vec2) -> f32 {
        // Calculate the direction vector of the line
        let line_direction  = vec2 (
            self.point2.x - self.point1.x,
            self.point2.y - self.point1.y,
        );
        let point_to_target  = vec2 (
            point.x - self.point1.x,
            point.y - self.point1.y,
        );

        // Calculate the dot product of the line direction and point_to_target
        let dot_product = line_direction.x * point_to_target.x + line_direction.y * point_to_target.y;

        // Calculate the distance from the point to the line
        let distance = dot_product.abs() / line_direction.length();

        // Return the distance
        distance
    }
}