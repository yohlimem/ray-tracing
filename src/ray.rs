use nannou::ease::circ;
use nannou::{prelude::*, geom::point};
use crate::line::{Line, self};
use crate::circle::Circle;

// TODO: fix the bugs with the new enum and add ciricles


#[derive(Clone, Copy)]
pub struct Ray{
    // pub angle: f32,
    // pub step: f64,
    pub pos: Vec2,
    pub done: bool,
    pub point: Vec2,
    
}
#[derive(Clone, Copy, Debug)]
pub enum Shape{
    Circle(Circle),
    Line(Line),
}

impl Ray{
    pub fn new() -> Self{
        Self { done: false, point: Vec2::ZERO, pos: Vec2::ZERO }
    }
    pub fn from(pos: Vec2) -> Self{
        Self { done: false, point: pos, pos }
    }
    /// returns what (0) and where (1) it hits.
    /// 
    // // dont forget to update the self.point position using update()
    pub fn trace<'a:'b, 'b>(&'b mut self, angle: f32, step: f32, shapes: &'a Vec<Shape>, last_shape:Option<&Shape>) -> (Option<Shape>, Vec2){
        self.point = self.pos;
        let dir = vec2(angle.cos() * step, angle.sin() * step);

        // move loop
        for _ in 0..(1000.0/step) as usize {
            // move the point in step size and angle
            self.point += dir;
            // check all lines
            for shape in shapes {

                match shape {
                    Shape::Line(line) => {
                        // find if a line intersects with a point
                            if line.intersect(self.point) {
                                if let Some(last_shape) = last_shape{
                                    if let Shape::Line(last_line) = last_shape{
                                        
                                        if line.compare(last_line) {
                                            break;
                                        }
                                        // return some line and the point
                                    }
                                    
                                }
                                return (Some(Shape::Line(*line)), self.point);
                            }
                            // else {
                            //     return (None, self.point);
                            // }

                    }
                    Shape::Circle(circle) => {
                        
                        if circle.intersect(self.point) {
                            if let Some(last_circle) = last_shape{
                                if let Shape::Circle(last_circle) = last_circle{
                                    if circle.compare(last_circle) {
                                        break;
                                    }
                                    // return some line and the point
                                    
                                }
                            }
                            return (Some(Shape::Circle(*circle)), self.point);
                        }
                    }
                }
            }
        }
        // if you did not intersect with any line no line and the point
        let point = self.point;
        return (None, point);
    }

    pub fn bounce_angle(shape: &Shape, point: Vec2, pos:Vec2) -> f32{
        // let line_vector = line.point1 - line.point2;
        match shape {
            Shape::Line(line) => {

                let ray_vector = point - pos;
                let ray_to_ground = ray_vector.angle_between(vec2(1.0, 0.0));
                
                
                // let line_equ = line.slope();
                // if line_equ.is_none(){
                    //     // println!("0.0");
                    //     return ray_to_ground
                    // }
                    // let normal_m = -1.0/line.slope().unwrap_or(f32::MAX);
                    // let normal_line = Line::equation_to_line(normal_m, line.intercept().unwrap_or(f32::MAX));
                    // let normal_vector = normal_line.to_vector();
                    
                    
                    // let normal_to_ground = normal_vector.angle_between(vec2(-1.0, 0.0));
                    let line_to_ground = line.to_vector().angle_between(vec2(1.0, 0.0));
                    // let normal_to_ray: f32 = normal_vector.angle_between(ray_vector);
                    // println!("ray: {}, normal: {}", ray_to_ground, normal_to_ground);
                    
                    if line_to_ground <= 0.0{
                        ray_to_ground - (line_to_ground)
                        
                    } else {
                        
                        ray_to_ground - (line_to_ground + PI/2.0)
                    }
            },
            Shape::Circle(circle) => {
                let normal = circle.normal(point);
                let ray_vector = point - pos;
                let normal_to_ray = ray_vector.angle_between(normal);
                let normal_to_x = normal.angle_between(vec2(1.0, 0.0));
                if normal_to_x < 0.0 {
                    normal_to_ray + abs(normal_to_x) + PI
                }
                else {
                    normal_to_ray - abs(normal_to_x) + PI
                }
                // (normal_to_ray + abs(normal_to_x))
            }
        }
        
        
    }
}



pub trait Shape_Util {
    fn compare(&self, other: &Self) -> bool;
    fn intersect(&self, point:Vec2) -> bool;
}