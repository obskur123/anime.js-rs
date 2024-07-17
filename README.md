# animejs-rs

[![Crates.io](https://img.shields.io/crates/v/animejs-rs.svg)](https://crates.io/crates/animejs-rs)
[![Documentation](https://docs.rs/animejs-rs/badge.svg)](https://docs.rs/animejs-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

anime.js bindings para Rust, permitiendo el uso de la popular biblioteca de animación JavaScript en proyectos Rust WebAssembly.

## Características

- Integración de anime.js con Rust y WebAssembly
- Conversión de `HashMap` de Rust a opciones de anime.js
- Soporte para valores de tipo string y número en las opciones de animación

## Instalación

Añade esto a tu `Cargo.toml`:

```toml
[dependencies]
animejs-rs = "0.1.0"
```