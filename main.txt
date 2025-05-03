mod map_renderer;
mod map;
mod game;

use game::Game;
use macroquad::prelude::*;

use map_renderer::MapRenderer;
use map::Map;

enum GameState {
    Splash,
    Play,
}


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
        window_resizable: false, // Disable window resizing
        // Additional options (e.g., fullscreen: false)
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
    let mut game = Game::new(&map);
    let map_renderer = MapRenderer::new(&map, &game);

    let mut game_state = GameState::Splash;


    // --- Ciclo Principale dell'Applicazione ---
    loop {
        
        match game_state {
            GameState::Splash => {
                clear_background(BLACK);

                let texture = load_texture("assets/splash.png").await.unwrap();
                draw_texture(
                    &texture,
                    screen_center.0 - texture.width() / 2.0,
                    screen_center.1 - texture.height() / 2.0,
                    WHITE,
                );

                draw_text(
                    "Press SPACE or click to start",
                    screen_center.0 - 150.0,
                    screen_center.1,
                    30.0,
                    WHITE,
                );

                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    game_state = GameState::Play;
                }
            }
            GameState::Play => {
                clear_background(WHITE);
                map_renderer.draw_map(screen_center);

                // Gestione del clic sinistro: movimento.
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();

                    if let Some(room) = map_renderer.get_node_at_position(mx, my, screen_center) {
                        game.player_pos = room;
                        //self.check_hazards();
                    }
                }

            }
        }

        // Draw the current frame
        next_frame().await;

    }
}