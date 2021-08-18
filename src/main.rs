use env_logger::Env;
use hierarchical_pathfinding::prelude::*;
use log::info;
// use rand::prelude::*;
use oorandom::Rand32;
use std::fs::File;
use std::io::{prelude::*, BufWriter, IoSlice};

mod pathfinding;
mod petgraph;

fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let width = 1024;
    let height = 1024;

    let start = (40, 90);
    let end = (900, 601);

    #[allow(unused_mut)]
    let mut map = Map::new_random(width, height);

    info!("Random map generated");

    use std::time::Instant;
    let a = Instant::now();

    #[allow(unused_mut)]
    let mut pathfinding = make_pathcache(width, height, &map);

    info!(
        "finished creating the pathcache in {:?}",
        Instant::now() - a
    );

    // add a solid horizontal wall across the map, then update the pathcache
    let mut changed = Vec::with_capacity(width);
    let temp_y = 63;
    for x in 0..width {
        map.set_cost(x, temp_y, -1);
        changed.push((x, temp_y))
    }

    info!("prepped the solid wall");

    let d = Instant::now();
    pathfinding.tiles_changed(&changed, |(x, y)| map.get_tile_cost(x, y));
    let e = Instant::now();

    info!("updated the pathmap with the changed tiles in {:?}", e - d);

    // info!("printing the map");
    // let map_print_timer = Instant::now();
    // map.print_map(Some("map.txt")).expect("Failed to print map");
    // info!("Printed the map in {:?}", Instant::now() - map_print_timer);

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
    //         //     info!("   path cost = {}", temp.cost())
    //         // }
    //     }
    // }

    // info!(
    //     "start position: ({}, {}), start cost: {}",
    //     start.0,
    //     start.1,
    //     map.get_tile_cost(start.0, start.1)
    // );
    // info!(
    //     "end position: ({}, {}), end cost: {}",
    //     end.0,
    //     end.1,
    //     map.get_tile_cost(end.0, end.1)
    // );

    // let path_timer = Instant::now();
    let path = pathfinding.find_path(start, end, |(x, y)| map.get_tile_cost(x, y));
    // info!("hierarchial path timer:{:?}", Instant::now() - path_timer);
    // info!("do we have a path? {}", path.is_some());

    // let path_timer = Instant::now();
    // let pathfinding_path = crate::pathfinding::find_path(&map, start, end);
    // info!("pathfinding path timer:{:?}", Instant::now() - path_timer);
    // info!("do we have a path? {}", pathfinding_path.is_some());

    // let petgraph_setup_timer = Instant::now();
    // let (petgraph_graph, nodes) = petgraph::create_petgraph(&map);
    // info!(
    //     "petgraph setup timer:{:?}",
    //     Instant::now() - petgraph_setup_timer
    // );
    // let path_timer = Instant::now();
    // let petgraph_path = petgraph::get_path(
    //     petgraph_graph,
    //     nodes[map.get_tile_index(start.0, start.1).unwrap()],
    //     nodes[map.get_tile_index(end.0, end.1).unwrap()],
    // );
    // info!("petgraph path timer:{:?}", Instant::now() - path_timer);
    // info!("do we have a path? {}", petgraph_path.is_some());

    // if let Some(temp) = path {
    //     info!("path cost = {}", temp.cost())
    // }

    // sleep(std::time::Duration::new(10, 0));
}

fn make_pathcache(width: usize, height: usize, map: &Map) -> PathCache<MooreNeighborhood> {
    PathCache::new(
        (width, height),                       // the size of the Grid
        |(x, y)| map.get_tile_cost(x, y),      // get the cost for walking over a Tile
        MooreNeighborhood::new(width, height), // the Neighborhood
        // PathCacheConfig::with_chunk_size(32),
        PathCacheConfig {
            chunk_size: 32,
            // perfect_paths: true,
            ..Default::default()
        },
    )
}

#[allow(unused)]
fn update_pathcache() {}

pub struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    #[allow(unused)]
    pub fn new(width: usize, height: usize) -> Self {
        let tile_count = width * height;
        Map {
            tiles: vec![Tile { cost: 1 }; tile_count],
            width,
            height,
        }
    }

    #[allow(unused)]
    pub fn new_random(width: usize, height: usize) -> Self {
        let tile_count = width * height;
        let mut tiles = Vec::with_capacity(tile_count);
        let mut rng = Rand32::new(4); // 4 breaks pathfinding, but petgraph and hierarchial work
        for _ in 0..tile_count {
            tiles.push(Tile {
                cost: rng.rand_range(0..10) as isize - 1,
            });
        }
        Map {
            tiles,
            width,
            height,
        }
    }

    pub fn set_cost(&mut self, x: usize, y: usize, cost: isize) {
        let pos = self.get_tile_index(x, y);
        if let Some(pos) = pos {
            self.tiles[pos].cost = cost;
        }
    }

    fn get_tile_cost(&self, x: usize, y: usize) -> isize {
        let index = self.get_tile_index(x, y).unwrap();
        self.tiles[index].cost
    }

    fn get_tile_index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            // Index out of bounds
            return None;
        }

        Some(x + y * self.width)
    }

    fn get_tile_position(&self, index: usize) -> Option<(usize, usize)> {
        if index >= self.tiles.len() {
            None
        } else {
            let x = index % self.width;
            let y = index / self.width;
            Some((x, y))
        }
    }

    /// Prints the map to a file. If no file is give, prints to the terminal.
    #[allow(unused)]
    fn print_map(&self, file_path: Option<&str>) -> std::io::Result<()> {
        let mut outputs = Vec::with_capacity(self.height);
        let mut row = String::with_capacity(self.width);
        for (index, tile) in self.tiles.iter().enumerate() {
            // Should be infallible, we're iterating through our tiles.
            let (x, y) = self.get_tile_position(index).unwrap();
            // Push the row into rows, and reset it. Special case for the first and last tiles.
            if index == self.tiles.len() - 1 {
                let cost = tile.cost;
                if cost < 0 {
                    row.push('#');
                } else {
                    row.push('.');
                }
                outputs.push(row.clone());
            } else {
                if (x == 0 && index != 0) {
                    row.push('\n');
                    outputs.push(row.clone());
                    row.clear();
                }

                let cost = tile.cost;
                if cost < 0 {
                    row.push('#');
                } else {
                    row.push('.');
                }
            }
        }

        if let Some(file_path) = file_path {
            let file = File::create(file_path)?;
            let mut buf_writer = BufWriter::new(file);
            for row in outputs {
                let temp = IoSlice::new(row.as_bytes());
                buf_writer.write(&temp);
            }
        } else {
            // Write to terminal
            for row in &outputs {
                println!("{:?}", row);
            }
        }

        Ok(())
    }

    //     #[allow(unused)]
    //     fn print_map_with_path(
    //         &self,
    //         file_path: Option<&str>,
    //         path: Path,
    //         start: (usize, usize),
    //         end: (usize, usize),
    //     ) -> std::io::Result<()> {
    //         let mut outputs = Vec::with_capacity(self.height);
    //         let mut row = String::with_capacity(self.width);
    //         for (index, tile) in self.tiles.iter().enumerate() {
    //             // Should be infallible, we're iterating through our tiles.
    //             let (x, y) = self.get_tile_position(index).unwrap();
    //             // Push the row into rows, and reset it. Special case for the first and last tiles.
    //             if index == self.tiles.len() - 1 {
    //                 let cost = tile.cost;
    //                 if cost < 0 {
    //                     row.push('#');
    //                 } else {
    //                     row.push('.');
    //                 }
    //                 outputs.push(row.clone());
    //             } else {
    //                 if (x == 0 && index != 0) {
    //                     row.push('\n');
    //                     outputs.push(row.clone());
    //                     row.clear();
    //                 }

    //                 let cost = tile.cost;
    //                 if cost < 0 {
    //                     row.push('#');
    //                 } else {
    //                     row.push('.');
    //                 }
    //             }
    //         }

    //         if let Some(file_path) = file_path {
    //             let file = File::create(file_path)?;
    //             let mut buf_writer = BufWriter::new(file);
    //             for row in outputs {
    //                 let temp = IoSlice::new(row.as_bytes());
    //                 buf_writer.write(&temp);
    //             }
    //         } else {
    //             // Write to terminal
    //             for row in &outputs {
    //                 println!("{:?}", row);
    //             }
    //         }

    //         Ok(())
    //     }
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    cost: isize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_map(width: usize, height: usize) -> Map {
        let map = Map::new(width, height);
        map
    }

    #[test]
    fn test_get_tile_position() {
        let (width, height) = (8, 5);
        let map = init_map(width, height);

        assert_eq!(map.get_tile_position(3), Some((3, 0)));
        assert_eq!(map.get_tile_position(8), Some((0, 1)));
        assert_eq!(map.get_tile_position(21), Some((5, 2)));
        assert_eq!(map.get_tile_position(42), None);
    }
}
