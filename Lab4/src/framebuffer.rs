pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            zbuffer: vec![f32::INFINITY; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = f32::INFINITY;
        }
    }

    pub fn point(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;

            if self.zbuffer[index] > depth {
                self.buffer[index] = self.current_color;
                self.zbuffer[index] = depth;
            }
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    // Método para obtener el color actual
    pub fn get_current_color(&self) -> u32 {
        self.current_color
    }

    // Método para dibujar una línea usando el algoritmo de Bresenham
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, depth: f32) {
        let mut x0 = x0 as isize;
        let mut y0 = y0 as isize;
        let x1 = x1 as isize;
        let y1 = y1 as isize;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 }; // Cambié `mut` a inmutable
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        while x0 != x1 || y0 != y1 {
            self.point(x0 as usize, y0 as usize, depth);

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    // Método para renderizar en pantalla
    pub fn render_to_screen(&self) {
    // Aquí se debería implementar el código para enviar el buffer a una pantalla o archivo.
    for y in 0..self.height {
        for x in 0..self.width {
            let index = y * self.width + x;
            // Elimina la variable color si no la usas
            // let color = self.buffer[index];
            // Aquí se utilizaría una función para dibujar `self.buffer[index]` en (x, y).
        }
    }
}
}
