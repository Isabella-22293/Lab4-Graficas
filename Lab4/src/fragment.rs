use nalgebra_glm::{Vec2, Vec3};
use crate::color::Color;

pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub depth: f32,
    pub normal: Vec3,
    pub intensity: f32,
    pub vertex_position: Vec3,
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: Color, depth: f32, normal: Vec3, intensity: f32, vertex_position: Vec3) -> Self {
        Fragment {
            position: Vec2::new(x, y),
            color,
            depth,
            normal,
            intensity,
            vertex_position,
        }
    }

    // Método para aplicar iluminación basada en la intensidad
    pub fn apply_lighting(&mut self) {
        let illuminated_color = self.color * self.intensity;
        self.color = illuminated_color;
    }

    // Método para interpolar entre colores
    pub fn interpolate_color(&mut self, other: &Color, t: f32) {
        self.color = self.color.lerp(other, t);
    }

    // Método para clonar el fragmento
    pub fn clone_fragment(&self) -> Fragment {
        Fragment {
            position: self.position,
            color: self.color,
            depth: self.depth,
            normal: self.normal,
            intensity: self.intensity,
            vertex_position: self.vertex_position,
        }
    }
}
