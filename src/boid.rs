use crate::vec2_extra::Vec2Extra;
use new_egui_macroquad::macroquad::prelude::*;

#[derive(Copy, Clone)]
pub struct Boid {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub max_speed: f32,
    pub size: f32,
    pub color: Color,
}

impl Boid {
    /// Applies the three steering forces to the boid's acceleration: alignment, cohesion, and separation.
    pub fn flock(
        &mut self,
        boids: &[Boid],
        alignment_mult: f32,
        cohesion_mult: f32,
        separation_mult: f32,
    ) {
        let neighbours = self.neighbours(boids, 100.);

        let alignment = self.alignment(&neighbours) * alignment_mult;
        let cohesion = self.coherence(&neighbours) * cohesion_mult;
        let separation = self.separation(&neighbours) * separation_mult;

        self.acc += alignment;
        self.acc += cohesion;
        self.acc += separation;
    }

    /// Updates the boid's position, velocity, and acceleration.
    pub fn update(&mut self) {
        self.vel += self.acc * get_frame_time();
        self.vel.limit(self.max_speed);
        self.pos += self.vel * get_frame_time();
        self.pos.wrap_around();
        self.acc = Vec2::ZERO;
    }

    /// Draws the boid.
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.size, self.color);
    }

    /// Draws debug information for the boid.
    pub fn draw_debug(&self, boids: &[Boid]) {
        draw_circle_lines(self.pos.x, self.pos.y, 100., 1., DARKGRAY);

        for boid in &self.neighbours(boids, 100.) {
            draw_line(self.pos.x, self.pos.y, boid.pos.x, boid.pos.y, 1., GRAY);
        }
    }

    /// Returns the boids within a certain radius of the boid.
    fn neighbours(&self, boids: &[Boid], radius: f32) -> Vec<Boid> {
        boids
            .iter()
            .filter(|boid| {
                let d = self.pos.distance(boid.pos);
                d > 0. && d < radius
            })
            .copied()
            .collect()
    }

    /// Returns the steering force that aligns the boid's velocity with its neighbours'.
    fn alignment(&self, neighbours: &[Boid]) -> Vec2 {
        if neighbours.is_empty() {
            return Vec2::ZERO;
        }

        let vel_sum = neighbours
            .iter()
            .fold(Vec2::ZERO, |acc, boid| acc + boid.vel);
        let avg_vel = vel_sum / neighbours.len() as f32;
        let desired_vel = avg_vel.with_length(self.max_speed);
        desired_vel - self.vel
    }

    /// Returns the steering force that moves the boid towards the average position of its neighbours.
    fn coherence(&self, neighbours: &[Boid]) -> Vec2 {
        if neighbours.is_empty() {
            return Vec2::ZERO;
        }

        let pos_sum = neighbours
            .iter()
            .fold(Vec2::ZERO, |acc, boid| acc + boid.pos);
        let avg_pos = pos_sum / neighbours.len() as f32;
        let desired_vel = (avg_pos - self.pos).with_length(self.max_speed);
        desired_vel - self.vel
    }

    /// Returns the steering force that moves the boid away from its neighbours.
    /// Closer neighbours generate stronger repulsion.
    fn separation(&self, neighbours: &[Boid]) -> Vec2 {
        if neighbours.is_empty() {
            return Vec2::ZERO;
        }

        let repulsion_sum = neighbours.iter().fold(Vec2::ZERO, |acc, boid| {
            let d = self.pos.distance(boid.pos).max(0.01);
            acc + (self.pos - boid.pos) / d
        });
        let avg_repulsion = repulsion_sum / neighbours.len() as f32;
        let desired_vel = avg_repulsion.with_length(self.max_speed);
        desired_vel - self.vel
    }
}
