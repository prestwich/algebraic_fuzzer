pub mod traits;
pub mod algebraic_fuzzer;

#[cfg(all(feature = "eip2537", not(feature = "eip2539")))]
pub mod eip_2537_fuzzer;

#[cfg(all(feature = "eip2539", not(feature = "eip2537")))]
pub mod eip_2539_fuzzer;
