#[cfg(feature = "diesel")]
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "diesel", derive(Queryable))]
#[cfg_attr(feature = "diesel", diesel(table_name = frequencies))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frequency {
    pub id: u64,
    pub label: String,
}

// `frequencies` is an enumeration table, is shouldn't need to be updated or created via model.
