use macroquad::prelude::*;
use std::f32::consts::PI; // Per usare il valore di Pi greco

/// Configurazione della finestra di Macroquad.
/// Questa funzione viene chiamata all'avvio per impostare i parametri della finestra.
fn window_conf() -> Conf {
    Conf {
        // Titolo della finestra
        window_title: "Cerchi Concentrici HD AA".to_string(),
        // Dimensioni desiderate della finestra in pixel
        window_width: 800,
        window_height: 600,
        // Abilita il supporto per display ad alta densità di pixel (Retina, HiDPI).
        // Importante per avere un rendering corretto alla risoluzione specificata.
        high_dpi: true,
        // Attiva il Multi-Sample Anti-Aliasing (MSAA) per smussare i bordi.
        // Valori comuni sono 2, 4, 8. Un valore più alto migliora la qualità
        // ma richiede più risorse GPU. 4 è un buon compromesso.
        sample_count: 4,
        // Altre opzioni (es. fullscreen: false, window_resizable: true)
        // possono essere aggiunte qui se necessario.
        ..Default::default() // Usa i valori predefiniti per le altre opzioni non specificate
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
    let ring_radii: [f32; NUM_RINGS] = [80.0, 130.0, 180.0];

    // Distribuzione dei nodi su ciascuna circonferenza (totale 5 + 15 + 5 = 20)
    let nodes_per_ring: [usize; NUM_RINGS] = [5, 15, 5];

    // Angolo di partenza per i nodi su ciascuna circonferenza (in radianti)
    // Mettiamo 0.0 per tutti, ma potresti variarli per "ruotare" i set di nodi
    let start_angles: [f32; NUM_RINGS] = [-0.5*PI, 0.5*PI, -0.5*PI];
    

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

        next_frame().await;
    }
}