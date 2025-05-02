
use std::f32::consts::PI;

use macroquad::{color::{BLACK, GRAY, LIGHTGRAY, WHITE}, math::Vec2, shapes::{draw_circle, draw_line, draw_poly_lines}, text::draw_text};

use crate::map::Map;


// --- Costanti Configurabili ---
// const TOTAL_DOTS: usize = 20; // Non più usata direttamente per il calcolo principale
pub const NUM_RINGS: usize = 3;
pub const DOT_RADIUS: f32 = 12.0;     // Raggio dei cerchi piccoli (nodi)
pub const RING_THICKNESS: f32 = 2.0; // Spessore delle linee guida

// Raggi delle 3 circonferenze concentriche
pub const RING_RADII: [f32; NUM_RINGS] = [ 180.0, 130.0, 80.0 ];

// Distribuzione dei nodi su ciascuna circonferenza (totale 5 + 10 + 5 = 20)
pub const NODES_PER_RING: [usize; NUM_RINGS] = [5, 10, 5];

// Angolo di partenza per i nodi su ciascuna circonferenza (in radianti)
// Mettiamo 0.0 per tutti, ma potresti variarli per "ruotare" i set di nodi
pub const START_ANGLES: [f32; NUM_RINGS] = [-0.5*PI, -0.5*PI, 0.5*PI];

pub struct MapRenderer {
    map: Map,
    node_positions: Vec<(f32, f32)>,
}

impl MapRenderer {
    pub fn new(map: Map) -> Self {
        let node_positions = (0..NUM_RINGS)
            .flat_map(|i| calculate_node_positions((0.0, 0.0), RING_RADII[i], NODES_PER_RING[i], START_ANGLES[i]))
            .collect();
        MapRenderer { map, node_positions }
    }

    
    pub fn draw_map(&self, center: (f32, f32)) {

        // --- Disegna le Circonferenze Concentriche Guida (Opzionale) ---
        for &radius in RING_RADII.iter() {
            draw_poly_lines(center.0, center.1, 200, radius, 0.0, RING_THICKNESS, LIGHTGRAY);
        }

        

        for i in 0..self.map.get_caves() {
            let base = Vec2::new(center.0 + self.node_positions[i].0, center.1 + self.node_positions[i].1);
            let conn = self.map.get_connections()[i][2];
            if i < conn {
                let other = Vec2::new(center.0 + self.node_positions[conn].0, center.1 + self.node_positions[conn].1);
                draw_line(base.x, base.y, other.x, other.y, 2.0, GRAY);
            }
        }

        // Disegna un cerchio (nodo) per ogni posizione calcolata
        for &(node_x, node_y) in self.node_positions.iter() {
            draw_circle(center.0 + node_x, center.1 + node_y, DOT_RADIUS, BLACK); // Cambiato colore in Blu
        }

        for i in 0..self.map.get_caves() {
            // Visualizza il numero della stanza (utile per il debug)
            let number = format!("{}", i);
            draw_text(&number, center.0 + self.node_positions[i].0 - 5.0, center.1 + self.node_positions[i].1 + 5.0, 16.0, WHITE);
        }

    }
}

/// Calcola le posizioni (coordinate dei centri) dei nodi distribuiti
/// uniformemente lungo la circonferenza specificata.
///
/// # Argomenti
/// * `center` - Una tupla (x, y) che rappresenta il centro della circonferenza.
/// * `radius` - Il raggio della circonferenza.
/// * `num_nodes` - Quanti nodi distribuire sulla circonferenza.
/// * `start_angle_rad` - L'angolo (in radianti) a cui posizionare il primo nodo.
///                       0.0 corrisponde alla posizione "ore 3".
///
/// # Ritorna
/// Un vettore di tuple (x, y), dove ogni tupla è il centro di un nodo.
pub fn calculate_node_positions(
    center: (f32, f32),
    radius: f32,
    num_nodes: usize,
    start_angle_rad: f32,
) -> Vec<(f32, f32)> {
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

        positions.push((node_x, node_y)); // Aggiunge le coordinate al vettore risultato
    }

    positions // Restituisce il vettore di posizioni
}

