//! Serde implemetations.
//!
//! This implements are enable with `serde` feature.  
//! ```sh
//! viks = { version = "*", features = ["serde"] }
//! ```

impl<'de> serde::Deserialize<'de> for crate::Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        crate::Key::new(&s).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for crate::Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
