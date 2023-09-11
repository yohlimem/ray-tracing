use nannou::prelude::*;

use crate::ray::Shape_Util;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub point1: Vec2,
    pub point2: Vec2,
    pub thickness: f32,
}

impl Line {
    pub fn from(point1: Vec2, point2: Vec2, thickness: f32) -> Self {
        Line {
            point1,
            point2,
            thickness,
        }
    }
    pub fn new() -> Self {
        Line {
            point1: Vec2::ZERO,
            point2: Vec2::ZERO,
            thickness: 1.0,
        }
    }

    /// find the distance between this line and a point
    pub fn distance_to_point(&self, point: &Vec2) -> f32 {
        // Calculate the direction vector of the line
        let line_direction = vec2(self.point2.x - self.point1.x, self.point2.y - self.point1.y);
        let point_to_target = vec2(point.x - self.point1.x, point.y - self.point1.y);

        // Calculate the dot product of the line direction and point_to_target
        let dot_product =
            line_direction.x * point_to_target.x + line_direction.y * point_to_target.y;

        // Calculate the distance from the point to the line
        let distance = dot_product.abs() / line_direction.length();

        // Return the distance
        distance
    }
    pub fn slope(&self) -> Option<f32> {
        let dx = self.point2.x - self.point1.x;
        let dy = self.point2.y - self.point1.y;

        // Check if the line is vertical (infinite slope)
        if dx == 0.0 {
            None
        } else {
            Some(dy / dx)
        }
    }

    pub fn intercept(&self) -> Option<f32> {
        // Calculate the slope (if not vertical)
        if let Some(slope) = self.slope() {
            let intercept = self.point1.y - slope * self.point1.x;
            Some(intercept)
        } else {
            // Line is vertical, no intercept
            None
        }
    }
    pub fn equation_to_line(m: f32, b: f32) -> Self {
        let point1_x = 1.0;
        let point1_y = m * point1_x + b;
        let point2_x = 10.0;
        let point2_y = m * point2_x + b;
        Line {
            point1: vec2(point1_x, point1_y),
            point2: vec2(point2_x, point2_y),
            thickness: 1.0,
        }
    }
    pub fn to_vector(&self) -> Vec2 {
        vec2(self.point2.x - self.point1.x, self.point2.y - self.point1.y)
    }
}

impl Shape_Util for Line {
    /// compare two lines. does not consider thickness
    fn compare(&self, line: &Line) -> bool {
        return self.point1 == line.point1 && self.point2 == line.point2;
    }
    /// point: the point you want to find intersection with
    ///
    /// thickness: the minimum distance an intersect will trigger
    ///
    /// written by chat GPT
    fn intersect(&self, point: Vec2) -> bool {
        let v1 = vec2(self.point2.x - self.point1.x, self.point2.y - self.point1.y);
        let v2 = vec2(point.x - self.point1.x, point.y - self.point1.y);

        let dot_product = v1.x * v2.x + v1.y * v2.y;
        let squared_length_v1 = v1.x * v1.x + v1.y * v1.y;

        // Calculate the t value along the line segment
        let t = dot_product / squared_length_v1;

        // Check if the intersection point is within the bounds of the line segment
        if t >= 0.0 && t <= 1.0 {
            // Calculate the point of intersection on the line
            let intersection_point = vec2(self.point1.x + t * v1.x, self.point1.y + t * v1.y);

            // Calculate the distance between the intersection point and the input point
            let distance = (intersection_point.x - point.x).hypot(intersection_point.y - point.y);

            return distance <= self.thickness;
        }

        false
    }
}
