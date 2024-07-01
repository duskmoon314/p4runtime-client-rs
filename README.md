# p4runtime-client-rs

A P4Runtime client wrapper in Rust, highly inspired by [p4runtime-go-client](https://github.com/antoninbas/p4runtime-go-client).

This crate is based on the generated code in [duskmoon314/p4runtime:rust](https://github.com/duskmoon314/p4runtime/tree/rust).
Hopefully, it will be merged into the main repository in the future.

## Usage

See [examples/basic](examples/basic/README.md) for a basic example.

## Features

- [x] Basic Read and Write
- [x] Table Operations
- [x] Counter Operations
- [x] Digest Operations
- [ ] Action Profile Operations
- [ ] Meter Operations
- [ ] Register Operations
- [ ] Value Set Operations
- [ ] Direct Counter Operations
- [ ] Direct Meter Operations
- [ ] Direct Register Operations
- [ ] Helper features
  - [ ] DigestList Conversion
  - [ ] PipelineConfig builder
    - [x] `build_tofino_config`

## License

All codes in this repository, unless otherwise noted, are licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.
