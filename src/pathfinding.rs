// https://github.com/samueltardieu/pathfinding
// #![allow(unused)]
use crate::Map;
use pathfinding::prelude::{absdiff, astar};

impl Map {
    // We're just going to assume 2d moore neighbourhood for everything (aka 4 cardinal directions + diagonals)
    fn get_neighbours_pathfinding(&self, pos: (usize, usize)) -> Vec<((usize, usize), usize)> {
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
            let mut cost = self.get_tile_cost(x, y) as usize;
            if cost == 0 {
                cost = 1;
            }
            // Safe because we filtered out all the costs below 0
            ((x, y), cost)
        })
        .collect()
    }
}

fn distance(start: (usize, usize), end: (usize, usize)) -> usize {
    absdiff(start.0, end.0) + absdiff(start.1, end.1)
}

pub fn find_path(
    map: &Map,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<(Vec<(usize, usize)>, usize)> {
    let temp = astar(
        &start,
        |p| map.get_neighbours_pathfinding(*p),
        |p| distance(*p, end) / 3,
        |p| *p == end,
    );

    temp
}
