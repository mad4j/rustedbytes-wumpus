// Hunt the Wumpus implementato in Rust con pattern MVC e macroquad
// Struttura del progetto:
// - Model: gestisce la logica di gioco
// - View: gestisce la rappresentazione grafica
// - Controller: gestisce gli input dell'utente e aggiorna il model

use std::f32::consts::PI;

use macroquad::prelude::*;

mod game_state;
mod game_model;
mod game_controller; // Add this line to include the new module

use game_state::GameState;
use game_model::GameModel;
use game_controller::GameController; // Import GameController from the new module

// ----- COSTANTI -----
const NUM_ROOMS: usize = 20;
//const NUM_TUNNELS_PER_ROOM: usize = 3;
//const NUM_BATS: usize = 2;
//const NUM_PITS: usize = 2;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const ROOM_RADIUS: f32 = 12.0;

// Numero di cerchi concentrici in cui sono disposti i nodi
const NUM_RINGS: usize = 3;

// Raggi delle 3 circonferenze concentriche
pub const RING_RADII: [f32; NUM_RINGS] = [ 200.0, 140.0, 60.0 ];

// Spessore delle linee guida
pub const LINE_THICKNESS: f32 = 2.0;

// Distribuzione dei nodi su ciascuna circonferenza (totale 5 + 10 + 5 = 20)
pub const NODES_PER_RING: [usize; NUM_RINGS] = [5, 10, 5];

// Angolo di partenza per i nodi su ciascuna circonferenza (in radianti)
// Mettiamo 0.0 per tutti, ma potresti variarli per "ruotare" i set di nodi
pub const START_ANGLES: [f32; NUM_RINGS] = [-0.5*PI, -0.5*PI, 0.5*PI];

// ----- VIEW -----
struct GameView {
    font_size: f32,
    title_font_size: f32,

    splash_texture: Option<Texture2D>,
}

impl GameView {
    fn new() -> Self {
        GameView {
            font_size: 20.0,
            title_font_size: 40.0,
            splash_texture: None,
        }
    }

    async fn load_resources(&mut self) {
        self.splash_texture = load_texture("assets/splash.png").await.ok();
    }

    fn draw_splash(&self) {
        clear_background(BLACK);
        
        if let Some(splash_texture) = &self.splash_texture {
            draw_texture(
                splash_texture,
                SCREEN_WIDTH / 2.0 - splash_texture.width() / 2.0,
                SCREEN_HEIGHT / 2.0 - splash_texture.height() / 2.0,
                WHITE,
            );
        }

        
        // Titolo del gioco
        let title = "HUNT THE WUMPUS";
        let title_size = measure_text(title, None, self.title_font_size as u16, 1.0);
        draw_text(
            title,
            SCREEN_WIDTH/2.0 - title_size.width/2.0,
            SCREEN_HEIGHT/3.0,
            self.title_font_size,
            GOLD
        );
        
        // Sottotitolo
        let subtitle = "Un classico gioco di avventura";
        let subtitle_size = measure_text(subtitle, None, (self.font_size * 1.2) as u16, 1.0);
        draw_text(
            subtitle,
            SCREEN_WIDTH/2.0 - subtitle_size.width/2.0,
            SCREEN_HEIGHT/3.0 + 50.0,
            self.font_size * 1.2,
            WHITE
        );
        
        // Istruzioni
        let instructions = [
            "Esplora le stanze della caverna e caccia il temibile Wumpus.",
            "Attenzione alle fosse e ai pipistrelli giganti!",
            "Usa il mouse per muoverti tra le stanze.",
            "Premi SPAZIO per attivare la modalità di tiro con l'arco.",
            "",
            "Premi SPAZIO o clicca per iniziare"
        ];
        
        for (i, line) in instructions.iter().enumerate() {
            let line_size = measure_text(line, None, self.font_size as u16, 1.0);
            draw_text(
                line,
                SCREEN_WIDTH/2.0 - line_size.width/2.0,
                SCREEN_HEIGHT/2.0 + (i as f32 * 30.0),
                self.font_size,
                LIGHTGRAY
            );
        }
        
        // Animazione "Premi per iniziare"
        let blink_rate = ((get_time() * 2.0) as f32).sin() * 0.5 + 0.5;
        let press_start = "Premi SPAZIO o clicca per iniziare";
        let press_start_size = measure_text(press_start, None, self.font_size as u16, 1.0);
        draw_text(
            press_start,
            SCREEN_WIDTH/2.0 - press_start_size.width/2.0,
            SCREEN_HEIGHT - 100.0,
            self.font_size,
            Color::new(1.0, 1.0, 1.0, blink_rate)
        );
    }


    fn draw_game(&self, model: &GameModel) {
        clear_background(WHITE);
        
        // Disegna le connessioni tra le stanze (tunnel)
        
        let center_x = SCREEN_WIDTH / 2.0;
        let center_y = SCREEN_HEIGHT / 2.0;
        let center = (center_x, center_y);

        // --- Disegna le Circonferenze Concentriche Guida (Opzionale) ---
        for &radius in RING_RADII.iter() {
            draw_poly_lines(center.0, center.1, 200, radius, 0.0, LINE_THICKNESS, LIGHTGRAY);
        }

        for i in 0..NUM_ROOMS {
            let base = model.room_positions[i];
            let conn = model.tunnels[i][2];
            if i < conn {
                let other =  model.room_positions[conn];
                draw_line(base.x, base.y, other.x, other.y, LINE_THICKNESS, LIGHTGRAY);
            }
        }

        // Disegna le stanze
        for (i, pos) in model.room_positions.iter().enumerate() {
            let color = if i == model.player_position {
                GREEN
            } else if model.game_over && i == model.wumpus_position {
                RED
            } else {
                GRAY
            };
            
            draw_circle(pos.x, pos.y, ROOM_RADIUS, color);
            draw_text(&i.to_string(), pos.x - 10.0, pos.y + 8.0, self.font_size, BLUE);
        }
        
        // Disegna l'interfaccia utente
        let message_width = measure_text(&model.message, None, self.font_size as u16, 1.0).width;
        draw_text(&model.message, SCREEN_WIDTH/2.0 - message_width/2.0, SCREEN_HEIGHT - 50.0, self.font_size, BLACK);
        
        // Disegna lo stato della freccia
        let arrow_text = if model.has_arrow { "Freccia: [x]" } else { "Freccia: [ ]" };
        draw_text(arrow_text, 20.0, 30.0, self.font_size, BLUE);
        
        // Disegna i comandi disponibili
        draw_text("Usa il mouse per muoverti o tirare frecce", 20.0, SCREEN_HEIGHT - 20.0, self.font_size - 5.0, LIGHTGRAY);
        
        // Se il gioco è finito, mostra il messaggio di vittoria/sconfitta
        if model.game_over {
            let end_message = if model.win {
                "Hai vinto! Premi R per ricominciare."
            } else {
                "Game Over! Premi R per ricominciare."
            };
            
            let text_size = measure_text(end_message, None, (self.font_size * 1.5) as u16, 1.0);
            draw_rectangle(
                SCREEN_WIDTH/2.0 - text_size.width/2.0 - 20.0,
                SCREEN_HEIGHT/2.0 - 30.0,
                text_size.width + 40.0,
                60.0,
                Color::new(0.0, 0.0, 0.0, 0.8)
            );
            draw_text(
                end_message,
                SCREEN_WIDTH/2.0 - text_size.width/2.0,
                SCREEN_HEIGHT/2.0 + 10.0,
                self.font_size * 1.5,
                if model.win { GREEN } else { RED }
            );
        }
    }

    fn draw_game_over(&self, model: &GameModel, transition_timer: f32) {
        // Prima disegna il gioco per mostrare lo stato finale
        self.draw_game(model);
        
        // Poi sovrapponi il messaggio di game over con un effetto di fade-in
        let alpha = f32::min(transition_timer, 1.0);
        
        // Disegna il Wumpus nella sua posizione
        let wumpus_pos = model.room_positions[model.wumpus_position];
        draw_circle(wumpus_pos.x, wumpus_pos.y, ROOM_RADIUS, RED);
        
        // Rettangolo semi-trasparente per il messaggio
        draw_rectangle(
            0.0,
            SCREEN_HEIGHT/2.0 - 150.0,
            SCREEN_WIDTH,
            300.0,
            Color::new(0.0, 0.0, 0.0, 0.8 * alpha)
        );
        
        // Messaggio di vittoria o sconfitta
        let end_message = if model.win {
            "Hai vinto! Hai eliminato il Wumpus!"
        } else {
            "Game Over! Il Wumpus ti ha preso!"
        };
        
        let text_size = measure_text(end_message, None, (self.title_font_size * 0.8) as u16, 1.0);
        draw_text(
            end_message,
            SCREEN_WIDTH/2.0 - text_size.width/2.0,
            SCREEN_HEIGHT/2.0 - 50.0,
            self.title_font_size * 0.8,
            if model.win { GREEN } else { RED }
        );
        
        // Mostra statistiche di gioco
        let game_time = model.end_time - model.start_time;
        let stats_message = format!("Mosse: {} | Tempo: {:.1} secondi", 
                                   model.moves_count,
                                   game_time);
        
        let stats_size = measure_text(&stats_message, None, self.font_size as u16, 1.0);
        draw_text(
            &stats_message,
            SCREEN_WIDTH/2.0 - stats_size.width/2.0,
            SCREEN_HEIGHT/2.0,
            self.font_size,
            WHITE
        );
        
        // Istruzioni per ricominciare
        let restart_text = "Premi R per ricominciare o ESC per tornare al menu";
        let restart_size = measure_text(restart_text, None, self.font_size as u16, 1.0);
        draw_text(
            restart_text,
            SCREEN_WIDTH/2.0 - restart_size.width/2.0,
            SCREEN_HEIGHT/2.0 + 50.0,
            self.font_size,
            WHITE
        );
    }


    fn get_clicked_room(&self, model: &GameModel, mouse_pos: Vec2) -> Option<usize> {
        for (i, pos) in model.room_positions.iter().enumerate() {
            if (mouse_pos - *pos).length() <= ROOM_RADIUS {
                return Some(i);
            }
        }
        None
    }
}

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
        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) || self.splash_timer > 3.0 {
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
        window_width: 800,
        window_height: 600,
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