use raylib::prelude::*;

pub struct Framebuffer {
    width: u32,
    height: u32,
    color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    /// Rellena toda la imagen con el color de fondo
    pub fn clear(&mut self) {
        // Clear by creating a new image with the background color
        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
    }

    /// Dibuja un píxel en (x, y) con el color actual
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    /// Cambia el color de fondo y limpia la imagen con ese color
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
        self.clear();
    }

    /// Cambia el color con el que se dibujarán los próximos píxeles
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    /// Guarda la imagen en el archivo especificado
    pub fn render_to_file(&mut self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }
}
