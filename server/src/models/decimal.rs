use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Numeric;
use rust_decimal::Decimal;
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Wrapper type for rust_decimal::Decimal that implements Diesel traits
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Numeric)]
pub struct PgDecimal(pub Decimal);

impl Deref for PgDecimal {
    type Target = Decimal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PgDecimal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Decimal> for PgDecimal {
    fn from(d: Decimal) -> Self {
        PgDecimal(d)
    }
}

impl From<PgDecimal> for Decimal {
    fn from(d: PgDecimal) -> Self {
        d.0
    }
}

// Implement the ToSql trait to allow PgDecimal to be stored in the database
impl ToSql<Numeric, Pg> for PgDecimal {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let bytes = self.0.serialize();
        out.write_all(&bytes)?;
        Ok(IsNull::No)
    }
}

// Implement the FromSql trait to allow PgDecimal to be loaded from the database
impl FromSql<Numeric, Pg> for PgDecimal {
    fn from_sql(bytes: diesel::pg::PgValue) -> deserialize::Result<Self> {
        let bytes_slice = bytes.as_bytes();
        if let Ok(str_val) = std::str::from_utf8(bytes_slice) {
            if let Ok(decimal) = Decimal::from_str(str_val) {
                return Ok(PgDecimal(decimal));
            }
        }
        
        Err("Failed to parse Decimal from database value".into())
    }
} 