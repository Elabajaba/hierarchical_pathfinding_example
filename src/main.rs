use hierarchical_pathfinding::prelude::*;
use hashbrown::HashMap;

fn main() {
    // profiling::register_thread!("Main Thread");

    let width = 1024;
    let height = 1024;

    // profiling::scope!("Main Thread");
    let mut map = Map::new(width, height);

    let start = (0, 0);
    // let end = (31, 31);

    // let mut pathfinding = PathCache::new(
    //     (width, height),                       // the size of the Grid
    //     |(x, y)| map.get_tile_cost(x, y),      // get the cost for walking over a Tile
    //     MooreNeighborhood::new(width, height), // the Neighborhood
    //     PathCacheConfig {
    //         ..Default::default()
    //     }, // config
    // );
    use std::time;
    let a = time::Instant::now();
    let mut pathfinding = make_pathcache(width, height, &map);

    let b = time::Instant::now();
    let c = b - a;

    println!("finished creating the pathcache in {:?}", c);

    // add a solid horizontal wall across the map, then update the pathcache

    let mut changed = Vec::with_capacity(width);
    let temp_y = 32;
    for x in 0..width {
        map.set_cost(x, temp_y, -1);
        changed.push((x, temp_y))
    }

    println!("prepped the solid wall");

    let d = time::Instant::now();
    pathfinding.tiles_changed(&changed, |(x, y)| map.get_tile_cost(x, y));
    let e = time::Instant::now();

    println!("updated the pathmap with the changed tiles in {:?}", e - d);

    // for y in 0..height {
    //     for x in 0..width {
    //         let path = pathfinding.find_path(start, (x, y), |(x2, y2)| map.get_tile_cost(x2, y2));
    //         // print!(
    //         //     "position: ({}, {})   //   do we have a path? {}   //",
    //         //     x,
    //         //     y,
    //         //     path.is_some()
    //         // );
    //         // if let Some(temp) = path {
    //         //     println!("   path cost = {}", temp.cost())
    //         // }
    //     }
    // }
    // let path = pathfinding.find_path(start, end, |(x, y)| map.get_tile_cost(x, y));
    // println!("do we have a path? {}", path.is_some());
    // if let Some(temp) = path {
    //     println!("path cost = {}", temp.cost())
    // }
}

// #[profiling::function]
fn make_pathcache(width: usize, height: usize, map: &Map) -> PathCache<MooreNeighborhood> {
    PathCache::new_parallel(
        (width, height),                       // the size of the Grid
        |(x, y)| map.get_tile_cost(x, y),      // get the cost for walking over a Tile
        MooreNeighborhood::new(width, height), // the Neighborhood
        PathCacheConfig {
            chunk_size: 32,
            ..Default::default()
        }, // config
    )
}

// #[profiling::function]
fn update_pathcache() {}

pub struct Map {
    tiles: HashMap<usize, Tile>,
    width: usize,
    height: usize,
}

impl Map {
    // #[profiling::function]
    pub fn new(width: usize, height: usize) -> Self {
        let tile_count = width * height;
        let hashmap = HashMap::with_capacity(tile_count);
        Map {
            tiles: hashmap,
            width,
            height,
        }
    }

    // #[profiling::function]
    pub fn set_cost(&mut self, x: usize, y: usize, cost: isize) {
        let pos = self.get_tile_index(x, y);
        if let Some(pos) = pos {
            let tile = self.tiles.entry(pos).or_insert(Tile { cost: cost });
            tile.cost = cost;
            // self.tiles.get_mut(&pos).cost = cost;
        }
    }

    // #[profiling::function]
    fn get_tile_cost(&self, x: usize, y: usize) -> isize {
        let index = self.get_tile_index(x, y).unwrap();

        match self.tiles.get(&index) {
            Some(tile) => tile.cost,
            None => 1,
        }
    }

    // #[profiling::function]
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
