mod map_renderer;
mod map;

use macroquad::prelude::*;

use map_renderer::MapRenderer;
use map::Map;




/// Macroquad window configuration.
/// This function is called at startup to set the window parameters.
fn window_conf() -> Conf {
    Conf {
        // Window title
        window_title: "RustedBytes - Hunt the Wumpus".to_string(),
        // Desired window dimensions in pixels
        window_width: 800,
        window_height: 600,
        // Enable support for high pixel density displays (Retina, HiDPI).
        // Important for rendering correctly at the specified resolution.
        high_dpi: true,
        // Enable Multi-Sample Anti-Aliasing (MSAA) to smooth edges.
        // Common values are 2, 4, 8. Higher values improve quality
        // but require more GPU resources. 4 is a good compromise.
        sample_count: 4,
        // Additional options (e.g., fullscreen: false, window_resizable: true)
        // can be added here if needed.
        ..Default::default() // Use default values for unspecified options
    }
}




// Configurazione della finestra principale di macroquad usando la funzione window_conf
#[macroquad::main(window_conf)]
async fn main() {
    
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;
    let screen_center = (center_x, center_y);

    // create a new Map instance (use default connections)
    let map = Map::new();
    let map_renderer = MapRenderer::new(map);

    // --- Ciclo Principale dell'Applicazione ---
    loop {
        clear_background(WHITE);

        map_renderer.draw_map(screen_center);


        next_frame().await;
    }
}