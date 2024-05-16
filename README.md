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