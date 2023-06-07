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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trips{
    pub id: u64,
    pub start: String,
    pub end: String,
    pub max_people: u8,
    pub price: f32,
    pub id_frequency: u64,
    pub id_vehicle: u64,
    pub id_start_timing: u64,
    pub id_end_timing: u64,
}


#[cfg_attr(feature = "diesel", derive(Insertable))]
#[cfg_attr(feature = "diesel", diesel(table_name = trips))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTrips{
    pub start: String,
    pub end: String,
    pub max_people: u8,
    pub price: f32,
    pub id_frequency: u64,
    pub id_vehicle: u64,
    pub id_start_timing: u64,
    pub id_end_timing: u64,
}


#[cfg_attr(feature = "diesel", derive(AsChangeset))]
#[cfg_attr(feature = "diesel", diesel(table_name = trips))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTrips{
    pub start: Option<String>,
    pub end: Option<String>,
    pub max_people: Option<u8>,
    pub price: Option<f32>,
    pub id_frequency: Option<u64>,
    pub id_vehicle: Option<u64>,
    pub id_start_timing: Option<u64>,
    pub id_end_timing: Option<u64>,
}
