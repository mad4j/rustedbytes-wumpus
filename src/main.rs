mod graph;

use std::f32::consts::PI;

use macroquad::prelude::*;


const CAVES: usize = 20;
const CONNECTIONS: [[usize; 3]; CAVES] = [
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
];



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
fn calculate_node_positions(
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

// Configurazione della finestra principale di macroquad usando la funzione window_conf
#[macroquad::main(window_conf)]
async fn main() {
    // --- Costanti Configurabili ---
    // const TOTAL_DOTS: usize = 20; // Non più usata direttamente per il calcolo principale
    const NUM_RINGS: usize = 3;
    const DOT_RADIUS: f32 = 12.0;     // Raggio dei cerchi piccoli (nodi)
    const RING_THICKNESS: f32 = 2.0; // Spessore delle linee guida

    // Raggi delle 3 circonferenze concentriche
    let ring_radii: [f32; NUM_RINGS] = [ 180.0, 130.0, 80.0 ];

    // Distribuzione dei nodi su ciascuna circonferenza (totale 5 + 10 + 5 = 20)
    let nodes_per_ring: [usize; NUM_RINGS] = [5, 10, 5];

    // Angolo di partenza per i nodi su ciascuna circonferenza (in radianti)
    // Mettiamo 0.0 per tutti, ma potresti variarli per "ruotare" i set di nodi
    let start_angles: [f32; NUM_RINGS] = [-0.5*PI, -0.5*PI, 0.5*PI];
    
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;
    let screen_center = (center_x, center_y);

    let mut nodes_positions = vec![ ];
    for i in 0..NUM_RINGS {
        nodes_positions.append(&mut calculate_node_positions(screen_center, ring_radii[i], nodes_per_ring[i], start_angles[i]));
    }

    // --- Ciclo Principale dell'Applicazione ---
    loop {
        clear_background(WHITE);

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        let screen_center = (center_x, center_y);

        // --- Disegna le Circonferenze Concentriche Guida (Opzionale) ---
        for &radius in ring_radii.iter() {
            //draw_circle_lines(center_x, center_y, radius, RING_THICKNESS, LIGHTGRAY);
            draw_poly_lines(center_x, center_y, 200, radius, 0.0, RING_THICKNESS, LIGHTGRAY);
        }

        // --- Calcola e Disegna i Nodi usando la funzione dedicata ---
        for i in 0..NUM_RINGS {
            let radius = ring_radii[i];
            let num_nodes = nodes_per_ring[i];
            let start_angle = start_angles[i];

            // Chiama la funzione per ottenere le posizioni dei nodi per questa circonferenza
            let node_positions =
                calculate_node_positions(screen_center, radius, num_nodes, start_angle);

            // Disegna un cerchio (nodo) per ogni posizione calcolata
            for &(node_x, node_y) in node_positions.iter() {
                draw_circle(node_x, node_y, DOT_RADIUS, BLUE); // Cambiato colore in Blu
            }
        }

        for i in 0..CAVES {
            let base = Vec2::new(nodes_positions[i].0, nodes_positions[i].1);
            draw_circle(base.x, base.y, DOT_RADIUS, BLUE);
        
            let conn = CONNECTIONS[i][2];
            if i < conn {
                let other = Vec2::new(nodes_positions[conn].0, nodes_positions[conn].1);
                draw_line(base.x, base.y, other.x, other.y, 2.0, GRAY);
            }
            
            

             // Visualizza il numero della stanza (utile per il debug)
             let number = format!("{}", i);
             draw_text(&number, nodes_positions[i].0 - 5.0, nodes_positions[i].1 + 5.0, 16.0, BLACK);
        }

        next_frame().await;
    }
}