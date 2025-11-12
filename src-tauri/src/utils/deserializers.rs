use chrono::NaiveDateTime;
use serde::Deserialize;

/// Deserializes NaiveDateTime into String
///
/// # Usage
///
/// ```rust,ignore
/// #[derive(Deserialize)]
/// struct MyStruct {
///     #[serde(deserialize_with = "naive_datetime_from_str")]
///     my_datetime: NaiveDateTime,
/// }
/// ```
pub fn naive_datetime_from_str<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = match Deserialize::deserialize(deserializer) {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    return NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| serde::de::Error::custom(e.to_string()));
}
