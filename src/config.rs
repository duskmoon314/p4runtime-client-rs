//! Helper methods for building P4 device config

/// Build a Tofino config
///
/// Tofino's config is formed by concatenating the following:
/// 1. 4-byte little-endian length of the program name
/// 2. Program name in ASCII string
/// 3. 4-byte little-endian length of the Tofino binary
/// 4. Tofino binary (tofino.bin)
/// 5. 4-byte little-endian length of the context JSON
/// 6. Context JSON (context.json)
///
/// # Arguments
///
/// - program_name: Name of the program. It can be chosen freely.
/// - tofino_bin: Tofino binary (tofino.bin)
/// - context_json: Context JSON (context.json)
///
/// # Example
///
/// Here is a really simple example to show how this function works:
///
/// ```rust
/// # use p4runtime_client::config::build_tofino_config;
/// let program_name = "my_program";
/// let tofino_bin = vec![0x00, 0x01, 0x02, 0x03];
/// let context_json = vec![0x04, 0x05, 0x06, 0x07];
///
/// let config = build_tofino_config(program_name, tofino_bin, context_json);
/// assert_eq!(config, vec![
///     0x0A, 0x00, 0x00, 0x00, 0x6D, 0x79, 0x5F, 0x70, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D,
///     0x04, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03,
///     0x04, 0x00, 0x00, 0x00, 0x04, 0x05, 0x06, 0x07,
/// ]);
/// ```
///
pub fn build_tofino_config(
    program_name: &str,
    tofino_bin: impl AsRef<[u8]>,
    context_json: impl AsRef<[u8]>,
) -> Vec<u8> {
    let mut config = Vec::new();

    config.extend_from_slice(&(program_name.len() as u32).to_le_bytes());
    config.extend_from_slice(program_name.as_bytes());

    config.extend_from_slice(&(tofino_bin.as_ref().len() as u32).to_le_bytes());
    config.extend_from_slice(tofino_bin.as_ref());

    config.extend_from_slice(&(context_json.as_ref().len() as u32).to_le_bytes());
    config.extend_from_slice(context_json.as_ref());

    config
}
