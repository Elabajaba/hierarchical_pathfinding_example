// https://github.com/petgraph/petgraph
#![allow(unused)]
use crate::Map;
use log::info;
use petgraph::algo::astar;
use petgraph::graph::{NodeIndex, UnGraph};

impl Map {
    // fn distance(&self, start: (usize, usize), end: (usize, usize)) -> usize {
    //     absdiff(start.0, end.0) + absdiff(start.1, end.1)
    // }

    // We're just going to assume 2d moore neighbourhood for everything (aka 4 cardinal directions + diagonals)
    fn get_neighbours_petgraph(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        // let mut neighbours = Vec::new();
        let x = pos.0 as isize;
        let y = pos.1 as isize;

        vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
        .into_iter()
        .filter(|(x, y)| {
            if *x >= 0 && *x < self.width as isize && *y >= 0 && *y < self.height as isize {
                // Filter out negative cost edges
                if self.get_tile_cost(*x as usize, *y as usize) >= 0 {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .map(|(x, y)| {
            let x = x as usize;
            let y = y as usize;
            let index = self.get_tile_index(x, y).unwrap();
            // Safe because we filtered out all the costs below 0
            (index, self.get_tile_cost(x, y) as usize)
        })
        .collect()
    }
}

// Returns the graph, and a vec of NodeIndex required to work with the graph
// Positions of NodeIndex in the vec correspond to their indices in the map.
pub fn create_petgraph(map: &Map) -> (UnGraph<isize, usize>, Vec<NodeIndex>) {
    let map_size = map.height * map.width;
    let mut g: UnGraph<isize, usize> = UnGraph::with_capacity(map_size, map_size * 7);
    let mut nodes = Vec::with_capacity(map_size);
    for index in 0..map_size {
        let pos = map
            .get_tile_position(index)
            .expect("Error in create_petgraph. Index out of range.");
        // let node = g.add_node(map.get_tile_cost(pos.0, pos.1));
        let node = g.add_node(1); // Not sure what node weight does
        nodes.push(node);
    }

    for (index, node) in nodes.iter().enumerate() {
        let pos = map
            .get_tile_position(index)
            .expect("Error in create_petgraph. Index out of range.");
        let neighbours = map.get_neighbours_petgraph(pos);

        for (neighbour, cost) in neighbours {
            g.add_edge(*node, nodes[neighbour], cost);
        }
    }
    (g, nodes)
}

// Use the NodeIndex for start and end, not the map index.
// TODO: Does this even matter?
pub fn get_path(
    graph: UnGraph<isize, usize>,
    start: NodeIndex,
    end: NodeIndex,
) -> Option<(usize, Vec<NodeIndex>)> {
    let path = astar(
        &graph,
        start,
        |finish| finish == end,
        |e| *e.weight(),
        |_| 0,
    );

    path
}
