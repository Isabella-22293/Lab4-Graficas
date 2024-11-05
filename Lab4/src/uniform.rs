// Importaciones necesarias
use nalgebra_glm::Mat4; // Asegúrate de importar los tipos necesarios
use fastnoise_lite::FastNoiseLite; // Importa FastNoiseLite si lo usas

// Definición de la estructura Uniforms
pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: u32,
    pub noise: FastNoiseLite,
}
