#[cfg(feature = "diesel")]
use diesel::prelude::*;

#[cfg(feature = "diesel")]
use crate::schema::*;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "diesel", derive(Queryable))]
#[cfg_attr(feature = "diesel", diesel(table_name = users))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: u64,
    name: String,
}

#[cfg_attr(feature = "diesel", derive(Insertable))]
#[cfg_attr(feature = "diesel", diesel(table_name = users))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    name: String,
}
