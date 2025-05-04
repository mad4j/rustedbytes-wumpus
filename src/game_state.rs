#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Splash,  // Schermata iniziale
    Play,    // Gioco in corso
    Over,    // Fine partita (vittoria o sconfitta)
}
