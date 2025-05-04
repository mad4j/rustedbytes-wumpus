use crate::{game_model::GameModel, game_view::GameView};
use macroquad::prelude::*; // Update the import to use the new module

pub struct GameController {
    pub shoot_mode: bool,
    pub selected_room: Option<usize>,
    pub view: GameView, // Reference to the view for reusing get_clicked_room
}

impl GameController {
    pub fn new() -> Self {
        GameController {
            shoot_mode: false,
            selected_room: None,
            view: GameView::new(),
        }
    }

    pub fn process_input_play(&mut self, model: &mut GameModel) {
        // Toggle mode (movement or shooting)
        if is_key_pressed(KeyCode::Space) {
            if model.has_arrow {
                self.shoot_mode = !self.shoot_mode;
                if self.shoot_mode {
                    model.message = String::from(
                        "Modalità tiro attivata. Clicca su una stanza adiacente per tirare la freccia.",
                    );
                } else {
                    model.message = String::from("Modalità movimento attivata.");
                    model.generate_warnings();
                }
            } else {
                model.message = String::from("Non hai più frecce!");
            }
        }

        // Handle mouse click
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
            if let Some(room) = self.view.get_clicked_room(model, mouse_pos) {
                if self.shoot_mode {
                    model.shoot_arrow(room);
                    self.shoot_mode = false;
                } else {
                    model.move_player(room);
                }
            }
        }

        // Highlight the room under the cursor
        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        self.selected_room = self.view.get_clicked_room(model, mouse_pos);
    }
}
