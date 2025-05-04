// Hunt the Wumpus implementato in Rust con pattern MVC e macroquad
// Struttura del progetto:
// - Model: gestisce la logica di gioco
// - View: gestisce la rappresentazione grafica
// - Controller: gestisce gli input dell'utente e aggiorna il model

use std::f32::consts::PI;

use macroquad::prelude::*;

mod game_controller; // Add this line to include the new module
mod game_model;
mod game_state;
mod game_view; // Add this line to include the new module

use game_controller::GameController; // Import GameController from the new module
use game_model::GameModel;
use game_state::GameState;
use game_view::GameView; // Import GameView from the new module

// ----- COSTANTI -----
//const NUM_ROOMS: usize = 20;
//const NUM_TUNNELS_PER_ROOM: usize = 3;
//const NUM_BATS: usize = 2;
//const NUM_PITS: usize = 2;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
//const ROOM_RADIUS: f32 = 12.0;

// Numero di cerchi concentrici in cui sono disposti i nodi
const NUM_RINGS: usize = 3;

// Raggi delle 3 circonferenze concentriche
pub const RING_RADII: [f32; NUM_RINGS] = [200.0, 140.0, 60.0];

// Spessore delle linee guida
pub const LINE_THICKNESS: f32 = 2.0;

// Distribuzione dei nodi su ciascuna circonferenza (totale 5 + 10 + 5 = 20)
pub const NODES_PER_RING: [usize; NUM_RINGS] = [5, 10, 5];

// Angolo di partenza per i nodi su ciascuna circonferenza (in radianti)
// Mettiamo 0.0 per tutti, ma potresti variarli per "ruotare" i set di nodi
pub const START_ANGLES: [f32; NUM_RINGS] = [-0.5 * PI, -0.5 * PI, 0.5 * PI];

// ----- GAME STATE -----
struct Game {
    state: GameState,
    model: GameModel,
    view: GameView,
    controller: GameController,
    splash_timer: f32,
    transition_timer: f32,
}

impl Game {
    fn new() -> Self {
        Game {
            state: GameState::Splash,
            model: GameModel::new(),
            view: GameView::new(),
            controller: GameController::new(),
            splash_timer: 0.0,
            transition_timer: 0.0,
        }
    }

    async fn initialize(&mut self) {
        self.view.load_resources().await;
    }

    fn update(&mut self) {
        match self.state {
            GameState::Splash => self.update_splash(),
            GameState::Play => self.update_play(),
            GameState::Over => self.update_over(),
        }
    }

    fn update_splash(&mut self) {
        // Aggiorna il timer della schermata di splash
        self.splash_timer += get_frame_time();

        // Passa allo stato di gioco se viene premuto un tasto o passa abbastanza tempo
        if is_key_pressed(KeyCode::Space)
            || is_mouse_button_pressed(MouseButton::Left)
            || self.splash_timer > 3.0
        {
            self.state = GameState::Play;
        }
    }

    fn update_play(&mut self) {
        // Aggiorna il controller e il modello durante il gioco
        self.controller.process_input_play(&mut self.model);

        // Se il gioco è finito, passa allo stato "Over"
        if self.model.game_over {
            self.state = GameState::Over;
            self.transition_timer = 0.0;
        }
    }

    fn update_over(&mut self) {
        // Aggiorna il timer per la schermata di game over
        self.transition_timer += get_frame_time();

        // Permetti di ricominciare premendo R
        if is_key_pressed(KeyCode::R) {
            self.model.reset();
            self.state = GameState::Play;
        }

        // Permetti di tornare alla schermata iniziale premendo Esc
        if is_key_pressed(KeyCode::Escape) {
            self.model.reset();
            self.state = GameState::Splash;
            self.splash_timer = 0.0;
        }
    }

    fn draw(&self) {
        match self.state {
            GameState::Splash => self.view.draw_splash(),
            GameState::Play => self.view.draw_game(&self.model),
            GameState::Over => self.view.draw_game_over(&self.model, self.transition_timer),
        }
    }
}

/// Macroquad window configuration.
/// This function is called at startup to set the window parameters.
fn window_conf() -> Conf {
    Conf {
        // Window title
        window_title: "RustedBytes - Hunt the Wumpus".to_string(),
        // Desired window dimensions in pixels
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        // Enable support for high pixel density displays (Retina, HiDPI).
        // Important for rendering correctly at the specified resolution.
        high_dpi: false,
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

// ----- MAIN -----
#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    game.initialize().await;

    loop {
        // Aggiornamento e disegno gestiti attraverso i diversi stati
        game.update();
        game.draw();

        next_frame().await;
    }
}
