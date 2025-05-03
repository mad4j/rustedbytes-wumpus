use macroquad::rand::gen_range;

use crate::map::Map;


pub struct Game<'a> {

    pub map: &'a Map,

    pub player_pos: usize,
    pub wumpus_pos: usize,
    pub pits: Vec<usize>,
    pub bats: Vec<usize>,
    pub message: String,
    pub game_over: bool,
    pub win: bool,
}

impl<'a> Game<'a> {
    pub fn new(map: &'a Map) -> Self {
        
        let mut s = Game {
            map: &map,
            player_pos: 0,
            wumpus_pos: 0,
            pits: vec![],
            bats: vec![],
            message: "Muovi con clic sinistro, spara con clic destro".to_string(),
            game_over: false,
            win: false,
        };

        s.init();
        s
    }

    pub fn init(&mut self) {
        let mut used: Vec<usize> = Vec::new();
        // Reinitialize the random number generator to ensure randomness
        macroquad::rand::srand(macroquad::miniquad::date::now() as u64);
        // Funzione ausiliaria per scegliere una stanza casuale non ancora usata
        let mut pick_random = || {
            let mut idx = gen_range(0, self.map.get_caves() as i32) as usize;
            while used.contains(&idx) {
                idx = gen_range(0, self.map.get_caves() as i32) as usize;
            }
            used.push(idx);
            idx
        };

        self.player_pos = pick_random();
        self.wumpus_pos = pick_random();
        let pit1 = pick_random();
        let pit2 = pick_random();
        self.pits = vec![pit1, pit2];
        let bat1 = pick_random();
        let bat2 = pick_random();
        self.bats = vec![bat1, bat2];
    }

    // Restituisce le stanze adiacenti a quella in cui si trova il giocatore.
    pub fn current_adjacent(&self) -> Vec<usize> {
        self.map.get_connections()[self.player_pos].to_vec()
    }

    // Controlla i pericoli nella stanza corrente e fornisce indizi se presenti nelle stanze adiacenti.
    fn check_hazards(&mut self) {
        if self.player_pos == self.wumpus_pos {
            self.message = "Oh no! Il Wumpus ti ha mangiato!".to_string();
            self.game_over = true;
        } else if self.pits.contains(&self.player_pos) {
            self.message = "Sei caduto in un pozzo senza fondo!".to_string();
            self.game_over = true;
        } else if self.bats.contains(&self.player_pos) {
            self.message = "Pipistrelli giganti ti hanno trasportato in un'altra caverna!".to_string();
            self.player_pos = gen_range(0, self.map.get_caves() as i32) as usize;
            self.message += " Sei stato trasportato!";
        } else {
            let adj = self.current_adjacent();
            let mut clues = Vec::new();
            if adj.contains(&self.wumpus_pos) {
                clues.push("senti un odore terribile");
            }
            if adj.iter().any(|&r| self.pits.contains(&r)) {
                clues.push("senti un vento freddo");
            }
            if adj.iter().any(|&r| self.bats.contains(&r)) {
                clues.push("senti un fruscio");
            }
            if !clues.is_empty() {
                self.message = clues.join(" e ");
            } else {
                self.message = "La caverna è silenziosa".to_string();
            }
            self.message += " - Sinistro: muovi, Destro: spara";
        }
    }

/*
    fn update(&mut self) {
        if self.game_over {
            // Premi 'R' per ricominciare.
            if is_key_pressed(KeyCode::R) {
                self.init();
            }
            return;
        }

        // Gestione del clic sinistro: movimento.
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let click_vec = Vec2::new(mx, my);
            let adjacent = self.current_adjacent();
            for &room in &adjacent {
                let room_pos = Vec2::new(ROOM_POSITIONS[room].0, ROOM_POSITIONS[room].1);
                if room_pos.distance(click_vec) <= ROOM_RADIUS {
                    self.player_pos = room;
                    self.check_hazards();
                    break;
                }
            }
        }

        // Gestione del clic destro: sparare una freccia.
        if is_mouse_button_pressed(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let click_vec = Vec2::new(mx, my);
            let adjacent = self.current_adjacent();
            for &room in &adjacent {
                let room_pos = Vec2::new(ROOM_POSITIONS[room].0, ROOM_POSITIONS[room].1);
                if room_pos.distance(click_vec) <= ROOM_RADIUS {
                    // Se la stanza scelta contiene il Wumpus il giocatore vince.
                    if room == self.wumpus_pos {
                        self.message = "Hai colpito il Wumpus! Hai vinto!".to_string();
                        self.game_over = true;
                        self.win = true;
                    } else {
                        self.message = "Hai mancato! Il Wumpus si è spostato...".to_string();
                        self.wumpus_pos = gen_range(0, CAVES as i32) as usize;
                        self.check_hazards();
                    }
                    break;
                }
            }
        }
    }
*/
}
