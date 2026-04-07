# Rust Coding Guidelines for AI Agents

Distilled from the Microsoft RustTraining books. Apply these patterns when writing, reviewing, or refactoring Rust code in this project.

---

## Core Philosophy

- **Solve problems at the type level, not the test level.** The compiler checks every build — forever. Tests check specific cases; types check *all* cases.
- **Make invalid states unrepresentable.** If a bug can be a compile error, make it one.
- **Parse, don't validate.** Validate once at the boundary (`TryFrom`), then trust the type everywhere else.

---

## Type System Patterns

### Newtypes for Domain Primitives
Wrap primitives to prevent mixing unrelated values. Private fields enforce invariants.
```rust
struct Port(u16);       // not interchangeable with any u16
struct Email(String);   // not interchangeable with any String
struct Celsius(f64);    // not interchangeable with Fahrenheit

impl TryFrom<u16> for Port {
    type Error = &'static str;
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        if v > 0 { Ok(Port(v)) } else { Err("port must be > 0") }
    }
}
```

### Type-State Pattern for Protocols and Sessions
Encode valid transitions into the type system. Invalid transitions become compile errors.
```rust
struct Connection<State> { _state: PhantomData<State> }
struct Disconnected; struct Connected; struct Authenticated;

impl Connection<Disconnected> {
    fn connect(self) -> Result<Connection<Connected>, Error> { ... }
}
impl Connection<Connected> {
    fn authenticate(self, creds: &Credentials) -> Result<Connection<Authenticated>, Error> { ... }
}
impl Connection<Authenticated> {
    fn send(&mut self, data: &[u8]) -> Result<(), Error> { ... }
    // send() doesn't exist on Disconnected or Connected — compile error if misused
}
```

### Parse, Don't Validate
Validation happens exactly once in `TryFrom`. After that, the type is proof of validity.
```rust
struct RawData(Vec<u8>);    // unvalidated, cheap to construct
struct ValidData { /* private fields */ } // invariants upheld

impl TryFrom<RawData> for ValidData {
    type Error = ValidationError;
    fn try_from(raw: RawData) -> Result<Self, Self::Error> {
        // validate checksum, format, offsets — once, here only
    }
}

impl ValidData {
    fn field(&self) -> &str { /* no re-validation */ }
}
```

### Phantom Types for Dimensional Analysis
Zero runtime cost. Prevents unit mismatch bugs at compile time.
```rust
use std::marker::PhantomData;
struct Meters; struct Seconds;

struct Quantity<Unit>(f64, PhantomData<Unit>);

// Quantity<Meters> and Quantity<Seconds> are distinct types — can't be mixed
```

### Linear Types via Move Semantics
Non-`Clone`, non-`Copy` types can only be used once. Use for one-shot operations.
```rust
pub struct Nonce(u64); // NOT Clone, NOT Copy

fn encrypt(key: &Key, nonce: Nonce, data: &mut Vec<u8>) { ... }

// After encrypt(key, nonce, &mut data), nonce is consumed.
// Using nonce again is a compile error — nonce reuse is impossible.
```

---

## Error Handling

- **Use `Result<T, E>` everywhere.** Avoid `unwrap()` in library code; reserve it for tests or genuinely impossible cases.
- **Use `?` for propagation.** Every `?` is a visible, documented exit point — unlike exceptions.
- **Define typed errors.** Use `thiserror` for library errors; `anyhow` for application-level error aggregation.
- **Never use `panic!` for recoverable errors** at system boundaries.

```rust
fn process(path: &str) -> Result<Output, AppError> {
    let text = fs::read_to_string(path)?;          // exit point 1 — visible
    let data: Value = serde_json::from_str(&text)?; // exit point 2 — visible
    let validated = validate(&data)?;               // exit point 3 — visible
    Ok(transform(validated))
}
```

---

## Ownership and Borrowing

- **Prefer borrowing over cloning.** Clone only when ownership is genuinely needed.
- **Avoid `Rc<RefCell<T>>` unless necessary.** It shifts borrow checking to runtime and panics instead of compile errors.
- **Use `Cow<T>` to avoid allocation when modification is conditional.**
  ```rust
  fn normalize(s: &str) -> Cow<str> {
      if s.contains(' ') { Cow::Owned(s.replace(' ', "_")) }
      else { Cow::Borrowed(s) }
  }
  ```
- **Use `Weak<T>` to break reference cycles** in `Rc`/`Arc` graphs.

---

## Async / Concurrency

- **Never hold `std::sync::Mutex` across `.await`.** Use `tokio::sync::Mutex` for async-held locks.
- **Futures are lazy.** Nothing executes until polled. Do not assume work has started until `await`.
- **Keep business logic synchronous.** Add `async` only at I/O boundaries — avoid "function coloring" spreading through the whole codebase.
- **Use `JoinSet` for structured concurrency.** Guarantees task cleanup; avoids leaked background tasks.
- **Use `select!` with a shutdown channel for graceful shutdown** instead of abrupt task cancellation.
- **No async `Drop`.** Provide explicit `.close()` / `.shutdown()` methods for resources that need async cleanup.
- **Watch for cancellation hazards.** If a task holding a transaction is cancelled mid-flight, the transaction may be silently abandoned.

```rust
// Wrong — blocks executor thread
async fn bad() {
    let data = std::fs::read_to_string("file.txt").unwrap(); // synchronous I/O on async thread
}

// Correct
async fn good() -> Result<String, io::Error> {
    tokio::fs::read_to_string("file.txt").await
}
```

---

## Traits and Generics

- **Prefer generics over `dyn Trait` for hot paths.** Monomorphization is zero-cost; vtable dispatch is not.
- **Use `dyn Trait` when the set of types is open and performance is not critical** (e.g., plugin systems).
- **Don't use `Deref` for domain types.** It leaks the inner type's API and bypasses invariants.
- **Use `impl Trait` in return position (RPIT) for simple cases.** Use named generics when callers need to name the type.

---

## Memory and Performance

- **Avoid unnecessary heap allocation** in hot paths: prefer stack-allocated arrays, `SmallVec`, or pre-allocated buffers.
- **Use `#[inline]` sparingly.** Profile before annotating generic or hot functions.
- **For benchmarking, use Criterion.rs.** `Instant::now()` in a loop is not a reliable benchmark — it ignores CPU frequency scaling, branch prediction warmup, and optimizer elision.
- **Use `cargo flamegraph` or `perf` for profiling before optimizing.** Don't assume bottlenecks.
- **Compile-time constants over runtime constants.** Use `build.rs` or `const fn` to move work to compile time.

---

## Macros

- **Prefer generics and traits over macros.** Macros are harder to read, debug, and compose.
- **Use `macro_rules!` for syntactic repetition** that can't be expressed with generics.
- **Use `derive` macros** (`serde`, `thiserror`, `Debug`) rather than writing manual `impl` blocks.
- **Use procedural macros only when `macro_rules!` is insufficient.** They're full compiler plugins — powerful but complex.

---

## Build and Project Hygiene

- **In `build.rs`, always emit `cargo::rerun-if-changed`** to avoid rebuilding on every `cargo build`.
- **Never hardcode paths in `build.rs`** — use `std::env::var("CARGO_MANIFEST_DIR")` and env variables.
- **For reproducible builds, don't bake timestamps.** Respect `SOURCE_DATE_EPOCH`.
- **Use workspace `Cargo.toml`** for shared dependency versions across crates in a monorepo.
- **Run `cargo clippy --all-targets -- -D warnings` in CI.** Treat lints as errors.
- **Enable Miri in CI for unsafe code** to catch undefined behavior.

---

## Safety and Security

- **Minimize `unsafe` blocks.** Every `unsafe` block must have a comment explaining why it is sound.
- **Check all array indexing in hot loops** — prefer iterators over manual indexing to avoid off-by-one errors.
- **Never transmute between types** unless you can prove the representation is identical.
- **Validate all input at system boundaries** (network, file, FFI). Use the parse-don't-validate pattern.
- **Avoid `unwrap()` on user-controlled input.** It panics and can be used as a DoS vector.

---

## Reference Source

These guidelines are distilled from the [Microsoft RustTraining](https://github.com/microsoft/RustTraining) books:
- `async-book` — Async Rust: Futures to Production
- `c-cpp-book` — Rust for C/C++ Developers
- `csharp-book` — Rust for C# Developers
- `python-book` — Rust for Python Developers
- `engineering-book` — Production Rust Engineering
- `rust-patterns-book` — Rust Patterns
- `type-driven-correctness-book` — Type-Driven Correctness
