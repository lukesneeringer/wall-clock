//! Serialization to/from PostgreSQL

use diesel::deserialize::FromSql;
use diesel::deserialize::Result as DeserializeResult;
use diesel::pg::data_types::PgTime;
use diesel::pg::Pg;
use diesel::pg::PgValue;
use diesel::serialize::Output;
use diesel::serialize::Result as SerializeResult;
use diesel::serialize::ToSql;
use diesel::sql_types;

use crate::WallClockTime;

impl ToSql<sql_types::Time, Pg> for WallClockTime {
  fn to_sql<'se>(&'se self, out: &mut Output<'se, '_, Pg>) -> SerializeResult {
    let micros = self.seconds as i64 * 1_000_000 + self.micros as i64;
    ToSql::<sql_types::Time, Pg>::to_sql(&PgTime(micros), &mut out.reborrow())
  }
}

impl FromSql<sql_types::Time, Pg> for WallClockTime {
  fn from_sql(bytes: PgValue<'_>) -> DeserializeResult<Self> {
    let PgTime(offset) = FromSql::<diesel::sql_types::Time, Pg>::from_sql(bytes)?;
    Ok(Self { seconds: (offset / 1_000_000) as u32, micros: (offset % 1_000_000) as u32 })
  }
}
