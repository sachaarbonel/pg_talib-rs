use pgx::*;
use talib::Macd;

pg_module_magic!();

#[pg_extern]
fn hello_talib_ext() -> &'static str {
    "Hello, talib_ext"
}

#[pg_extern]
fn macd(
    inReal: Vec<f64>,
    inFastPeriod: i64,
    inSlowPeriod: i64,
    inSignalPeriod: i64,
) -> (
    name!(macd, Vec<f64>),
    name!(macdsignal, Vec<f64>),
    name!(macdhist, Vec<f64>),
) {
   Macd(&inReal, inFastPeriod, inSlowPeriod, inSignalPeriod)
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_talib_ext() {
        assert_eq!("Hello, talib_ext", crate::hello_talib_ext());
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
