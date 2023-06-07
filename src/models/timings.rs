use chrono::{NaiveDate, NaiveTime};
#[cfg(feature = "diesel")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "diesel")]
use super::days::Day;
#[cfg(feature = "diesel")]
use crate::schema::*;

#[cfg_attr(feature = "diesel", derive(Queryable, Associations))]
#[cfg_attr(feature = "diesel", diesel(belongs_to(Day, foreign_key = id)))]
#[cfg_attr(feature = "diesel", diesel(table_name = timings))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timing {
    pub id: u64,
    pub date: Option<NaiveDate>,
    pub time: NaiveTime,
    pub id_day: Option<u64>,
}

#[cfg_attr(feature = "diesel", derive(Insertable))]
#[cfg_attr(feature = "diesel", diesel(table_name = timings))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTiming {
    pub date: Option<NaiveDate>,
    pub time: NaiveTime,
    pub id_day: Option<u64>,
}

#[cfg_attr(feature = "diesel", derive(AsChangeset))]
#[cfg_attr(feature = "diesel", diesel(table_name = timings))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTiming {
    pub date: Option<Option<NaiveDate>>,
    pub time: Option<NaiveTime>,
    pub id_day: Option<Option<u64>>,
}
