#[cfg(feature = "chrono")]
use chrono;

use serde::Deserializer;

/// Types that can be deserialized via `#[serde(with = "serde_nanos")]`.
pub trait Deserialize<'de>: Sized {
    #[allow(missing_docs)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

impl<'de> Deserialize<'de> for std::time::Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde::Deserialize::deserialize(deserializer)
            .map(|nanos| std::time::Duration::from_nanos(nanos))
    }
}

#[cfg(feature = "chrono")]
impl<'de> Deserialize<'de> for chrono::Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde::Deserialize::deserialize(deserializer)
            .map(|nanos| chrono::Duration::nanoseconds(nanos))
    }
}
