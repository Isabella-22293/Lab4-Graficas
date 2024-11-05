use nalgebra_glm::Vec3;
use crate::color::Color;
use crate::shaders::{sun_shader, rocky_planet_shader, gas_giant_shader, small_gas_planet_shader, black_and_white};
// Asegúrate de que el módulo uniforms esté disponible
use crate::uniforms::Uniforms; // Asegúrate de que este módulo exista
use crate::fragment::Fragment;

// Estructura que representa un planeta o un cuerpo celeste
pub struct Planet {
    pub position: Vec3,
    pub radius: f32,
    pub planet_type: String,
    pub orbit_radius: f32, // Add this for orbit calculation
    pub speed: f32, // Speed of the orbit
}

impl Planet {
    pub fn new(position: Vec3, radius: f32, planet_type: &str, orbit_radius: f32, speed: f32) -> Planet {
        Planet {
            position,
            radius,
            planet_type: planet_type.to_string(),
            orbit_radius,
            speed,
        }
    }

    // Método para simular la órbita del planeta alrededor del sol
    pub fn orbit(&mut self, angle: f32) {
        self.position.x = self.orbit_radius * (angle * self.speed).cos();
        self.position.z = self.orbit_radius * (angle * self.speed).sin();
    }

    // Método para obtener el shader correspondiente al tipo de planeta
    pub fn get_shader(&self) -> fn(&Fragment, &Uniforms) -> Color {
        match self.planet_type.as_str() {
            "Sun" => sun_shader,
            "Rocky" => rocky_planet_shader,
            "GasGiant" => gas_giant_shader,
            "SmallGas" => small_gas_planet_shader,
            _ => black_and_white,
        }
    }
}

// Función para crear el sistema solar con los planetas necesarios
pub fn create_solar_system() -> Vec<Planet> {
    vec![
        Planet::new(Vec3::new(0.0, 0.0, 0.0), 1.5, "Sun", 0.0, 0.0), // Sun
        Planet::new(Vec3::new(2.5, 0.0, 0.0), 0.4, "Rocky", 2.5, 1.0), // Rocky planet
        Planet::new(Vec3::new(3.5, 0.0, 0.0), 0.6, "Rocky", 3.5, 0.8), // Rocky planet
        Planet::new(Vec3::new(5.0, 0.0, 0.0), 0.5, "Rocky", 5.0, 0.5), // Rocky planet
        Planet::new(Vec3::new(8.0, 0.0, 0.0), 1.2, "GasGiant", 8.0, 0.3), // Gas Giant
        Planet::new(Vec3::new(10.0, 0.0, 0.0), 0.8, "SmallGas", 10.0, 0.4), // Small Gas Planet
        Planet::new(Vec3::new(12.0, 0.0, 0.0), 0.7, "SmallGas", 12.0, 0.6), // Small Gas Planet
    ]
}
