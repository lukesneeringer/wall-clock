use std::fmt;

use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

use crate::WallClockTime;

impl Serialize for WallClockTime {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_str(&self.to_string())
  }
}

struct WallClockTimeVisitor;

impl<'de> Visitor<'de> for WallClockTimeVisitor {
  type Value = WallClockTime;

  #[cfg(not(tarpaulin_include))]
  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("a HH:MM:SS WallClockTime string")
  }

  fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
    s.parse().map_err(E::custom)
  }
}

impl<'de> Deserialize<'de> for WallClockTime {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    deserializer.deserialize_str(WallClockTimeVisitor)
  }
}

#[cfg(test)]
mod tests {
  use serde_test::assert_tokens;
  use serde_test::Token;

  use crate::time;
  use crate::WallClockTime;

  #[test]
  fn test_serde() {
    assert_tokens(&time!(09:30:00), &[Token::Str("09:30:00")]);
    assert_tokens(&WallClockTime::new_with_micros(15, 30, 45, 123_456), &[Token::Str(
      "15:30:45.123456",
    )]);
  }
}
