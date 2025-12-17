use std::collections::*;

use rand::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Entity(u64);

pub fn create_entity(entity_db: &mut HashSet<Entity>, rng: &mut impl Rng) -> Entity{
    loop{
        let ent = Entity(rng.random());
        if entity_db.insert(ent) {
            return ent
        }
    }
}