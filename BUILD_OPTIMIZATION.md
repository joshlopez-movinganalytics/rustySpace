# Build Time Optimization Guide

This project is now optimized for **fast iteration during development** and **optimal performance for release builds**.

## 🚀 Quick Start

### For Development (FASTEST - Recommended for iteration)
```bash
./run-dev.sh          # Build and run in dev mode with dynamic linking
# OR
cargo run             # Same as above
```

**First build:** ~5-6 minutes  
**Incremental builds:** **~5-20 seconds** ⚡ (HUGE improvement!)

### For Testing Performance
```bash
cargo run --profile release-fast
```

**Build time:** ~3-4 minutes  
**Performance:** Near-release quality

### For Final Release
```bash
./build.sh           # Full optimizations
# OR
cargo build --release
```

**Build time:** ~5-6 minutes  
**Performance:** Fully optimized

---

## 🔧 Optimizations Applied

### 1. **Dynamic Linking (BIGGEST SPEEDUP)**
- **What:** Bevy compiles to a shared library (.dylib) instead of static linking
- **Benefit:** Incremental builds only recompile YOUR code, not Bevy
- **Trade-off:** Dev builds are slightly slower to run (but MUCH faster to compile)
- **When:** Automatically used in `cargo run` and dev builds

### 2. **Fast Linker Configuration**
- **Location:** `.cargo/config.toml`
- **What:** Uses native linker optimized for macOS
- **Optional:** Install `zld` for even faster linking:
  ```bash
  brew install michaeleisel/zld/zld
  ```
  Then uncomment the zld section in `.cargo/config.toml`

### 3. **Profile Optimization**
- **`dev`**: Fast compilation, optimized dependencies
- **`release-fast`**: Quick release builds with good performance
- **`release`**: Fully optimized for final distribution

### 4. **Parallel Compilation**
- Uses 8 parallel jobs for faster builds
- Adjust in `.cargo/config.toml` if needed

---

## 📊 Build Time Comparison

| Build Type | First Build | Incremental | Performance | Use Case |
|------------|-------------|-------------|-------------|----------|
| **Dev (dynamic)** | ~5 min | **5-20 sec** ⚡ | Good | Daily development |
| **Release-fast** | ~3 min | ~1-2 min | Great | Testing performance |
| **Release** | ~5 min | ~3-4 min | Best | Final builds |

---

## 💡 Tips for Even Faster Builds

### 1. Use `cargo-watch` for automatic rebuilds
```bash
cargo install cargo-watch
cargo watch -x run
```

### 2. Use `sccache` for caching compiled dependencies
```bash
brew install sccache
export RUSTC_WRAPPER=sccache
```

### 3. Increase parallel jobs (if you have lots of RAM)
Edit `.cargo/config.toml`:
```toml
[build]
jobs = 12  # Increase if you have 16+ GB RAM
```

### 4. Only rebuild what changed
```bash
cargo run  # Instead of cargo clean && cargo build
```

### 5. Use `cargo-nextest` for faster test runs
```bash
cargo install cargo-nextest
cargo nextest run
```

---

## 🛠️ Advanced: Install Faster Linker (Optional)

### macOS - zld
```bash
brew install michaeleisel/zld/zld
```
Then uncomment the zld lines in `.cargo/config.toml`

**Expected speedup:** 20-50% faster linking

### macOS - mold (alternative)
```bash
brew install mold
```

---

## 📝 Development Workflow

### Recommended Workflow:
1. **Start:** `./run-dev.sh` - First build takes ~5 min
2. **Code changes:** Press Ctrl+C, make changes, `./run-dev.sh` again
3. **Rebuild:** Only **5-20 seconds** for incremental builds! ⚡
4. **Test performance:** `cargo run --profile release-fast`
5. **Final build:** `./build.sh`

### With cargo-watch (even better):
```bash
cargo watch -x run
```
- Automatically rebuilds on file changes
- Just save and test immediately!

---

## ⚠️ Important Notes

1. **Dynamic linking is for development only**
   - Release builds don't use it
   - Don't distribute dev builds to players

2. **First build is still slow**
   - This is normal - Bevy is a large framework
   - Subsequent builds are MUCH faster

3. **Clean builds reset the cache**
   - Avoid `cargo clean` unless necessary
   - Use `cargo build` for incremental builds

4. **Memory usage**
   - Dynamic linking uses more RAM during development
   - This is normal and worth the speed trade-off

---

## 🎯 Recommended Setup

For the best development experience:

```bash
# 1. Install faster linker (optional but recommended)
brew install michaeleisel/zld/zld

# 2. Install cargo-watch for auto-rebuild
cargo install cargo-watch

# 3. Start development with auto-reload
cargo watch -x run

# 4. Make changes and save - it rebuilds automatically!
```

**Result:** Change code → Save → Wait 5-20 seconds → Game restarts! 🎉

---

## 🐛 Troubleshooting

### "dyld: Library not loaded" error
- This happens if you move the executable without the .dylib files
- **Solution:** Run from project root or use release builds for distribution

### Builds still slow after changes
- Make sure you're using `cargo run` (not `cargo build --release`)
- Check that `Cargo.toml` has `features = ["dynamic_linking"]` for bevy
- Don't run `cargo clean` between builds

### Out of memory during compilation
- Reduce `jobs` in `.cargo/config.toml`
- Close other applications
- Use `cargo build` instead of parallel watch tools

---

## 📚 Additional Resources

- [Bevy Fast Compile Guide](https://bevyengine.org/learn/book/getting-started/setup/#enable-fast-compiles-optional)
- [Rust Compilation Performance](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [cargo-watch](https://github.com/watchexec/cargo-watch)

---

**Enjoy your 10x faster build times!** ⚡🚀

