use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;
mod planet;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use triangle::triangle;
use shaders::{vertex_shader, fragment_shader, sun_shader, rocky_planet_shader, gas_giant_shader, small_gas_planet_shader, black_and_white};
use fastnoise_lite::{FastNoiseLite, NoiseType};
use planet::Planet;
use crate::fragment::Fragment;
use crate::color::Color;

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite,
}

// Función para cargar el modelo de la esfera
fn load_sphere_model() -> Vec<Vertex> {
    // Ajusta el nombre y ubicación del archivo de la esfera en tu proyecto
    let sphere_obj = Obj::load("path_to_sphere_file.obj").expect("Error loading sphere model");
    sphere_obj.vertices
}

fn create_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    // Matrices de rotación y transformación
    // ...
    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;
    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;
    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn select_shader(planet_type: &str) -> fn(&Fragment, &Uniforms) -> Color {
    match planet_type {
        "sun" => sun_shader,
        "rocky" => rocky_planet_shader,
        "gas_giant" => gas_giant_shader,
        "small_gas" => small_gas_planet_shader,
        _ => black_and_white,
    }
}

fn create_solar_system() -> Vec<Planet> {
    vec![
        Planet::new(Vec3::new(0.0, 0.0, 0.0), 1.5, "Sun", 0.0, 0.0),     // Sun
        Planet::new(Vec3::new(2.5, 0.0, 0.0), 0.4, "Rocky", 2.5, 1.0),  // Rocky planet
        Planet::new(Vec3::new(3.5, 0.0, 0.0), 0.6, "Rocky", 3.5, 1.0),  // Rocky planet
        Planet::new(Vec3::new(5.0, 0.0, 0.0), 0.5, "Rocky", 5.0, 1.0),  // Rocky planet
        Planet::new(Vec3::new(8.0, 0.0, 0.0), 1.2, "GasGiant", 8.0, 0.5), // Gas Giant
        Planet::new(Vec3::new(10.0, 0.0, 0.0), 0.8, "SmallGas", 10.0, 0.5), // Small Gas Planet
        Planet::new(Vec3::new(12.0, 0.0, 0.0), 0.7, "SmallGas", 12.0, 0.5), // Small Gas Planet
    ]
}

fn update_planet_orbits(planets: &mut Vec<Planet>, time: f32) {
    for planet in planets {
        planet.position.x = planet.orbit_radius * time.cos();
        planet.position.z = planet.orbit_radius * time.sin();
    }
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], planet_type: &str) {
    let shader = select_shader(planet_type);

    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], planet_type, uniforms));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = shader(&fragment, &uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn main() {
    let mut planets = create_solar_system();
    let mut time: f32 = 0.0;
    let vertex_array = load_sphere_model();

    let mut window = Window::new("Sistema Solar Animado", 800, 600, WindowOptions::default()).unwrap();
    let mut framebuffer = Framebuffer::new(800, 600);

    while window.is_open() {
        time += 0.016;

        framebuffer.clear();
        update_planet_orbits(&mut planets, time);

        for planet in &planets {
            let model_matrix = create_model_matrix(planet.position, planet.radius, Vec3::new(0.0, 0.0, 0.0));
            let uniforms = Uniforms {
                model_matrix,
                view_matrix: create_view_matrix(Vec3::new(0.0, 2.0, 15.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
                projection_matrix: create_perspective_matrix(800.0, 600.0),
                viewport_matrix: create_viewport_matrix(800.0, 600.0),
                time: time as u32,
                noise: create_noise(),
            };

            render(&mut framebuffer, &uniforms, &vertex_array, planet.planet_type.as_str());
        }

        window.update_with_buffer(&framebuffer.buffer, 800, 600).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }
}
