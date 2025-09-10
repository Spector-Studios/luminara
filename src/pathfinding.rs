use macroquad::prelude::warn;

use crate::map::Map;
use crate::math::Point;
use crate::unit::Unit;
use crate::unit::UnitId;

use std::collections::HashSet;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
pub struct DijkstraMap {
    reachables: HashSet<Point>,
    map: Vec<u32>,
    came_from: Vec<Option<Point>>,
    start: Point,
    width: usize,
    heigth: usize,
}
impl DijkstraMap {
    pub const UNREACHABLE: u32 = u32::MAX;
    pub const DIRS: [Point; 4] = [
        Point::new(0, 1),
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(-1, 0),
    ];

    pub fn new(map: &Map, target: &Unit, units: &HashMap<UnitId, Unit>) -> Self {
        let mut dijkstra_map = vec![Self::UNREACHABLE; map.width * map.height];
        let mut came_from = vec![None; map.width * map.height];
        let mut reachables = HashSet::new();

        let mut heap = BinaryHeap::new();

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
                    dijkstra_map[next_idx] = next_dist;
                    came_from[next_idx] = Some(pos);
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
            came_from,
            start: target.pos,
            width: map.width,
            heigth: map.height,
        }
    }

    pub fn get_reachables(&self) -> &HashSet<Point> {
        &self.reachables
    }

    /// Caller should ensure the point is reachable
    pub fn get_path_to(&self, goal: impl Into<Point>) -> Vec<Point> {
        let goal = goal.into();
        let mut path = Vec::new();
        let mut current = goal;

        // bail if unreachable
        debug_assert!(self.map[self.idx(goal)] != Self::UNREACHABLE);
        if self.map[self.idx(goal)] == Self::UNREACHABLE {
            warn!("Requesting unreachable path");
            return path;
        }

        // backtrack using predecessors
        while current != self.start {
            path.push(current);
            if let Some(prev) = self.came_from[self.idx(current)] {
                current = prev;
            } else {
                // shouldn't happen unless goal isn't connected
                #[cfg(debug_assertions)]
                panic!("Path not found");

                return Vec::new();
            }
        }

        path.push(self.start);
        path
    }

    #[allow(clippy::cast_sign_loss)]
    fn idx(&self, pos: impl Into<Point>) -> usize {
        let pos = pos.into();
        debug_assert!(pos >= Point::zero());
        (pos.y as usize * self.width) + pos.x as usize
    }
}

pub fn get_manahattan_neighbours(from: Point, range: i32) -> impl Iterator<Item = Point> {
    (-range..=range).flat_map(move |dx| {
        (-range..=range).filter_map(move |dy| {
            if (dx, dy) == (0, 0) || dx.abs() + dy.abs() > range {
                None
            } else {
                Some(from + (dx, dy))
            }
        })
    })
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
