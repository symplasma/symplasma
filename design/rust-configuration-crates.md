# Rust Configuration Crates

What are the best Rust crates for implementing config for programs?

I'd like the following features:

- Supports KDL config lang. This is not essential, but is a strong preference.
- Supports a variety of popular config languages e.g. ron, toml, json, yaml, etc.
- Supports default values
- Supports generation of an example config for the user.

## Gemini 3 Answer

For a modern Rust project that requires KDL support, multiple formats, and high-quality user features like example generation, there isn't a single "silver bullet" crate that does everything natively. Instead, the best approach is to use a **layered configuration library** in combination with a **KDL-Serde bridge**.

Here are the best recommendations based on your specific requirements:

### 1. The Best All-Rounder: `confique`

`confique` is currently the most "user-friendly" crate for CLI tools because it treats the configuration file as a piece of documentation for your users.

- **KDL Support:** It does not support KDL out of the box (it supports TOML, YAML, JSON5), but because it is based on **Serde**, you can easily hook in KDL using the `serde-kdl2` crate.
- **Multi-format:** Excellent native support for TOML, YAML, and JSON5.
- **Default Values:** Supported directly in the struct definition using attributes: `#[config(default = 8080)]`.
- **Example Generation:** **This is its killer feature.** It can automatically generate a "template" config file (including your doc comments as headers) for the user.
- **Why choose it:** If "generating an example config" is a high priority, this is the best choice.

### 2. The Modern Powerhouse: `schematic`

`schematic` is a relatively new but very powerful crate designed for complex, modern applications (it powers the `moon` build system).

- **KDL Support:** Like `confique`, it is Serde-based. You can use it with a KDL parser.
- **Multi-format:** Supports **RON**, TOML, JSON, YAML, and even Pkl.
- **Default Values:** Powerful attribute-based defaults: `#[setting(default = 3000)]`.
- **Example Generation:** It focuses more on **Schema generation**. It can generate JSON Schemas or TypeScript types from your config, which is better for web-integrated tools or IDE autocomplete.
- **Why choose it:** It uses `miette` for "beautiful" error messages that point exactly to the line/column in the config file where the user made a mistake.

### 3. The "Standard" for Layering: `figment`

Created by the author of the Rocket web framework, `figment` is the go-to for "12-factor apps" that need to merge config files, environment variables, and CLI flags.

- **KDL Support:** Requires a third-party provider or `serde-kdl2`.
- **Multi-format:** Native support for TOML, JSON, and YAML.
- **Default Values:** Handled by merging a "default" Layer.
- **Example Generation:** No built-in template generator; you would typically just serialize a `Default` instance of your struct to the desired format string.
- **Why choose it:** If you have highly complex merging logic (e.g., "Development" vs "Production" profiles).

------------------------------------------------------------------------

### Key Supporting Crates

To make these work with KDL, you will likely need one of these "bridge" crates:

- **`serde-kdl2`**: This is the current best way to use KDL with Serde. It allows you to take any Serde-based crate (like the ones above) and make it understand KDL.
- **`knuffel`**: If you decide KDL is your *only* priority and don't care about the other formats, `knuffel` provides the best native KDL experience with great error messages, though it uses its own macros instead of Serde.

### Summary Recommendation

| If you want... | Use this crate | Why? |
|:----------------|:---------|:-------------------------------------------|
| **Example configs for users** | **`confique`** | Automatically turns doc comments into config file comments. |
| **The best error messages** | **`schematic`** | Built-in `miette` support makes user errors easy to fix. |
| **RON + many other formats** | **`schematic`** | One of the few major config crates with native RON support. |
| **Format-preserving KDL** | **`kdl`** | Use this if you need to *programmatically edit* the config without losing comments. |

**Pro-tip:** To support KDL alongside others in `confique` or `figment`, define your config struct with `#[derive(Serialize, Deserialize, Default)]`. You can then generate an example config for *any* format simply by doing:

``` rust
let default_config = MyConfig::default();
let example_kdl = serde_kdl2::to_string(&default_config)?;
// or
let example_toml = toml::to_string(&default_config)?;
```

---

- [‘confique’ search // Lib.rs](https://lib.rs/search?q=confique)
- [Clapfig — Rust config library // Lib.rs](https://lib.rs/crates/clapfig)
- [Confique — Rust config library // Lib.rs](https://lib.rs/crates/confique)
- [template in confique::toml - Rust](https://docs.rs/confique/latest/confique/toml/fn.template.html)
- [serde-kdl2 — Rust data encoding library // Lib.rs](https://lib.rs/crates/serde-kdl2)
- [serde\\_kdl2 - Rust](https://docs.rs/serde-kdl2/latest/serde_kdl2/)
- [Knuffel — Rust parser // Lib.rs](https://lib.rs/crates/knuffel)
- [KFL — Rust parser // Lib.rs](https://lib.rs/crates/kfl)
- [rinarakaki/kfl: KDL decoder/encoder and derive macros in Rust](https://github.com/rinarakaki/kfl)
- [Knurdy — Rust parser // Lib.rs](https://lib.rs/crates/knurdy)
- [kaydle — Rust data encoding library // Lib.rs](https://lib.rs/crates/kaydle)
- [#kdl // Lib.rs](https://lib.rs/keywords/kdl)
- [kdlfmt — command-line utility in Rust // Lib.rs](https://lib.rs/crates/kdlfmt)
- [KDL — Rust data encoding library // Lib.rs](https://lib.rs/crates/kdl)
- [knus — Rust parser // Lib.rs](https://lib.rs/crates/knus)
- [arborium-kdl — Rust parser // Lib.rs](https://lib.rs/crates/arborium-kdl)
- [kq — Rust utility // Lib.rs](https://lib.rs/crates/kq)
- [serde\\_kdl — Rust data encoding library // Lib.rs](https://lib.rs/crates/serde_kdl)
- [KDL2XML — Rust application // Lib.rs](https://lib.rs/crates/kdl2xml)
- [Doku — Rust library // Lib.rs](https://lib.rs/crates/doku)
