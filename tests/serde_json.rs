use serde;
use serde_json;

#[cfg(feature = "chrono")]
mod chrono {
    use chrono::Duration;
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct Payload {
        #[serde(with = "serde_nanos")]
        duration: Duration,
    }

    #[test]
    fn zero_nanosecond_duration_from_str() {
        let actual = serde_json::from_str::<Payload>(r#"{ "duration": 0 }"#).unwrap();
        let expected = Payload {
            duration: Duration::nanoseconds(0),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn one_second_duration_from_str() {
        let actual = serde_json::from_str::<Payload>(r#"{ "duration": 1000000000  }"#).unwrap();
        let expected = Payload {
            duration: Duration::seconds(1),
        };

        assert_eq!(actual, expected);
    }
}
