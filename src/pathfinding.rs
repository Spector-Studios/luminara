use crate::map::Map;
use crate::math::Point;
use crate::unit::Unit;
use crate::unit::UnitId;

use std::collections::HashSet;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
pub struct DijkstraMap {
    reachables: HashSet<Point>, // INFO may be make this a Hashset?
    map: Vec<u32>,
    max_distance: u32,
    width: usize,
    heigth: usize,
}
impl DijkstraMap {
    pub const UNREACHABLE: u32 = u32::MAX;
    const DIRS: [Point; 4] = [
        Point::new(0, 1),
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(-1, 0),
    ];

    pub fn new(map: &Map, target: &Unit, units: &HashMap<UnitId, Unit>) -> Self {
        let mut dijkstra_map = vec![Self::UNREACHABLE; map.width * map.height];
        let mut heap = BinaryHeap::new();
        let mut reachables = HashSet::new();

        dijkstra_map[map.point_to_idx(target.pos)] = 0;
        heap.push(Node {
            pos: target.pos,
            dist: 0,
        });

        while let Some(Node { pos, dist }) = heap.pop() {
            if dist > dijkstra_map[map.point_to_idx(pos)] {
                continue;
            }
            if dist > target.movement {
                continue;
            }

            reachables.insert(pos);

            for dir in Self::DIRS {
                let npos = pos + dir;

                if !map.in_bounds(npos) {
                    continue;
                }

                if units
                    .values()
                    .any(|unit| unit.faction != target.faction && unit.pos == npos)
                {
                    continue;
                }

                let move_cost = target.get_movement_cost(map.get_terrain(npos));
                if move_cost == Self::UNREACHABLE {
                    continue;
                }

                let next_dist = dist.saturating_add(move_cost);
                let next_idx = map.point_to_idx(npos);
                let prev_dist = dijkstra_map[next_idx];

                if next_dist <= target.movement
                    && (prev_dist == Self::UNREACHABLE || next_dist < prev_dist)
                {
                    dijkstra_map[map.point_to_idx(npos)] = next_dist;
                    heap.push(Node {
                        pos: npos,
                        dist: next_dist,
                    });
                }
            }
        }

        DijkstraMap {
            reachables,
            map: dijkstra_map,
            max_distance: target.movement,
            width: map.width,
            heigth: map.height,
        }
    }

    pub fn get_reachables(&self) -> &HashSet<Point> {
        &self.reachables
    }

    /// Caller should ensure the point is reachable
    pub fn get_path(&self, from: impl Into<Point>) -> Vec<Point> {
        let from = from.into();
        let start_d = self.map[self.idx(from)];
        assert!(start_d != Self::UNREACHABLE);

        let mut path = Vec::new();
        let mut current = from;
        path.push(from);

        let max_steps = self.max_distance + 5;
        for _ in 0..max_steps {
            let cd = self.map[self.idx(current)];

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

                let nd = self.map[self.idx(npos)];
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
            if self.map[self.idx(current)] != 0 {
                path.push(current);
            }
        }

        path
    }

    #[allow(clippy::cast_sign_loss)]
    fn idx(&self, pos: impl Into<Point>) -> usize {
        let pos = pos.into();
        assert!(pos >= Point::zero());
        (pos.y as usize * self.width) + pos.x as usize
    }
}

pub fn get_targetables(attacker: &Unit) -> Vec<Point> {
    let start = attacker.pos;
    let range = 2i32; // TODO Get range from unit

    let mut points = Vec::new();

    for dx in -range..=range {
        for dy in -range..=range {
            if dx.abs() + dy.abs() <= range {
                points.push(start + (dx, dy).into());
            }
        }
    }

    points
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
