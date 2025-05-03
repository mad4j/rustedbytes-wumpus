// Hunt the Wumpus implementato in Rust con pattern MVC e macroquad
// Struttura del progetto:
// - Model: gestisce la logica di gioco
// - View: gestisce la rappresentazione grafica
// - Controller: gestisce gli input dell'utente e aggiorna il model

use macroquad::prelude::*;
use ::rand::{rng, seq::{IndexedRandom, SliceRandom}, Rng};
use std::{collections::HashSet, f32::consts::PI};

// ----- COSTANTI -----
const NUM_ROOMS: usize = 20;
const NUM_TUNNELS_PER_ROOM: usize = 3;
const NUM_BATS: usize = 2;
const NUM_PITS: usize = 2;



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


// ----- MODEL -----
struct GameModel {
    // Topologia della caverna (grafo)
    tunnels: Vec<Vec<usize>>,
    // Posizioni degli elementi del gioco
    player_position: usize,
    wumpus_position: usize,
    bat_positions: HashSet<usize>,  
    pit_positions: HashSet<usize>,
    // Stato del gioco
    game_over: bool,
    win: bool,
    has_arrow: bool,
    message: String,
    // Coordinate visive delle stanze
    room_positions: Vec<Vec2>,
}

impl GameModel {
    fn new() -> Self {
        let tunnels = GameModel::create_cave_topology();
        let room_positions = GameModel::calculate_room_positions();
        let mut model = GameModel {
            tunnels,
            player_position: 0,
            wumpus_position: 0,
            bat_positions: HashSet::new(),
            pit_positions: HashSet::new(),
            game_over: false,
            win: false,
            has_arrow: true,
            message: String::from("Benvenuto a Hunt the Wumpus! Usa le frecce per muoverti, spazio per tirare una freccia."),
            room_positions,
        };
        model.initialize_game();
        model
    }

    fn create_cave_topology() -> Vec<Vec<usize>> {
        // Implementazione del dodecaedro standard per Hunt the Wumpus
        // In alternativa si potrebbe generare un grafo casuale
        vec![
            vec![1, 4, 5],    // 0
            vec![0, 2, 7],    // 1
            vec![1, 3, 9],    // 2
            vec![2, 4, 11],   // 3
            vec![0, 3, 13],   // 4

            vec![6, 14, 0],   // 5
            vec![5, 7, 18],   // 6
            vec![6, 8, 1],    // 7
            vec![7, 9, 19],   // 8
            vec![8, 10, 2],   // 9
            vec![9, 11, 15],  // 10
            vec![10, 12, 3],  // 11
            vec![11, 13, 16], // 12
            vec![12, 14, 4],  // 13
            vec![5, 13, 17],  // 14

            vec![16, 19, 10],  // 15
            vec![15, 17, 12],  // 16
            vec![16, 18, 14],  // 17
            vec![17, 19, 6],   // 18
            vec![15, 18, 8],   // 19
        ]
    }

    fn calculate_node_positions_by_ring(
        center: (f32, f32),
        radius: f32,
        num_nodes: usize,
        start_angle_rad: f32,
    ) -> Vec<Vec2> {
        let mut positions = Vec::with_capacity(num_nodes); // Pre-alloca spazio
        if num_nodes == 0 {
            return positions; // Nessun nodo da calcolare
        }
    
        // Calcola l'incremento angolare tra nodi adiacenti per una distribuzione uniforme
        let angle_step = 2.0 * PI / num_nodes as f32;
    
        for i in 0..num_nodes {
            // Calcola l'angolo per il nodo corrente, partendo da start_angle_rad
            let current_angle = start_angle_rad + (i as f32) * angle_step;
    
            // Calcola le coordinate cartesiane (x, y) del nodo usando la trigonometria
            // x = centro_x + raggio * cos(angolo)
            // y = centro_y + raggio * sin(angolo)
            let node_x = center.0 + radius * current_angle.cos();
            let node_y = center.1 + radius * current_angle.sin();
    
            positions.push(Vec2::new(node_x, node_y)); // Aggiunge le coordinate al vettore risultato
        }
    
        positions // Restituisce il vettore di posizioni
    }


    fn calculate_room_positions() -> Vec<Vec2> {
        
        let center_x = SCREEN_WIDTH / 2.0;
        let center_y = SCREEN_HEIGHT / 2.0;

        let positions: Vec<Vec2> = (0..NUM_RINGS)
            .flat_map(|i| GameModel::calculate_node_positions_by_ring((center_x, center_y), RING_RADII[i], NODES_PER_RING[i], START_ANGLES[i]))
            .collect();

        positions
    }

    fn initialize_game(&mut self) {
        let mut rng = rng();
        let mut available_rooms: Vec<usize> = (0..NUM_ROOMS).collect();
        available_rooms.shuffle(&mut rng);

        // Posiziona il giocatore
        self.player_position = available_rooms.pop().unwrap();

        // Posiziona il Wumpus
        self.wumpus_position = available_rooms.pop().unwrap();

        // Posiziona i pipistrelli
        self.bat_positions.clear();
        for _ in 0..NUM_BATS {
            self.bat_positions.insert(available_rooms.pop().unwrap());
        }

        // Posiziona le fosse
        self.pit_positions.clear();
        for _ in 0..NUM_PITS {
            self.pit_positions.insert(available_rooms.pop().unwrap());
        }

        self.game_over = false;
        self.win = false;
        self.has_arrow = true;
        self.message = String::from("Benvenuto a Hunt the Wumpus! Usa le frecce per muoverti, spazio per tirare una freccia.");
    }

    fn move_player(&mut self, room: usize) -> bool {
        // Verifica se la stanza è collegata alla posizione attuale
        if !self.tunnels[self.player_position].contains(&room) {
            self.message = format!("Non puoi andare direttamente alla stanza {}!", room);
            return false;
        }

        self.player_position = room;
        
        // Controlla se il giocatore è nella stanza del Wumpus
        if self.player_position == self.wumpus_position {
            self.message = String::from("Sei stato mangiato dal Wumpus! Game Over!");
            self.game_over = true;
            return true;
        }
        
        // Controlla se il giocatore è in una stanza con una fossa
        if self.pit_positions.contains(&self.player_position) {
            self.message = String::from("Sei caduto in una fossa! Game Over!");
            self.game_over = true;
            return true;
        }
        
        // Controlla se il giocatore è in una stanza con un pipistrello
        if self.bat_positions.contains(&self.player_position) {
            let mut rng = rng();
            self.player_position = rng.random_range(0..NUM_ROOMS);
            self.message = format!("Sei stato trasportato da un pipistrello gigante alla stanza {}!", self.player_position);
            
            // Controlla nuovamente se la nuova posizione è pericolosa
            if self.player_position == self.wumpus_position {
                self.message = String::from("Sei stato trasportato nella stanza del Wumpus! Game Over!");
                self.game_over = true;
                return true;
            }
            
            if self.pit_positions.contains(&self.player_position) {
                self.message = String::from("Sei stato trasportato in una stanza con una fossa! Game Over!");
                self.game_over = true;
                return true;
            }
            
            // Potremmo essere trasportati in un'altra stanza con pipistrelli
            return self.move_player(self.player_position);
        }
        
        // Genera gli avvertimenti
        self.generate_warnings();
        
        return true;
    }

    fn generate_warnings(&mut self) {
        let mut warnings = Vec::new();
        
        // Controlla se il Wumpus è vicino
        if self.tunnels[self.player_position].contains(&self.wumpus_position) {
            warnings.push("Senti un fetore nauseabondo...");
        }
        
        // Controlla se ci sono fosse vicine
        for &tunnel in &self.tunnels[self.player_position] {
            if self.pit_positions.contains(&tunnel) {
                warnings.push("Senti una brezza leggera...");
                break;
            }
        }
        
        // Controlla se ci sono pipistrelli vicini
        for &tunnel in &self.tunnels[self.player_position] {
            if self.bat_positions.contains(&tunnel) {
                warnings.push("Senti uno squittio in lontananza...");
                break;
            }
        }
        
        if warnings.is_empty() {
            self.message = format!("Ti trovi nella stanza {}. Tutto sembra tranquillo.", self.player_position);
        } else {
            self.message = format!("Ti trovi nella stanza {}. {}", self.player_position, warnings.join(" "));
        }
    }

    fn shoot_arrow(&mut self, target_room: usize) {
        if !self.has_arrow {
            self.message = String::from("Non hai più frecce!");
            return;
        }
        
        // Verifica se la stanza è collegata alla posizione attuale
        if !self.tunnels[self.player_position].contains(&target_room) {
            self.message = format!("Non puoi tirare la freccia alla stanza {}!", target_room);
            return;
        }
        
        self.has_arrow = false;
        
        if target_room == self.wumpus_position {
            self.message = String::from("Hai colpito il Wumpus! Hai vinto!");
            self.game_over = true;
            self.win = true;
        } else {
            self.message = format!("Hai mancato! La freccia è andata nella stanza {}.", target_room);
            
            // Il Wumpus potrebbe spostarsi
            let mut rng = rng();
            if rng.random::<f32>() < 0.75 {  // 75% di probabilità che il Wumpus si sposti
                let wumpus_tunnels = &self.tunnels[self.wumpus_position];
                let new_wumpus_pos = *wumpus_tunnels.choose(&mut rng).unwrap();
                
                if new_wumpus_pos == self.player_position {
                    self.message = String::from("Il Wumpus si è svegliato e ti ha trovato! Game Over!");
                    self.game_over = true;
                } else {
                    self.wumpus_position = new_wumpus_pos;
                    self.message += " Il Wumpus si è svegliato e si è spostato!";
                }
            }
        }
    }

    fn reset(&mut self) {
        self.initialize_game();
    }
}

// ----- VIEW -----
struct GameView {
    font_size: f32,
}

impl GameView {
    fn new() -> Self {
        GameView {
            font_size: 20.0,
        }
    }

    async fn load_resources(&self) {
        // Qui potremmo caricare immagini o altri asset
    }

    fn draw(&self, model: &GameModel) {
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

    fn get_clicked_room(&self, model: &GameModel, mouse_pos: Vec2) -> Option<usize> {
        for (i, pos) in model.room_positions.iter().enumerate() {
            if (mouse_pos - *pos).length() <= ROOM_RADIUS {
                return Some(i);
            }
        }
        None
    }
}

// ----- CONTROLLER -----
struct GameController {
    shoot_mode: bool,
    selected_room: Option<usize>,
}

impl GameController {
    fn new() -> Self {
        GameController {
            shoot_mode: false,
            selected_room: None,
        }
    }

    fn process_input(&mut self, model: &mut GameModel) {
        // Se il gioco è finito, controlla se il giocatore vuole ricominciare
        if model.game_over {
            if is_key_pressed(KeyCode::R) {
                model.reset();
            }
            return;
        }
        
        // Cambia modalità (movimento o tiro)
        if is_key_pressed(KeyCode::Space) {
            if model.has_arrow {
                self.shoot_mode = !self.shoot_mode;
                if self.shoot_mode {
                    model.message = String::from("Modalità tiro attivata. Clicca su una stanza adiacente per tirare la freccia.");
                } else {
                    model.message = String::from("Modalità movimento attivata.");
                    model.generate_warnings();
                }
            } else {
                model.message = String::from("Non hai più frecce!");
            }
        }
        
        // Gestione del click del mouse
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
            if let Some(room) = GameView::new().get_clicked_room(model, mouse_pos) {
                if self.shoot_mode {
                    model.shoot_arrow(room);
                    self.shoot_mode = false;
                } else {
                    model.move_player(room);
                }
            }
        }
        
        // Evidenzia la stanza sotto il cursore
        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        self.selected_room = GameView::new().get_clicked_room(model, mouse_pos);
    }
}

// ----- GAME STATE -----
struct Game {
    model: GameModel,
    view: GameView,
    controller: GameController,
}

impl Game {
    fn new() -> Self {
        Game {
            model: GameModel::new(),
            view: GameView::new(),
            controller: GameController::new(),
        }
    }

    async fn initialize(&mut self) {
        self.view.load_resources().await;
    }

    fn update(&mut self) {
        self.controller.process_input(&mut self.model);
    }

    fn draw(&self) {
        self.view.draw(&self.model);
    }
}

// ----- MAIN -----
#[macroquad::main("Hunt the Wumpus")]
async fn main() {
    let mut game = Game::new();
    game.initialize().await;
    
    loop {
        game.update();
        game.draw();
        
        next_frame().await;
    }
}