#[cfg(feature = "diesel")]
use diesel::prelude::*;

#[cfg(feature = "diesel")]
use crate::schema::*;
use crate::Blob;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "diesel", derive(Queryable))]
#[cfg_attr(feature = "diesel", diesel(table_name = users))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: u64,
    firstname: String,
    lastname: String,
    mail: String,
    passwd: String,
    profile_pic: Option<Blob>,
    smoker: bool,
    id_favorite_vehicle: Option<u64>,
}

#[cfg_attr(feature = "diesel", derive(Insertable))]
#[cfg_attr(feature = "diesel", diesel(table_name = users))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    firstname: String,
    lastname: String,
    mail: String,
    passwd: String,
    profile_pic: Option<Blob>,
    smoker: bool,
    id_favorite_vehicle: Option<u64>,
}

#[cfg_attr(feature = "diesel", derive(AsChangeset))]
#[cfg_attr(feature = "diesel", diesel(table_name = users))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    firstname: Option<String>,
    lastname: Option<String>,
    mail: Option<String>,
    profile_pic: Option<Option<Blob>>,
    smoker: Option<bool>,
    id_favorite_vehicle: Option<Option<u64>>,
}
