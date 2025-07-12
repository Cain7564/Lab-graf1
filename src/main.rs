
mod line;
mod framebuffers;
use raylib::prelude::*;
use crate::framebuffers::Framebuffer;
use crate::line::line;

fn main() {
    let mut fb = Framebuffer::new(2000, 2000, Color::WHITE);
    fb.set_background_color(Color::WHITE);
    fb.set_current_color(Color::RED);

    let poly = vec![
        Vector2::new(682.0, 175.0),
        Vector2::new(708.0, 120.0),
        Vector2::new(735.0, 148.0),
        Vector2::new(739.0, 170.0),
    ];



    // Dibujar los lados del polígono en color morado
    let morado = Color::new(128, 0, 128, 255); // Purple
    fb.set_current_color(morado);
    for window in poly.windows(2) {
        let start = window[0];
        let end = window[1];
        line(&mut fb, start, end);
    }
    line(&mut fb, poly[poly.len() - 1], poly[0]); // cerrar el polígono

    fb.render_to_file("poligono5.png");
    println!("Polígono dibujado y guardado como 'poligono5.png'!");
}
