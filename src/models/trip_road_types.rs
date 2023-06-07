#[cfg(feature = "diesel")]
use diesel::prelude::*;

#[cfg(feature = "diesel")]
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "diesel", derive(Queryable))]
#[cfg_attr(feature = "diesel", diesel(primary_key(id_trip, id_road_type)))]
#[cfg_attr(feature = "diesel", diesel(belongs_to(id_road_type, foreign_key = id)))]
#[cfg_attr(feature = "diesel", diesel(belongs_to(id_trip, foreign_key = id)))]
#[cfg_attr(feature = "diesel", diesel(table_name = trips_road_types))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripsRoadTypes{
    pub id_trip: u64,
    pub id_road_type: u64,
}

#[cfg_attr(feature = "diesel", derive(Insertable))]
#[cfg_attr(feature = "diesel", diesel(table_name = trips_road_types))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTripsRoadTypes{
    pub id_trip: u64,
    pub id_road_type: u64,
}
