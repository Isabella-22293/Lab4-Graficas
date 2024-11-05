use nalgebra_glm::{Vec3, dot};
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::color::Color;
use crate::Uniforms;
use crate::shaders::{sun_shader, rocky_planet_shader, gas_giant_shader, small_gas_planet_shader};

// Actualizar la función triangle para aplicar el shader correcto
pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex, planet_type: &str, uniforms: &Uniforms) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let (a, b, c) = (v1.transformed_position, v2.transformed_position, v3.transformed_position);

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);

    let light_dir = Vec3::new(0.0, 0.0, 1.0);
    let triangle_area = edge_function(&a, &b, &c);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
            let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c, triangle_area);

            if w1 >= 0.0 && w1 <= 1.0 && w2 >= 0.0 && w2 <= 1.0 && w3 >= 0.0 && w3 <= 1.0 {
                let normal = (v1.transformed_normal * w1 + v2.transformed_normal * w2 + v3.transformed_normal * w3).normalize();
                let intensity = dot(&normal, &light_dir).max(0.0);
                let depth = a.z * w1 + b.z * w2 + c.z * w3;
                let vertex_position = v1.position * w1 + v2.position * w2 + v3.position * w3;

                let color = get_shader_for_planet(planet_type, intensity, uniforms);

                fragments.push(
                    Fragment::new(
                        x as f32,
                        y as f32,
                        color,
                        depth,
                        normal,
                        intensity,
                        vertex_position,
                    )
                );
            }
        }
    }

    fragments
}

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;

    (min_x, min_y, max_x, max_y)
}

fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3, area: f32) -> (f32, f32, f32) {
    let w1 = edge_function(b, c, p) / area;
    let w2 = edge_function(c, a, p) / area;
    let w3 = edge_function(a, b, p) / area;

    (w1, w2, w3)
}

fn edge_function(a: &Vec3, b: &Vec3, c: &Vec3) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}

// Seleccionar el shader adecuado para cada tipo de planeta
fn get_shader_for_planet(planet_type: &str, intensity: f32, uniforms: &Uniforms) -> Color {
    let fragment_default = Fragment::new(0.0, 0.0, Color::black(), 0.0, Vec3::new(0.0, 0.0, 0.0), 0.0, Vec3::new(0.0, 0.0, 0.0));

    match planet_type {
        "sun" => sun_shader(&fragment_default, uniforms) * intensity,
        "rocky_planet" => rocky_planet_shader(&fragment_default, uniforms) * intensity,
        "gas_giant" => gas_giant_shader(&fragment_default, uniforms) * intensity,
        "small_gas_planet" => small_gas_planet_shader(&fragment_default, uniforms) * intensity,
        _ => Color::new(100, 100, 100) * intensity // Color por defecto en caso de no coincidir
    }
}