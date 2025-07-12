
mod line;
mod framebuffers;
use raylib::prelude::*;
use crate::framebuffers::Framebuffer;
use crate::line::line;
// Algoritmo scanline para rellenar un polígono
fn scanline_fill(fb: &mut Framebuffer, poly: &[Vector2], color: Color) {
    let min_y = poly.iter().map(|v| v.y as i32).min().unwrap_or(0);
    let max_y = poly.iter().map(|v| v.y as i32).max().unwrap_or(0);
    for y in min_y..=max_y {
        let mut xs = vec![];
        for i in 0..poly.len() {
            let v1 = poly[i];
            let v2 = poly[(i + 1) % poly.len()];
            if (v1.y as i32 <= y && v2.y as i32 > y) || (v2.y as i32 <= y && v1.y as i32 > y) {
                let x = v1.x + (y as f32 - v1.y) * (v2.x - v1.x) / (v2.y - v1.y);
                xs.push(x as i32);
            }
        }
        xs.sort();
        for pair in xs.chunks(2) {
            if pair.len() == 2 {
                for x in pair[0]..=pair[1] {
                    fb.set_current_color(color);
                    fb.set_pixel(x as u32, y as u32);
                }
            }
        }
    }
}
fn main() {
    let mut fb = Framebuffer::new(600, 600, Color::WHITE);
    fb.set_background_color(Color::WHITE);
    fb.set_current_color(Color::RED);

    let poly = vec![
        Vector2::new(377.0, 249.0),
        Vector2::new(411.0, 197.0),
        Vector2::new(436.0, 249.0),
    ];


    // Rellenar el polígono con scanline
    scanline_fill(&mut fb, &poly, Color::RED);

    // Dibujar los lados del polígono
    for window in poly.windows(2) {
        let start = window[0];
        let end = window[1];
        line(&mut fb, start, end);
    }
    line(&mut fb, poly[poly.len() - 1], poly[0]); // cerrar el polígono

    fb.render_to_file("poligonotres.png");
    println!("Polígono dibujado y guardado como 'poligonotres.png'!");
}
