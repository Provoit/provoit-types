#[cfg(feature = "diesel")]
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "diesel", derive(Queryable))]
#[cfg_attr(feature = "diesel", diesel(table_name = days))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Day {
    pub id: u64,
    pub label: String,
}

// `days` is an enumeration table, is shouldn't need to be updated or created via model.
