//! This module defines a `Map` structure that represents a cave system with connections between caves.
//! It provides methods to initialize the map and retrieve information about the caves and their connections.

/// Number of caves in the map.
const CAVES: usize = 20;

/// Each cave is connected to three other caves, represented as an array of indices.
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

/// The `Map` struct represents a cave system with connections between caves.
pub struct Map {
    connections: Vec<[usize; 3]>,
}

/// Represents a map structure with connections between caves.
///
/// # Methods
/// 
/// - `new`: Creates a new `Map` instance initialized with default connections.
/// - `init_with`: Initializes a `Map` instance with the given connections.
/// - `get_caves`: Returns the number of caves in the map.
/// - `get_connections`: Returns a reference to the vector of connections.
///
/// # Fields
/// 
/// - `connections`: A vector of arrays, where each array represents a connection
///   between caves. Each array contains three `usize` values representing the connected caves.
impl Map {

    /// Creates a new `Map` instance initialized with default connections.
    pub fn new() -> Self {
        Map::init_with(CONNECTIONS.to_vec())
    }

    /// Initializes a `Map` instance with the given connections.
    pub fn init_with(connections: Vec<[usize; 3]>) -> Self {
        Map {
            connections: connections.to_vec(),
        }
    }

    /// Returns the number of caves in the map.
    pub fn get_caves(&self) -> usize {
        self.connections.len()
    }
    
    /// Returns a reference to the vector of connections.
    pub fn get_connections(&self) -> &Vec<[usize; 3]> {
        &self.connections
    }
}
