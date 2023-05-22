#[cfg(feature = "diesel")]
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "diesel", derive(Queryable))]
#[cfg_attr(feature = "diesel", diesel(table_name = vehicle_types))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleType {
    id: u64,
    label: String,
}

// `vehicle_types` is an enumeration table, is shouldn't need to be updated or created via model.
