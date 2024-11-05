use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3}; // Se eliminó 'dot'
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;
    let w = transformed.w;
    let screen_position = uniforms.viewport_matrix * Vec4::new(transformed.x / w, transformed.y / w, transformed.z / w, 1.0);

    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());
    let transformed_normal = normal_matrix * vertex.normal;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal,
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    black_and_white(fragment, uniforms)
}

// Shader para el sol
pub fn sun_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color { // Prefijado con _
    let bright_yellow = Color::new(255, 204, 0);
    bright_yellow * fragment.intensity
}

// Shader para planetas rocosos
pub fn rocky_planet_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color { // Prefijado con _
    let rocky_color = Color::new(139, 69, 19);
    rocky_color * fragment.intensity
}

// Shader para el gigante gaseoso
pub fn gas_giant_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color { // Prefijado con _
    let mut rng = rand::thread_rng();
    let gas_color = if rng.gen_bool(0.5) {
        Color::new(30, 144, 255)
    } else {
        Color::new(255, 69, 0)
    };
    gas_color * fragment.intensity
}

// Shader para planetas gaseosos pequeños
pub fn small_gas_planet_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color { // Prefijado con _
    let mut rng = rand::thread_rng();
    let gas_color = if rng.gen_bool(0.5) {
        Color::new(255, 182, 193)
    } else {
        Color::new(255, 105, 180)
    };
    gas_color * fragment.intensity
}

// Shader en blanco y negro
pub fn black_and_white(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;
    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);
    let random_number = rng.gen_range(0..=100);
    let black_or_white = if random_number < 50 {
        Color::new(0, 0, 0)
    } else {
        Color::new(255, 255, 255)
    };
    black_or_white * fragment.intensity
}
