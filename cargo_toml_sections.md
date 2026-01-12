# Sections for the cargo.toml file

## `[package]`

Defines **metadata about a single Rust crate** (package).

It answers: *What is this crate?*

Common fields:

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"
authors = ["Alice <alice@example.com>"]
description = "An example Rust project"
license = "MIT"
```

Key points:

* Required for **any publishable crate**
* Identifies the crate’s name, version, Rust edition, etc.
* One `Cargo.toml` can only have **one `[package]` section**

---

## `[dependencies]`

Lists **external crates that your package depends on**.

It answers: *What other code does this crate need?*

Example:

```toml
[dependencies]
serde = "1.0"
rand = "0.8"
tokio = { version = "1.35", features = ["full"] }
```

Key points:

* Each entry is a crate from **crates.io** (or git/path sources)
* Versions follow **semantic versioning**
* Cargo automatically downloads, builds, and links them
* Variants include:

  * `[dev-dependencies]` → used only for tests/examples
  * `[build-dependencies]` → used by build scripts

---

## `[workspace]`

Defines a **workspace**, which is a collection of related crates managed together.

It answers: *How are multiple crates organized and built together?*

Example:

```toml
[workspace]
members = [
    "crates/api",
    "crates/core",
    "crates/cli"
]
```

Key points:

* Used in **monorepos** or multi-crate projects
* Shared:

  * `Cargo.lock`
  * target directory
* Each member has its **own `Cargo.toml`** (usually with `[package]`)
* The workspace root usually **does NOT** have a `[package]` section

You can also centralize dependency versions:

```toml
[workspace.dependencies]
serde = "1.0"
```

---

## How they relate

| Section          | Purpose                | Scope               |
| ---------------- | ---------------------- | ------------------- |
| `[package]`      | Describes one crate    | Single crate        |
| `[dependencies]` | External crates used   | Single crate        |
| `[workspace]`    | Groups multiple crates | Multi-crate project |

---

### Quick mental model

* **`[package]`** → “Who am I?”
* **`[dependencies]`** → “What do I need?”
* **`[workspace]`** → “Who do I work with?”



* **`[dependencies]`** – runtime dependencies
* **`[dev-dependencies]`** – used for tests, examples, benchmarks
* **`[build-dependencies]`** – used by `build.rs`

Example:

```toml
[dev-dependencies]
criterion = "0.5"
```

---

## Build & target configuration

### `[lib]`

Configures the library target.

```toml
[lib]
name = "my_lib"
crate-type = ["cdylib"]
```

Used when:

* Building a library instead of (or in addition to) a binary
* Controlling output type (`rlib`, `cdylib`, `staticlib`)

---

### `[[bin]]`

Defines one or more binary targets.

```toml
[[bin]]
name = "my_app"
path = "src/main.rs"
```

Useful for:

* Multiple binaries in one crate
* Custom binary paths

---

### `[[example]]`, `[[test]]`, `[[bench]]`

Define examples, tests, and benchmarks explicitly.

```toml
[[example]]
name = "demo"
path = "examples/demo.rs"
```

---

## Features & conditional compilation

### `[features]`

Defines **optional features** for your crate.

```toml
[features]
default = ["json"]
json = ["serde", "serde_json"]
```

Used for:

* Optional functionality
* Enabling/disabling dependencies
* Reducing compile times

---

## Workspace-related

### `[workspace]`

Defines workspace members.

---

### `[workspace.dependencies]`

Centralizes dependency versions across workspace crates.

```toml
[workspace.dependencies]
serde = "1.0"
```

---

## Profiles (build behavior)

### `[profile.dev]`, `[profile.release]`

Controls compiler optimization and debug settings.

```toml
[profile.release]
opt-level = 3
lto = true
```

Common profiles:

* `dev`
* `release`
* `test`
* `bench`

---

## Build scripts

### `[package.build]`

Specifies a build script.

```toml
[package]
build = "build.rs"
```

Used when:

* Linking native libraries
* Code generation
* Platform-specific logic

---

## Platform-specific sections

### Target-specific dependencies

```toml
[target.'cfg(windows)'.dependencies]
winapi = "0.3"
```

Used for:

* OS-specific code
* Architecture-specific dependencies

---

## Publishing & metadata

### `[package.metadata]`

Free-form metadata for tools.

```toml
[package.metadata.docs.rs]
all-features = true
```

Cargo ignores this; external tools read it.

---

### `[badges]`

Shows status badges on crates.io.

```toml
[badges]
travis-ci = { repository = "user/repo" }
```

---

## Patching & overriding dependencies (advanced)

### `[patch]`

Override dependencies globally.

```toml
[patch.crates-io]
serde = { path = "../serde" }
```

Used for:

* Local development
* Forked crates

---

### `[replace]` (deprecated)

Older override mechanism (avoid in new projects).

---

## Linting & tooling (newer Rust)

### `[lints]`

Configure compiler lints at the package level.

```toml
[lints.rust]
unsafe_code = "forbid"
```

---

## Documentation

### `[package.metadata.docs.rs]`

Docs.rs-specific configuration.

```toml
[package.metadata.docs.rs]
features = ["full"]
```

---

## Summary table

| Section                | Purpose                  |
| ---------------------- | ------------------------ |
| `[package]`            | Crate metadata           |
| `[dependencies]`       | Runtime deps             |
| `[dev-dependencies]`   | Test/example deps        |
| `[build-dependencies]` | Build script deps        |
| `[lib]`                | Library config           |
| `[[bin]]`              | Binary targets           |
| `[features]`           | Optional functionality   |
| `[workspace]`          | Multi-crate setup        |
| `[profile.*]`          | Build optimization       |
| `[target.*]`           | Platform-specific config |
| `[patch]`              | Dependency overrides     |
| `[package.metadata]`   | Tool-specific metadata   |

---

### Rule of thumb

* **Small project** → `[package]`, `[dependencies]`
* **Library** → add `[features]`, `[lib]`
* **Multi-crate repo** → add `[workspace]`
* **Production builds** → tune `[profile.release]`




### Best practices (hard-earned wisdom)

✅ Prefer crates.io whenever possible

✅ Use path dependencies inside workspaces

⚠️ Avoid long-term git dependencies

🔒 Commit Cargo.lock for applications

📦 Don’t commit Cargo.lock for libraries



### Mental model

- crates.io → “Stable and published”

- path → “Local and editable”

- git → “Remote but unstable”