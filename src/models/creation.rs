//! This module holds the types needed to create composite types.
//! That is types that spans across multiple tables.

use serde::{Deserialize, Serialize};

use super::{timings::NewTiming, trips::NewTrip};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTrip {
    pub trip: NewTrip,
    pub start_timing: NewTiming,
    pub end_timing: NewTiming,
    pub road_types: Vec<u64>,
}
