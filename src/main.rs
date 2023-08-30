use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use nannou::math;
mod ray;
use ray::Line;
use ray::Ray;

struct Model {
    // window: Window,
    egui: Egui,
    lines: Vec<Line>,
    rays: Vec<Ray>,
}

fn main() {
    nannou::app(model).update(update).run();
    
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let lines = vec![Line::from(vec2(-100.0, -200.0), vec2(100.0, -150.0), 1.0)];
    let rays = vec![Ray::new()];
    Model {
        egui,
        lines,
        rays,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("Rum window").show(&ctx, |ui| {
        ui.label("res");
    });


    for ray in &mut model.rays {
        ray.trace((app.mouse.y - ray.pos.y).atan2(app.mouse.x - ray.pos.x), 0.1, &model.lines);
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent){
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw_shapes(&draw, &model.rays, &model.lines);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn draw_shapes(draw: &Draw, rays: &Vec<Ray>, lines: &Vec<Line>){
    // draw all lines
    for line in lines{
        draw.line().end(line.point1).start(line.point2).weight(line.thickness);
    }

    for ray in rays {
        println!("ray point: {}", ray.point);
        draw.line().end(ray.point).start(ray.pos);
    }

}
