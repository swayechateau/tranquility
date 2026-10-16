# 1. Ensure all tests pass
cargo test --all

# 2. Check for warnings
cargo clippy --all --all-targets

# 3. Verify dry-run publish
cargo publish --dry-run

# 4. Check documentation builds
cargo doc --all --no-deps --open

# 5. Final verification
cargo tree