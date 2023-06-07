#![allow(non_snake_case)]
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
    pub id_user: u64,
    pub id_type: u64,
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
