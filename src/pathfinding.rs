use std::collections::BinaryHeap;

use bracket_pathfinding::prelude::{Algorithm2D, Point};

use crate::game::Unit;
use crate::map::Map;

#[derive(Debug)]
pub struct DijkstraMap {
    reachables: Vec<Point>, // INFO may be make this a Hashset?
    dijkstra_map: Vec<u32>,
    max_distance: u32,
    width: usize,
    heigth: usize,
}
impl DijkstraMap {
    pub const UNREACHABLE: u32 = u32::MAX;
    const DIRS: [Point; 4] = [
        Point::constant(0, 1),
        Point::constant(0, -1),
        Point::constant(1, 0),
        Point::constant(-1, 0),
    ];

    pub fn new(map: &Map, unit: &Unit) -> Self {
        let mut dijkstra_map = vec![Self::UNREACHABLE; map.width * map.height];
        let mut heap = BinaryHeap::new();
        let mut reachables = Vec::new();

        dijkstra_map[map.point2d_to_index(unit.pos)] = 0;
        heap.push(Node {
            pos: unit.pos,
            dist: 0,
        });

        while let Some(Node { pos, dist }) = heap.pop() {
            if dist > dijkstra_map[map.point2d_to_index(pos)] {
                continue;
            }
            if dist > unit.movement {
                continue;
            }

            reachables.push(pos);

            for dir in Self::DIRS {
                let npos = pos + dir;

                if !map.in_bounds(npos) {
                    continue;
                }

                let move_cost = unit.get_movement_cost(map.get_terrain(npos));
                if move_cost == Self::UNREACHABLE {
                    continue;
                }

                let next_dist = dist.saturating_add(move_cost);
                let next_idx = map.point2d_to_index(npos);
                let prev_dist = dijkstra_map[next_idx];

                if next_dist <= unit.movement
                    && (prev_dist == Self::UNREACHABLE || next_dist < prev_dist)
                {
                    dijkstra_map[map.point2d_to_index(npos)] = next_dist;
                    heap.push(Node {
                        pos: npos,
                        dist: next_dist,
                    });
                }
            }
        }

        DijkstraMap {
            reachables,
            dijkstra_map,
            max_distance: unit.movement,
            width: map.width,
            heigth: map.height,
        }
    }

    pub fn get_reachables(&self) -> &[Point] {
        &self.reachables
    }

    /// Caller should ensure the point is reachable
    pub fn get_path(&self, from: impl Into<Point>) -> Vec<Point> {
        let from = from.into();
        let start_d = self.dijkstra_map[self.idx(from)];
        assert!(start_d != Self::UNREACHABLE);

        let mut path = Vec::new();
        let mut current = from;
        path.push(from);

        let max_steps = self.max_distance + 5;
        for _ in 0..max_steps {
            let cd = self.dijkstra_map[self.idx(current)];

            if cd == 0 {
                break;
            }

            let mut best = current;
            let mut best_d = cd;

            for dir in Self::DIRS {
                let npos = current + dir;
                if !self.reachables.contains(&npos) {
                    continue;
                }

                let nd = self.dijkstra_map[self.idx(npos)];
                if nd < best_d {
                    best_d = nd;
                    best = npos;
                }
            }

            if best == current {
                path.clear();
                break;
            }

            current = best;
            if self.dijkstra_map[self.idx(current)] != 0 {
                path.push(current);
            }
        }

        path
    }

    fn idx(&self, pos: impl Into<Point>) -> usize {
        let pos = pos.into();
        ((pos.y * self.width as i32) + pos.x) as usize
    }
}

#[derive(PartialEq, Eq)]
struct Node {
    pos: Point,
    dist: u32,
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| other.pos.x.cmp(&self.pos.x))
            .then_with(|| other.pos.y.cmp(&self.pos.y))
    }
}
