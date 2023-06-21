#![allow(non_snake_case)]
use std::collections::HashMap;

#[cfg(feature = "diesel")]
use diesel::prelude::*;

#[cfg(feature = "diesel")]
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "diesel", derive(Queryable))]
#[cfg_attr(feature = "diesel", diesel(belongs_to(User, foreign_key = id)))]
#[cfg_attr(feature = "diesel", diesel(belongs_to(VehicleType, foreign_key = id)))]
#[cfg_attr(feature = "diesel", diesel(table_name = vehicles))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: u64,
    pub name: String,
    pub year: u16,
    pub nb_doors: u8,
    pub nb_seats: u8,
    pub trunk_size_L: u16,
    pub pic: Option<Vec<u8>>,
    pub id_user: u64,
    pub id_type: u64,
}

#[cfg_attr(feature = "diesel", derive(Insertable))]
#[cfg_attr(feature = "diesel", diesel(table_name = vehicles))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewVehicle {
    pub name: String,
    pub year: u16,
    pub nb_doors: u8,
    pub nb_seats: u8,
    pub trunk_size_L: u16,
    pub pic: Option<Vec<u8>>,
    pub id_user: Option<u64>,
    pub id_type: u64,
}

/// Convert to a NewVehicle from an event's HashMap
impl From<HashMap<String, String>> for NewVehicle {
    fn from(value: HashMap<String, String>) -> Self {
        Self {
            name: value.get("name").expect("Le nom est requis").to_owned(),
            year: value
                .get("year")
                .expect("Année requise")
                .parse()
                .expect("Année invalide"),
            nb_doors: value
                .get("nb_doors")
                .expect("Le nombre de porte est requis")
                .parse()
                .expect("Nombre de portes invalide"),
            nb_seats: value
                .get("nb_seats")
                .expect("Le nombre de sièges est requis")
                .parse()
                .expect("Nombre de sièges invalide"),
            trunk_size_L: value
                .get("trunk_size_L")
                .expect("La taille du coffre est requise")
                .parse()
                .expect("Taille du coffre incorrecte"),
            pic: None,
            id_user: None,
            id_type: 1,
        }
    }
}

#[cfg_attr(feature = "diesel", derive(AsChangeset))]
#[cfg_attr(feature = "diesel", diesel(table_name = vehicles))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVehicle {
    pub name: Option<String>,
    pub year: Option<u16>,
    pub nb_doors: Option<u8>,
    pub nb_seats: Option<u8>,
    pub trunk_size_L: Option<u16>,
    pub pic: Option<Option<Vec<u8>>>,
    pub id_type: Option<u64>,
}
