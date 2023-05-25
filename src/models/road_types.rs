#[cfg(feature = "diesel")]
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "diesel", derive(Queryable))]
#[cfg_attr(feature = "diesel", diesel(table_name = road_types))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadType {
    id: u64,
    label: String,
}

// `road_types` is an enumeration table, is shouldn't need to be updated or created via model.
