use crate::game_model::GameModel;
use macroquad::prelude::*;

// Constants
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const ROOM_RADIUS: f32 = 12.0;
pub const RING_RADII: [f32; 3] = [200.0, 140.0, 60.0];
pub const LINE_THICKNESS: f32 = 2.0;

pub struct GameView {
    font_size: f32,
    title_font_size: f32,
    splash_texture: Option<Texture2D>,
}

impl GameView {
    pub fn new() -> Self {
        GameView {
            font_size: 20.0,
            title_font_size: 40.0,
            splash_texture: None,
        }
    }

    pub async fn load_resources(&mut self) {
        self.splash_texture = load_texture("assets/splash.png").await.ok();
    }

    pub fn draw_splash(&self) {
        clear_background(BLACK);

        if let Some(splash_texture) = &self.splash_texture {
            draw_texture(
                splash_texture,
                SCREEN_WIDTH / 2.0 - splash_texture.width() / 2.0,
                SCREEN_HEIGHT / 2.0 - splash_texture.height() / 2.0,
                WHITE,
            );
        }

        let title = "HUNT THE WUMPUS";
        let title_size = measure_text(title, None, self.title_font_size as u16, 1.0);
        draw_text(
            title,
            SCREEN_WIDTH / 2.0 - title_size.width / 2.0,
            SCREEN_HEIGHT / 3.0,
            self.title_font_size,
            GOLD,
        );

        let subtitle = "Un classico gioco di avventura";
        let subtitle_size = measure_text(subtitle, None, (self.font_size * 1.2) as u16, 1.0);
        draw_text(
            subtitle,
            SCREEN_WIDTH / 2.0 - subtitle_size.width / 2.0,
            SCREEN_HEIGHT / 3.0 + 50.0,
            self.font_size * 1.2,
            WHITE,
        );

        let instructions = [
            "Esplora le stanze della caverna e caccia il temibile Wumpus.",
            "Attenzione alle fosse e ai pipistrelli giganti!",
            "Usa il mouse per muoverti tra le stanze.",
            "Premi SPAZIO per attivare la modalità di tiro con l'arco.",
            "",
            "Premi SPAZIO o clicca per iniziare",
        ];

        for (i, line) in instructions.iter().enumerate() {
            let line_size = measure_text(line, None, self.font_size as u16, 1.0);
            draw_text(
                line,
                SCREEN_WIDTH / 2.0 - line_size.width / 2.0,
                SCREEN_HEIGHT / 2.0 + (i as f32 * 30.0),
                self.font_size,
                LIGHTGRAY,
            );
        }

        let blink_rate = ((get_time() * 2.0) as f32).sin() * 0.5 + 0.5;
        let press_start = "Premi SPAZIO o clicca per iniziare";
        let press_start_size = measure_text(press_start, None, self.font_size as u16, 1.0);
        draw_text(
            press_start,
            SCREEN_WIDTH / 2.0 - press_start_size.width / 2.0,
            SCREEN_HEIGHT - 100.0,
            self.font_size,
            Color::new(1.0, 1.0, 1.0, blink_rate),
        );
    }

    pub fn draw_game(&self, model: &GameModel) {
        clear_background(WHITE);

        let center_x = SCREEN_WIDTH / 2.0;
        let center_y = SCREEN_HEIGHT / 2.0;
        let center = (center_x, center_y);

        for &radius in RING_RADII.iter() {
            draw_poly_lines(
                center.0,
                center.1,
                200,
                radius,
                0.0,
                LINE_THICKNESS,
                LIGHTGRAY,
            );
        }

        for i in 0..model.tunnels.len() {
            let base = model.room_positions[i];
            let conn = model.tunnels[i][2];
            if i < conn {
                let other = model.room_positions[conn];
                draw_line(base.x, base.y, other.x, other.y, LINE_THICKNESS, LIGHTGRAY);
            }
        }

        for (i, pos) in model.room_positions.iter().enumerate() {
            let color = if i == model.player_position {
                GREEN
            } else if model.game_over && i == model.wumpus_position {
                RED
            } else {
                GRAY
            };

            draw_circle(pos.x, pos.y, ROOM_RADIUS, color);
            draw_text(
                &i.to_string(),
                pos.x - 10.0,
                pos.y + 8.0,
                self.font_size,
                BLUE,
            );
        }

        let message_width = measure_text(&model.message, None, self.font_size as u16, 1.0).width;
        draw_text(
            &model.message,
            SCREEN_WIDTH / 2.0 - message_width / 2.0,
            SCREEN_HEIGHT - 50.0,
            self.font_size,
            BLACK,
        );

        let arrow_text = if model.has_arrow {
            "Freccia: [x]"
        } else {
            "Freccia: [ ]"
        };
        draw_text(arrow_text, 20.0, 30.0, self.font_size, BLUE);

        draw_text(
            "Usa il mouse per muoverti o tirare frecce",
            20.0,
            SCREEN_HEIGHT - 20.0,
            self.font_size - 5.0,
            LIGHTGRAY,
        );

        if model.game_over {
            let end_message = if model.win {
                "Hai vinto! Premi R per ricominciare."
            } else {
                "Game Over! Premi R per ricominciare."
            };

            let text_size = measure_text(end_message, None, (self.font_size * 1.5) as u16, 1.0);
            draw_rectangle(
                SCREEN_WIDTH / 2.0 - text_size.width / 2.0 - 20.0,
                SCREEN_HEIGHT / 2.0 - 30.0,
                text_size.width + 40.0,
                60.0,
                Color::new(0.0, 0.0, 0.0, 0.8),
            );
            draw_text(
                end_message,
                SCREEN_WIDTH / 2.0 - text_size.width / 2.0,
                SCREEN_HEIGHT / 2.0 + 10.0,
                self.font_size * 1.5,
                if model.win { GREEN } else { RED },
            );
        }
    }

    pub fn draw_game_over(&self, model: &GameModel, transition_timer: f32) {
        self.draw_game(model);

        let alpha = f32::min(transition_timer, 1.0);

        let wumpus_pos = model.room_positions[model.wumpus_position];
        draw_circle(wumpus_pos.x, wumpus_pos.y, ROOM_RADIUS, RED);

        draw_rectangle(
            0.0,
            SCREEN_HEIGHT / 2.0 - 150.0,
            SCREEN_WIDTH,
            300.0,
            Color::new(0.0, 0.0, 0.0, 0.8 * alpha),
        );

        let end_message = if model.win {
            "Hai vinto! Hai eliminato il Wumpus!"
        } else {
            "Game Over! Il Wumpus ti ha preso!"
        };

        let text_size = measure_text(end_message, None, (self.title_font_size * 0.8) as u16, 1.0);
        draw_text(
            end_message,
            SCREEN_WIDTH / 2.0 - text_size.width / 2.0,
            SCREEN_HEIGHT / 2.0 - 50.0,
            self.title_font_size * 0.8,
            if model.win { GREEN } else { RED },
        );

        let game_time = model.end_time - model.start_time;
        let stats_message = format!(
            "Mosse: {} | Tempo: {:.1} secondi",
            model.moves_count, game_time
        );

        let stats_size = measure_text(&stats_message, None, self.font_size as u16, 1.0);
        draw_text(
            &stats_message,
            SCREEN_WIDTH / 2.0 - stats_size.width / 2.0,
            SCREEN_HEIGHT / 2.0,
            self.font_size,
            WHITE,
        );

        let restart_text = "Premi R per ricominciare o ESC per tornare al menu";
        let restart_size = measure_text(restart_text, None, self.font_size as u16, 1.0);
        draw_text(
            restart_text,
            SCREEN_WIDTH / 2.0 - restart_size.width / 2.0,
            SCREEN_HEIGHT / 2.0 + 50.0,
            self.font_size,
            WHITE,
        );
    }

    pub fn get_clicked_room(&self, model: &GameModel, mouse_pos: Vec2) -> Option<usize> {
        for (i, pos) in model.room_positions.iter().enumerate() {
            if (mouse_pos - *pos).length() <= ROOM_RADIUS {
                return Some(i);
            }
        }
        None
    }
}
