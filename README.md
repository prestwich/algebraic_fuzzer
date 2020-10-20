### Algebraic Fuzzer for BLS

Fuzz BLS12-381
`cargo run`

Fuzz BLS12-377:
`cargo run --bin eip2539 --no-default-features --features eip2539

TODO: 
- implement `sqrt_for_one_mod_four` in `eip1962`
- re-enable invalid subgroup fuzzing