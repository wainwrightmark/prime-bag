# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.

[crates.io]: https://crates.io/crates/prime_bag

## v0.4 (unreleased)
- `Features` added `EMPTY` constant
- `Features` made some functions constant
- `Breaking Changes` renamed `into_prime_index` to `to_prime_index`

## v0.3 (2023-03-19)

- `Breaking Changes` - the default number of primes is now 32. This can be increased to 256 with the `primes256` feature
- Performance improvements

## v0.2 (2023-12-15)

- Added additional metadata to Cargo.toml
- Removed unsafe code
- Wrote more documentation

## v0.1.0 (2023-12-15)

- Initial Release on [crates.io] :tada:

[crates.io]: https://crates.io/crates/prime_bag
