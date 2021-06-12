use hierarchical_pathfinding::prelude::*;

fn main() {
    let width = 64;
    let height = 64;

    let map = Map::new(width, height);

    let start = (0, 0);
    let end = (63, 0);

    let pathfinding = PathCache::new(
        (width, height),                           // the size of the Grid
        |(x, y)| map.get_tile_cost(x, y),          // get the cost for walking over a Tile
        MooreNeighborhood::new(width, height), // the Neighborhood
        PathCacheConfig {
            ..Default::default()
        }, // config
    );

    let path = pathfinding.find_path(start, end, |(x, y)| map.get_tile_cost(x, y));
    println!("do we have a path? {}", path.is_some());
    if let Some(temp) = path {
        println!("path cost = {}", temp.cost())
    }
}

pub struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let tile_count = width * height;
        Map {
            tiles: vec![Tile { cost: 1 }; tile_count],
            width,
            height,
        }
    }

    fn get_tile_cost(&self, x: usize, y: usize) -> isize {
        if let Some(index) = self.get_tile_index(x, y) {
            return self.tiles[index].cost;
        }
        -1
    }

    fn get_tile_index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            // Index out of bounds
            return None;
        }

        Some(x + y * self.width)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    cost: isize,
}
