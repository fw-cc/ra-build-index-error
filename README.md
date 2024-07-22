### Example of rust-analyzer failing to recognise/index build artifacts after #17246

This example repository generates rust code to represent the protocol buffer
data structure defined in `src/protos/example.proto`.
Since the most recent update to rust-analyzer type recognition of these
structures has failed with no noticeable errors beyond annotations and
suggestions being missing.

### Verification

This should be as simple as `cargo run` to verify the generated code functions
at compile time.
Once these files have been generated I would expect rust-analyzer to index them
and produce type hints in the IDE.

Please see included screenshot of how this displays on VSCode.
