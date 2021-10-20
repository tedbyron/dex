//! Encounter types.

use crate::utility::{Name, NamedAPIResource};
use pokeapi_macro::pokeapi_struct;

#[pokeapi_struct]
struct EncounterMethod {
    id: i32,
    name: String,
    order: i32,
    names: Vec<Name>,
}

#[pokeapi_struct]
struct EncounterCondition {
    id: i32,
    name: String,
    names: Vec<Name>,
    values: Vec<NamedAPIResource<EncounterConditionValue>>,
}

#[pokeapi_struct]
struct EncounterConditionValue {
    id: i32,
    name: String,
    condition: NamedAPIResource<EncounterCondition>,
    names: Vec<Name>,
}
