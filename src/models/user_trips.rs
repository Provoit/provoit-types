#[cfg(feature = "diesel")]
use diesel::prelude::*;

#[cfg(feature = "diesel")]
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "diesel", derive(Queryable))]
#[cfg_attr(feature = "diesel", diesel(primary_key(id_user, id_trip)))]
#[cfg_attr(feature = "diesel", diesel(table_name = user_trips))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTrip {
    pub id_user: u64,
    pub id_trip: u64,
}

#[cfg_attr(feature = "diesel", derive(Insertable))]
#[cfg_attr(feature = "diesel", diesel(table_name = user_trips))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUserTrip {
    pub id_user: u64,
    pub id_trip: u64,
}
