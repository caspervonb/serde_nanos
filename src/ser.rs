#[cfg(feature = "chrono")]
use chrono;

use serde::ser::Error;
use serde::Serializer;

/// Types that can be serialized via `#[serde(with = "serde_nanos")]`.
pub trait Serialize {
    #[allow(missing_docs)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

impl Serialize for std::time::Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let nanoseconds = self.as_nanos() as i64;
        serializer.serialize_i64(nanoseconds)
    }
}

#[cfg(feature = "chrono")]
impl Serialize for chrono::Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let nanoseconds = self.num_nanoseconds().unwrap();
        serializer.serialize_i64(nanoseconds)
    }
}
