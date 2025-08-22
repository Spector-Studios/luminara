use std::collections::HashMap;

use crate::Map;
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

    pub fn spawn_units(&mut self, units: &[Unit]) {
        for unit in units {
            self.units.insert(self.next_unit_id, *unit);
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
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Faction {
    Player,
    Neutral,
    Enemy,
}
