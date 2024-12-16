mod boid;
mod vec2_extra;

use boid::Boid;
use new_egui_macroquad::egui;
use new_egui_macroquad::macroquad::{self, prelude::*};

fn draw_debugger(
    debug_view: &mut bool,
    boid_count: &mut usize,
    alignment_mult: &mut f32,
    cohesion_mult: &mut f32,
    separation_mult: &mut f32,
) {
    new_egui_macroquad::ui(|egui_ctx| {
        egui::Window::new("Debugger").show(egui_ctx, |ui| {
            ui.checkbox(debug_view, "Debug view");
            ui.add(egui::Slider::new(boid_count, 0..=1000).text("Boid count"));
            ui.add(egui::Slider::new(alignment_mult, 0.0..=5.0).text("Alignment"));
            ui.add(egui::Slider::new(cohesion_mult, 0.0..=5.0).text("Cohesion"));
            ui.add(egui::Slider::new(separation_mult, 0.0..=5.0).text("Separation"));
        });
    });
}

#[macroquad::main("Boids")]
async fn main() {
    let mut boids: Vec<Boid> = vec![];
    let mut debug_view = false;
    let mut boid_count = 0;
    let mut alignment_mult = 1.;
    let mut cohesion_mult = 1.;
    let mut separation_mult = 1.;

    loop {
        clear_background(BLACK);

        while boids.len() < boid_count {
            boids.push(Boid::default());
        }

        while boids.len() > boid_count {
            boids.pop();
        }

        let c = boids.clone();
        for boid in &mut boids {
            boid.flock(&c, alignment_mult, cohesion_mult, separation_mult);
            boid.update();
            if debug_view {
                boid.draw_debug(&c);
            }
            boid.draw();
        }

        draw_debugger(
            &mut debug_view,
            &mut boid_count,
            &mut alignment_mult,
            &mut cohesion_mult,
            &mut separation_mult,
        );

        new_egui_macroquad::draw();

        next_frame().await
    }
}
