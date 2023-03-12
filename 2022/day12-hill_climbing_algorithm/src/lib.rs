use std::u8;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Coordinate {
    x: u8,
    y: u8,
}

impl Coordinate {
    pub fn from(x: u8, y: u8) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    height: u8,
    coordinate: Coordinate,
    max: Coordinate,
}

impl Position {
    /// Constructs a Position from its components.
    ///
    /// Parameters:
    /// - height: The elevation at this position. Assumed to be within 0..=25.
    /// - coordinate: The Coordinates of this position.
    /// - max: The South-Eastern-most Coordinate of the map. This is stored here, so the map does
    /// not have to be queried for its dimensions at every neighbour lookup. Anything south or east
    /// of this Coordinate will not be considered part of the map.
    pub fn from(height: u8, coordinate: Coordinate, max: Coordinate) -> Self {
        Self {
            height,
            coordinate,
            max,
        }
    }

    /// Returns this Position's coordinate component
    pub fn coordinate(&self) -> Coordinate {
        self.coordinate
    }

    /// Returns this Position's height component
    pub fn height(&self) -> u8 {
        self.height
    }
    /// Finds all direct neighbours of this Position in the 4 cardinal directions as a Vector of
    /// Coordinates. This will not return any Coordinates outside the grid. The order of
    /// Coordinates in the result is always 
    ///
    /// 1. Western neighbour,
    /// 2. Northern neighbour,
    /// 3. Southern neighbour,
    /// 4. Eastern neighbour,
    ///
    /// skipping any that would be outside the grid.
    ///
    /// # Examples
    /// ```
    /// use day12_hill_climbing_algorithm::*;
    /// let this_position = Position::from(0, Coordinate::from(2, 1), Coordinate::from(2, 1));
    ///
    /// assert_eq!(this_position.neighbours(), vec![
    ///         Coordinate::from(1, 1),
    ///         Coordinate::from(2, 0),
    ///     ]);
    /// ```
    pub fn neighbours(&self) -> Vec<Coordinate> {
        let mut out = Vec::new();
        if self.coordinate.x > 0 {
            out.push(Coordinate { x: self.coordinate.x - 1, y: self.coordinate.y });
        }
        if self.coordinate.y > 0 {
            out.push(Coordinate { x: self.coordinate.x, y: self.coordinate.y - 1 });
        }
        if self.coordinate.y < self.max.y {
            out.push(Coordinate { x: self.coordinate.x, y: self.coordinate.y + 1 });
        }
        if self.coordinate.x < self.max.x {
            out.push(Coordinate { x: self.coordinate.x + 1, y: self.coordinate.y });
        }
        out
    }

    /// Finds all direct neighbours of this Position, that can reach it directly, meaning their heigth is no
    /// more than one unit below the height of this cell.
    ///
    /// Parameters:
    /// - self: This Position
    /// - grid: A 2D-Array of the map, where each element denotes the height at the coordinate
    /// represented by its indices.
    ///
    /// Returns:
    /// A Vector of all Coordinates that can reach this Position directly. The order is always
    /// West, North, South, East, skipping any that would be outside the grid or can't reach this
    /// Position directly.
    ///
    /// # Examples
    /// ```
    /// use day12_hill_climbing_algorithm::*;
    /// let this_position = Position::from(2, Coordinate::from(2, 1), Coordinate::from(2, 2));
    /// let grid = vec![
    ///         vec![0, 4, 0],
    ///         vec![1, 3, 2],
    ///         vec![2, 3, 1],
    ///     ];
    ///
    /// assert_eq!(this_position.reverse_reachable_neighbours(&grid), vec![
    ///         Coordinate::from(1, 1), // The western neighbour can reach us because they are heigher than us.
    ///         Coordinate::from(2, 2), // The southern neighbour can reach us because they are only 1 below us.
    ///         // But the northern neighbour can't reach us because they are more than 1 below us.
    ///     ]);
    /// ```
    pub fn reverse_reachable_neighbours(&self, grid: &[Vec<u8>]) -> Vec<Coordinate> {
        self.neighbours()
            .iter()
            .filter(|neighbour| grid[neighbour.y as usize][neighbour.x as usize]>=self.height.saturating_sub(1))
            .copied()
            .collect()
    }
}

 /// For a given destination, returns all starting positions in the grid that can reach the
 /// destination as a 2D-vector where the first dimension represents the distance to the destination.
 /// So `result[0]` will be a vector that only contains the destination itself (0 steps removed from
 /// it), `result[1]` will contain all its direct neighbours that can reach it, and so on. Generally
 /// `result[n]` will contain any Position that needs exactly n steps to reach the destination on its
 /// shortest path.
 ///
 /// Parameters:
 /// - destination: The `Position` the network is centered on. The distances will be in relation to
 /// this.
 /// - grid: The complete map as a 2D-Array, where each element represents the height at the
 /// Position denoted by its indices.
 ///
 /// Returns:
 /// A 2D-Vector containing all `Position` that can reach the destination in any way. The index of
 /// its first dimension equals the distance from that Position to the destination, meaning all
 /// Positions in `result[n]` are exactly `n` steps away from the destination on their shortest
 /// path. The index of the second dimension has no inherent meaning.
 ///
 /// # Examples
 /// ```
 /// use day12_hill_climbing_algorithm::*;
 /// let max = Coordinate::from(2, 2);
 /// let this_position = Position::from(4, max, max);
 /// let grid = vec![
 ///         vec![0, 4, 0],
 ///         vec![1, 4, 3],
 ///         vec![2, 3, 4],
 ///     ];
 ///
 /// assert_eq!(get_network_to(this_position, &grid), vec![
 ///        vec![ this_position ],
 ///        vec![ 
 ///            Position::from(3, Coordinate::from(1, 2), max),
 ///            Position::from(3, Coordinate::from(2, 1), max), ],
 ///        vec![
 ///            Position::from(2, Coordinate::from(0, 2), max),
 ///            Position::from(4, Coordinate::from(1, 1), max), ],
 ///        vec![ 
 ///            Position::from(1, Coordinate::from(0, 1), max),
 ///            Position::from(4, Coordinate::from(1, 0), max), ],
 ///        vec![
 ///            Position::from(0, Coordinate::from(0, 0), max), ],
 ///     ]);
 /// ```
pub fn get_network_to(destination: Position, grid: &[Vec<u8>]) -> Vec<Vec<Position>> {
    let mut network = vec![vec![destination]];
    loop {
        let last_distance = &network[network.len()-1];
        let mut new_this_distance = Vec::new();
        last_distance.iter().for_each(|last_position| {
            last_position.reverse_reachable_neighbours(grid).iter().for_each(|neighbour| {
                let neighbour_position = Position {
                    coordinate: *neighbour,
                    height: grid[neighbour.y as usize][neighbour.x as usize],
                    max: destination.max,
                };
                if !network.iter().flatten().any(|position| position == &neighbour_position) && !new_this_distance.contains(&neighbour_position) {
                    new_this_distance.push(neighbour_position);
                }
            });
        });
        if new_this_distance.is_empty() {
            break;
        }
        network.push(new_this_distance);
    }
    network
}

/// Converts a String-encoded map into the grid representation and finds the starting point, ending
/// point and the last represented point of the grid.
///
/// Parameters:
/// - map: A str representing the grid. This is assumed to be written in a recangular fassion (so
/// all lines are of equal length and no positions are empty), where the line number indicates the
/// North-South component of a coordinate and the position inside the line (or its column)
/// indicates the East-West component. The first character is assumed to be the North-Western-most
/// point of the grid. Each character is assumed to represent:
///   - the elevation at this coordinate, indicated by a lowercase letter, whose position in the
///   English alphabet denotes the elevation (a=0, b=1, c=2, ... z=25), or 
///   - the starting position, indicated by an uppercase S, and assumed to be at elevation 0, or 
///   - the end position, indicated by an uppercase E, and assumed to be at elevation 25.
///
/// Returns:
/// - grid: A 2D-Vector of u8, containing the same elevation data as `map`, but in a numerical format.
/// The layout is the same as in map, so `grid[y][x]` will be the elevation indicated by the x'th
/// character in line y (both 0-indexed) of map.
/// - start: The Coordinate of the last character marked with an uppercase S in the map. It is
/// assumed to have elevation 0.
/// - end: The Coordinate of the last character marked with an uppercase E in the map. It is
/// assumed to have elevation 25.
/// - max: The Coordinate of the last character in the last line of map. This is used to
/// determine the extent of the map.
///
/// # Panics
///
/// This panics if map contains lines of different length.
///
/// # Examples
/// ```
/// use day12_hill_climbing_algorithm::*;
/// let map = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
/// let (grid, start, end, max) = parse(map);
/// assert_eq!(grid, vec![
///         vec![0, 0, 1, 16, 15, 14, 13, 12],
///         vec![0, 1, 2, 17, 24, 23, 23, 11],
///         vec![0, 2, 2, 18, 25, 25, 23, 10],
///         vec![0, 2, 2, 19, 20, 21, 22,  9],
///         vec![0, 1, 3,  4,  5,  6,  7,  8],
///     ]);
/// assert_eq!(start, Coordinate::from(0, 0));
/// assert_eq!(end, Coordinate::from(5, 2));
/// assert_eq!(max, Coordinate::from(7, 4));
/// ```
///
pub fn parse(map: &str) -> (Vec<Vec<u8>>, Coordinate, Coordinate, Coordinate) {
    let mut grid = Vec::new();
    let mut start = Coordinate { x: 0, y: 0, };
    let mut end = Coordinate { x: 0, y: 0, };

    for row in 0..map.lines().count() {
        let mut this_row = Vec::new();
        for charcode in map.lines().nth(row).unwrap().bytes() {
            match charcode {
                b'S' => { start = Coordinate { x: this_row.len() as u8, y: row as u8, }; this_row.push(0); },
                b'E' => { end = Coordinate { x: this_row.len() as u8, y: row as u8, }; this_row.push(25); },
                c => this_row.push(c - b'a'),
            }
        }
        grid.push(this_row);
    }
    let max = Coordinate {
        x: grid[0].len() as u8 - 1,
        y: grid.len() as u8 - 1,
    };

    grid.iter().enumerate().for_each(|(idx, row)| {
        if row.len() != max.x as usize + 1 {
            panic!("Tried to parse a non-rectangular map. Row {idx} has {} characters, but row 0 has {}.", row.len(), max.x + 1);
        }
    });

    (grid, start, end, max)
}
