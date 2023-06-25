use std::collections::HashMap;

#[cfg(feature = "diesel")]
use diesel::prelude::*;

#[cfg(feature = "diesel")]
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "diesel", derive(Queryable))]
#[cfg_attr(feature = "diesel", diesel(belongs_to(Frequencies, foreign_key = id)))]
#[cfg_attr(feature = "diesel", diesel(belongs_to(Vehicle, foreign_key = id)))]
#[cfg_attr(feature = "diesel", diesel(belongs_to(Timings, foreign_key = id)))]
#[cfg_attr(feature = "diesel", diesel(table_name = trips))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Trip {
    pub id: u64,
    pub start: String,
    pub end: String,
    pub max_people: u8,
    pub price: f32,
    pub recurring: bool,
    pub id_frequency: Option<u64>,
    pub id_vehicle: u64,
    pub id_start_timing: u64,
    pub id_end_timing: u64,
}

#[cfg_attr(feature = "diesel", derive(Insertable))]
#[cfg_attr(feature = "diesel", diesel(table_name = trips))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTrip {
    pub start: String,
    pub end: String,
    pub max_people: u8,
    pub price: f32,
    pub recurring: bool,
    pub id_frequency: Option<u64>,
    pub id_vehicle: u64,
    pub id_start_timing: u64,
    pub id_end_timing: u64,
}

/// Convert to a NewTrip from an event's HashMap
impl From<HashMap<String, String>> for NewTrip {
    fn from(value: HashMap<String, String>) -> Self {
        Self {
            start: value
                .get("start")
                .expect("Le lieu de départ est requis")
                .to_owned(),
            end: value
                .get("end")
                .expect("Le lieu d'arriver est obligatoire")
                .to_owned(),
            max_people: value
                .get("max_people")
                .expect("Le nombre de personnes est obligatoire")
                .parse()
                .expect("Nombre de personnes invalide"),
            price: value
                .get("price")
                .expect("Le prix est requis")
                .parse()
                .expect("Prix invalide"),
            recurring: value
                .get("recurring")
                .unwrap_or(&"false".to_string())
                .eq("true"),
            id_frequency: value
                .get("id_frequency")
                .map(|v| v.parse().expect("Fréquence invalide")),
            id_vehicle: value
                .get("id_vehicle")
                .expect("Le véhicule est requis")
                .parse()
                .expect("Véhicule invalide"),
            id_start_timing: 0,
            id_end_timing: 0,
        }
    }
}

#[cfg_attr(feature = "diesel", derive(AsChangeset))]
#[cfg_attr(feature = "diesel", diesel(table_name = trips))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTrip {
    pub start: Option<String>,
    pub end: Option<String>,
    pub max_people: Option<u8>,
    pub price: Option<f32>,
    pub recurring: Option<bool>,
    pub id_frequency: Option<Option<u64>>,
    pub id_vehicle: Option<u64>,
    pub id_start_timing: Option<u64>,
    pub id_end_timing: Option<u64>,
}
