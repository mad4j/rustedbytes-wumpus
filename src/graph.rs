
use std::f32::consts::PI; // Per usare il valore di Pi greco


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


pub fn draw_map() {

}