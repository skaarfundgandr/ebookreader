use chrono::NaiveDateTime;

/// Serializes a string into NaiveDateTime
///
/// # Usage
///
/// ```rust,ignore
/// // Sample json record
/// pub struct Record {
///     id: i32,
///     #[serde(serialize_with = "naive_datetime_to_str")]
///     timestamp: NaiveDateTime,
/// }
/// ```
pub fn naive_datetime_to_str<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    return serializer.serialize_str(&datetime.format("%Y-%m-%d %H:%M:%S").to_string());
}

/// Serializes a string into NaiveDateTime
///
/// # Usage
///
/// ```rust,ignore
/// // Sample json record
/// pub struct Record {
///     id: i32,
///     #[serde(serialize_with = "str_to_naive_datetime")]
///     timestamp: String,
/// }
/// ```
pub fn str_to_naive_datetime<S>(s: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let datetime = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| serde::ser::Error::custom(e.to_string()))?;
    return serializer.serialize_str(&datetime.format("%Y-%m-%d %H:%M:%S").to_string());
}
