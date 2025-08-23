use std::collections::HashMap;

use crate::Map;
use crate::math::Point;
use crate::unit::ErasedUnit;
use crate::unit::Unit;
use crate::unit::UnitId;

// TODO Make a builder for this
pub struct WorldState {
    pub units: HashMap<UnitId, Unit>,
    pub map: Map,
    next_unit_id: UnitId,
}

impl WorldState {
    pub fn new(map: Map) -> Self {
        Self {
            units: HashMap::with_capacity(20),
            map,
            next_unit_id: UnitId::new(0),
        }
    }

    pub fn spawn_units(&mut self, units: &[ErasedUnit]) {
        for unit in units {
            self.units.insert(
                self.next_unit_id,
                Unit::from_erased(self.next_unit_id, *unit),
            );
            self.next_unit_id.next();
        }
    }

    pub fn setup_turn(&mut self) {
        self.units.iter_mut().for_each(|(_, unit)| {
            unit.turn_complete = false;
        });
    }

    pub fn get_unmoved_unit(&self, faction: Faction) -> Option<&UnitId> {
        self.units
            .iter()
            .filter(|(_, unit)| unit.faction == faction)
            .find(|(_, unit)| unit.turn_complete == false)
            .map(|(id, _)| id)
    }

    pub fn get_unmoved_by_pos(&self, faction: Faction, pos: impl Into<Point>) -> Option<UnitId> {
        let pos = pos.into();
        self.units
            .iter()
            .filter(|(_, unit)| unit.faction == faction && unit.turn_complete == false)
            .find(|(_, unit)| unit.pos == pos)
            .map(|(id, _)| *id)
    }

    pub fn is_tile_empty(&self, pos: impl Into<Point>, except: Option<UnitId>) -> bool {
        let pos = pos.into();
        !self
            .units
            .iter()
            .any(|(id, unit)| unit.pos == pos && Some(*id) != except)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Faction {
    Player,
    Neutral,
    Enemy,
}
