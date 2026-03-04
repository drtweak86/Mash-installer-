# CI Debugging Guide for MASH Installer

## Common CI Issues and Solutions

### 1. Docker Image Build Failures

**Symptoms:**
- `cargo fetch` fails with "No such file or directory"
- Docker build fails during dependency fetching

**Solutions:**
- Ensure Dockerfile has proper `apt-get update` before installing packages
- Add `libsqlite3-dev` to Dockerfile dependencies
- Use multi-stage builds to reduce image size

### 2. Code Coverage Failures

**Symptoms:**
- Tarpaulin fails with "Test failed during run"
- Coverage report generation times out

**Solutions:**
- Increase timeout in `tarpaulin.toml` (300 seconds recommended)
- Use XML output format for Codecov compatibility
- Add ignore patterns for files that don't need coverage
- Run with `--verbose` flag for debugging

### 3. Ubuntu Distro Test Failures

**Symptoms:**
- `libsqlite3.so.0: cannot open shared object file`
- Binary fails to run in Ubuntu container

**Solutions:**
- Install `libsqlite3-0` in the test container
- Use `docker run` with proper volume mounting
- Ensure all runtime dependencies are available

### 4. aarch64 Cross-Compilation Failures

**Symptoms:**
- `unable to find dynamic system library 'sqlite3'`
- Linker errors during cross-compilation

**Solutions:**
- Install `gcc-aarch64-linux-gnu` and related tools
- Set up proper library paths for cross-compilation
- Use `cargo-zigbuild` or similar tools for better cross-compilation support

## CI Workflow Best Practices

1. **Dependency Caching:** Use `Swatinem/rust-cache` for faster builds
2. **Multi-stage Builds:** Reduce final image size
3. **Error Handling:** Add proper error handling in CI scripts
4. **Timeout Management:** Set appropriate timeouts for long-running jobs
5. **Artifact Management:** Clean up artifacts between jobs

## Debugging Commands

```bash
# Test Docker build locally
docker build -t mash-installer .

# Test cross-compilation
cargo build --target aarch64-unknown-linux-gnu

# Run coverage locally (requires cargo-tarpaulin)
cargo tarpaulin --all-features --out Xml

# Test in Ubuntu container
docker run --rm -v "$PWD/target/release/mash-setup:/usr/local/bin/mash-setup" ubuntu:24.04 bash -c "apt update && apt install -y libsqlite3-0 && mash-setup --version"
```

## Troubleshooting Checklist

- [ ] Check Dockerfile for missing dependencies
- [ ] Verify tarpaulin configuration
- [ ] Ensure cross-compilation tools are installed
- [ ] Test runtime dependencies in containers
- [ ] Review CI logs for specific error messages
- [ ] Check GitHub Actions runner environment
- [ ] Validate cargo configuration and workspace setup
