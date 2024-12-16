use new_egui_macroquad::macroquad::prelude::*;

pub trait Vec2Extra {
    fn limit(&mut self, max_length: f32);
    fn wrap_around(&mut self);
    fn random(min: f32, max: f32) -> Vec2;
    fn random_in_screen() -> Vec2;
    fn with_length(&self, length: f32) -> Vec2;
}

impl Vec2Extra for Vec2 {
	/// Limits the length of the vector to `max_length`.
    fn limit(&mut self, max_length: f32) {
        let current_length = self.length();
        if current_length > max_length {
            let scale_factor = max_length / current_length;
            self.x *= scale_factor;
            self.y *= scale_factor;
        }
    }

	/// Wraps the vector around the screen edges.
    fn wrap_around(&mut self) {
        if self.x > screen_width() {
            self.x = 0.;
        } else if self.x < 0. {
            self.x = screen_width();
        }

        if self.y > screen_height() {
            self.y = 0.;
        } else if self.y < 0. {
            self.y = screen_height();
        }
    }

    /// Returns a random vector with components in the range `[min, max]`.
    fn random(min: f32, max: f32) -> Vec2 {
        Vec2::new(rand::gen_range(min, max), rand::gen_range(min, max))
    }

    /// Returns a random vector within the screen bounds.
    fn random_in_screen() -> Vec2 {
        Vec2::new(rand::gen_range(0., screen_width()), rand::gen_range(0., screen_height()))
    }

    /// Returns the same vector with the specified length.
    fn with_length(&self, length: f32) -> Vec2 {
        self.try_normalize().unwrap_or(Vec2::ZERO) * length
    }
}
