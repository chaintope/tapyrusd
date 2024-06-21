#[cfg(feature = "0_5_2")]
pub const VERSION: &str = "0.5.2";

#[cfg(all(feature = "0_5_1", not(feature = "0_5_2")))]
pub const VERSION: &str = "0.5.1";
