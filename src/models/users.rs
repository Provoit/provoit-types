use chrono::naive::NaiveDateTime;
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
    pub id: u64,
    pub firstname: String,
    pub lastname: String,
    pub mail: String,
    #[serde(skip)]
    pub passwd: String,
    #[serde(skip)]
    pub token: Option<String>,
    #[serde(skip)]
    pub token_gentime: Option<NaiveDateTime>,
    pub profile_pic: Option<Blob>,
    pub smoker: bool,
    pub id_favorite_vehicle: Option<u64>,
}

#[cfg_attr(feature = "diesel", derive(Insertable))]
#[cfg_attr(feature = "diesel", diesel(table_name = users))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
    pub mail: String,
    pub passwd: String,
    pub profile_pic: Option<Blob>,
    pub smoker: bool,
    pub id_favorite_vehicle: Option<u64>,
}

#[cfg_attr(feature = "diesel", derive(AsChangeset))]
#[cfg_attr(feature = "diesel", diesel(table_name = users))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub mail: Option<String>,
    pub profile_pic: Option<Option<Blob>>,
    pub smoker: Option<bool>,
    pub id_favorite_vehicle: Option<Option<u64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    pub mail: String,
    pub passwd: String,
}

