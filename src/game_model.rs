use ::rand::{
    Rng, rng,
    seq::{IndexedRandom, SliceRandom},
};
use macroquad::prelude::*;
use std::{collections::HashSet, f32::consts::PI};

// Constants
const NUM_ROOMS: usize = 20;
const NUM_TUNNELS_PER_ROOM: usize = 3;
const NUM_BATS: usize = 2;
const NUM_PITS: usize = 2;
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
//const ROOM_RADIUS: f32 = 12.0;
const NUM_RINGS: usize = 3;
pub const RING_RADII: [f32; NUM_RINGS] = [200.0, 140.0, 60.0];
//pub const LINE_THICKNESS: f32 = 2.0;
pub const NODES_PER_RING: [usize; NUM_RINGS] = [5, 10, 5];
pub const START_ANGLES: [f32; NUM_RINGS] = [-0.5 * PI, -0.5 * PI, 0.5 * PI];

pub struct GameModel {
    pub tunnels: [[usize; NUM_TUNNELS_PER_ROOM]; NUM_ROOMS],
    pub player_position: usize,
    pub wumpus_position: usize,
    bat_positions: HashSet<usize>,
    pit_positions: HashSet<usize>,
    pub game_over: bool,
    pub win: bool,
    pub has_arrow: bool,
    pub message: String,
    pub room_positions: Vec<Vec2>,
    pub moves_count: u32,
    pub start_time: f64,
    pub end_time: f64,
}

impl GameModel {
    pub fn new() -> Self {
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
            message: String::from(
                "Benvenuto a Hunt the Wumpus! Usa le frecce per muoverti, spazio per tirare una freccia.",
            ),
            room_positions,
            moves_count: 0,
            start_time: 0.0,
            end_time: 0.0,
        };
        model.initialize_game();
        model
    }

    fn create_cave_topology() -> [[usize; NUM_TUNNELS_PER_ROOM]; NUM_ROOMS] {
        [
            [1, 4, 5],    // 0
            [0, 2, 7],    // 1
            [1, 3, 9],    // 2
            [2, 4, 11],   // 3
            [0, 3, 13],   // 4
            [6, 14, 0],   // 5
            [5, 7, 18],   // 6
            [6, 8, 1],    // 7
            [7, 9, 19],   // 8
            [8, 10, 2],   // 9
            [9, 11, 15],  // 10
            [10, 12, 3],  // 11
            [11, 13, 16], // 12
            [12, 14, 4],  // 13
            [5, 13, 17],  // 14
            [16, 19, 10], // 15
            [15, 17, 12], // 16
            [16, 18, 14], // 17
            [17, 19, 6],  // 18
            [15, 18, 8],  // 19
        ]
    }

    fn calculate_node_positions_by_ring(
        center: (f32, f32),
        radius: f32,
        num_nodes: usize,
        start_angle_rad: f32,
    ) -> Vec<Vec2> {
        let mut positions = Vec::with_capacity(num_nodes);
        if num_nodes == 0 {
            return positions;
        }

        let angle_step = 2.0 * PI / num_nodes as f32;

        for i in 0..num_nodes {
            let current_angle = start_angle_rad + (i as f32) * angle_step;
            let node_x = center.0 + radius * current_angle.cos();
            let node_y = center.1 + radius * current_angle.sin();
            positions.push(Vec2::new(node_x, node_y));
        }

        positions
    }

    fn calculate_room_positions() -> Vec<Vec2> {
        let center_x = SCREEN_WIDTH / 2.0;
        let center_y = SCREEN_HEIGHT / 2.0;

        (0..NUM_RINGS)
            .flat_map(|i| {
                GameModel::calculate_node_positions_by_ring(
                    (center_x, center_y),
                    RING_RADII[i],
                    NODES_PER_RING[i],
                    START_ANGLES[i],
                )
            })
            .collect()
    }

    fn initialize_game(&mut self) {
        let mut rng = rng();
        let mut available_rooms: Vec<usize> = (0..NUM_ROOMS).collect();
        available_rooms.shuffle(&mut rng);

        self.player_position = available_rooms.pop().unwrap();
        self.wumpus_position = available_rooms.pop().unwrap();

        self.bat_positions.clear();
        for _ in 0..NUM_BATS {
            self.bat_positions.insert(available_rooms.pop().unwrap());
        }

        self.pit_positions.clear();
        for _ in 0..NUM_PITS {
            self.pit_positions.insert(available_rooms.pop().unwrap());
        }

        self.game_over = false;
        self.win = false;
        self.has_arrow = true;
        self.message = String::from(
            "Benvenuto a Hunt the Wumpus! Usa le frecce per muoverti, spazio per tirare una freccia.",
        );
        self.moves_count = 0;
        self.start_time = get_time();
        self.end_time = 0.0;
    }

    pub fn move_player(&mut self, room: usize) -> bool {
        if !self.tunnels[self.player_position].contains(&room) {
            self.message = format!("Non puoi andare direttamente alla stanza {}!", room);
            return false;
        }

        self.player_position = room;

        if self.player_position == self.wumpus_position {
            self.message = String::from("Sei stato mangiato dal Wumpus! Game Over!");
            self.game_over = true;
            return true;
        }

        if self.pit_positions.contains(&self.player_position) {
            self.message = String::from("Sei caduto in una fossa! Game Over!");
            self.game_over = true;
            return true;
        }

        if self.bat_positions.contains(&self.player_position) {
            let mut rng = rng();
            self.player_position = rng.random_range(0..NUM_ROOMS);
            self.message = format!(
                "Sei stato trasportato da un pipistrello gigante alla stanza {}!",
                self.player_position
            );

            if self.player_position == self.wumpus_position {
                self.message =
                    String::from("Sei stato trasportato nella stanza del Wumpus! Game Over!");
                self.game_over = true;
                return true;
            }

            if self.pit_positions.contains(&self.player_position) {
                self.message =
                    String::from("Sei stato trasportato in una stanza con una fossa! Game Over!");
                self.game_over = true;
                return true;
            }

            return self.move_player(self.player_position);
        }

        self.generate_warnings();

        true
    }

    pub fn generate_warnings(&mut self) {
        let mut warnings = Vec::new();

        if self.tunnels[self.player_position].contains(&self.wumpus_position) {
            warnings.push("Senti un fetore nauseabondo...");
        }

        for &tunnel in &self.tunnels[self.player_position] {
            if self.pit_positions.contains(&tunnel) {
                warnings.push("Senti una brezza leggera...");
                break;
            }
        }

        for &tunnel in &self.tunnels[self.player_position] {
            if self.bat_positions.contains(&tunnel) {
                warnings.push("Senti uno squittio in lontananza...");
                break;
            }
        }

        if warnings.is_empty() {
            self.message = format!(
                "Ti trovi nella stanza {}. Tutto sembra tranquillo.",
                self.player_position
            );
        } else {
            self.message = format!(
                "Ti trovi nella stanza {}. {}",
                self.player_position,
                warnings.join(" ")
            );
        }
    }

    pub fn shoot_arrow(&mut self, target_room: usize) {
        if !self.has_arrow {
            self.message = String::from("Non hai più frecce!");
            return;
        }

        if !self.tunnels[self.player_position].contains(&target_room) {
            self.message = format!("Non puoi tirare la freccia alla stanza {}!", target_room);
            return;
        }

        self.has_arrow = false;

        if target_room == self.wumpus_position {
            self.message = String::from("Hai colpito il Wumpus! Hai vinto!");
            self.game_over = true;
            self.win = true;
            self.end_time = get_time();
        } else {
            self.message = format!(
                "Hai mancato! La freccia è andata nella stanza {}.",
                target_room
            );

            let mut rng = rng();
            if rng.random::<f32>() < 0.75 {
                let wumpus_tunnels = &self.tunnels[self.wumpus_position];
                let new_wumpus_pos = *wumpus_tunnels.choose(&mut rng).unwrap();

                if new_wumpus_pos == self.player_position {
                    self.message =
                        String::from("Il Wumpus si è svegliato e ti ha trovato! Game Over!");
                    self.game_over = true;
                    self.end_time = get_time();
                } else {
                    self.wumpus_position = new_wumpus_pos;
                    self.message += " Il Wumpus si è svegliato e si è spostato!";
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.initialize_game();
    }
}
