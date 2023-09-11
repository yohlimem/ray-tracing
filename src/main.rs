use circle::Circle;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
mod ray;
mod line;
mod circle;
use line::Line;
use ray::Ray;
use ray::Shape;

// TODO: Create a way to make rays bounce!
struct Model {
    // window: Window,
    egui: Egui,
    shapes: Vec<Shape>,
    rays: Vec<Ray>,
    points: Vec<(Vec2,Vec2)>,
    num: f32,
}

const BOUNCES:u32 = 10;

fn main() {
    nannou::app(model).update(update).run();
    
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    // let lines = vec![
    //     Line::from(vec2(-100.0, -200.0), vec2(100.0, -150.0), 1.0),
    //     Line::from(vec2(-100.0, 100.0), vec2(100.0, 150.0), 1.0),
    //     Line::from(vec2(-100.0, 100.0), vec2(-100.0, -200.0), 1.0),
    //     Line::from(vec2(100.0, 150.0), vec2(100.0, -150.0), 1.0),
    // ];
    let mut shapes = vec![
        // Shape::Line(Line::from(vec2(-100.0, -200.0), vec2(100.0, -200.0), 1.0)),
        // Shape::Line(Line::from(vec2(-100.0, 100.0), vec2(100.0, 100.0), 1.0)),
        // Shape::Line(Line::from(vec2(-100.0, 100.0), vec2(-100.0, -200.0), 1.0)),
        // Shape::Line(Line::from(vec2(100.0, 200.0), vec2(100.0, -200.0), 1.0)),
        // Shape::Circle(Circle::from(vec2(150.0, 0.0), 100.0)),
    ];
    for x in -10..10{
        for y in -10..10 {
            shapes.push(Shape::Circle(Circle::from(vec2(x as f32 * 50.0 + 25.0, y as f32 * 50.0 + 25.0), 20.0)))

        }
    }
    let rays = vec![Ray::new()];
    Model {
        egui,
        shapes,
        rays,
        points: vec![],
        num: 0.0,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    {
        let egui = &mut model.egui;
        egui.set_elapsed_time(update.since_start);

        let ctx = egui.begin_frame();

        egui::Window::new("Rum window").show(&ctx, |ui| {
            ui.label("res");
            // ui.add(egui::Checkbox::new(, "mouse control?"));
        });
    }
    model.num += 0.01;
    let first_angle = ( app.mouse.y).atan2(app.mouse.x);

    ray_trace(app, model, first_angle);
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent){
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw_shapes(&draw, &model);
    draw.text(&(app.mouse.y - model.rays[0].pos.y).atan2(app.mouse.x - model.rays[0].pos.x).to_degrees().to_string()).x_y(0.0, 0.0).color(BLACK);
    // draw.text(&(model.rays[0].bounce_angle(&model.lines[0])).to_degrees().to_string()).xy(model.rays[0].point).color(BLACK);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn draw_shapes(draw: &Draw, model: &Model){
    // draw all lines
    for shape in &model.shapes{
        match shape {
            Shape::Line(line) => {
                draw.line().end(line.point1).start(line.point2).weight(line.thickness);
            },
            Shape::Circle(circle) => {
                draw.ellipse().radius(circle.radius).xy(circle.pos).stroke_weight(1.0).stroke_color(BLACK);

            }
        }    
    }

    for ray in 0..model.points.len() {
        draw.line().end(model.points[ray].1).start(model.points[ray].0);//.color(Rgba8::new(0,0,0,50));
    }

}

fn ray_trace(app: &App, model: &mut Model, angle: f32){
    model.points.clear();
    for ray in 0..model.rays.len(){
        let mut hits:Vec<Shape> = vec![];
        
        let pos = model.rays[ray].pos.clone();
        let hit = model.rays[ray].trace(angle, 0.1, &model.shapes, None);
        // let hit = model.rays[ray].trace(-31.0, 0.1, &model.lines, None);
        model.points.push((pos, hit.1));
        
        if hit.0.is_some() {
            hits.push(hit.0.unwrap().clone());
            for _ in 0..BOUNCES{
                model.rays[ray].pos = model.points.last().unwrap().1;
                let last_shape = hits.last().unwrap();
                // let new_angle = 0.0;
                let new_angle = Ray::bounce_angle(last_shape, model.points.last().unwrap().1, model.points.last().unwrap().0);
                // if new_angle.is_none(){ return; }
                let hit2 = model.rays[ray].trace(new_angle, 0.1, &model.shapes, Some(last_shape));
                model.points.push((model.points.last().unwrap().1, hit2.1));
                // println!("degree: {}, i: {}", (new_angle.to_degrees()).abs(), i);
                // println!("hist: {:?}, i: {}",hits, i);
                if hit2.0.is_none(){
                    break;
                }
                else{
                    hits.push(hit2.0.unwrap().clone());

                }
                
            }
        }
        model.rays[ray].pos = pos;
    }
}
