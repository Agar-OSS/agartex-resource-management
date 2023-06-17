use sqlx::types::chrono::{DateTime, NaiveDateTime};

pub mod json_time {
	use super::*;
	use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error};
    
	pub fn serialize<S: Serializer>(time: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error> {
		time.to_string().serialize(serializer)
	}

	pub fn _deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDateTime, D::Error> {
	 	let time: String = Deserialize::deserialize(deserializer)?;
	 	Ok(DateTime::parse_from_rfc3339(time.as_str()).map_err(D::Error::custom)?.naive_utc())
	}
}
