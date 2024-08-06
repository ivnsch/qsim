# QSim

Plots for basic quantum models:

- Infinite well
- Harmonic oscillator

Rendered with [bevy](https://bevyengine.org)

Note: doesn't solve differential equations, uses directly the solutions.

More models might be added.

```
cargo run
```

Web (not tested):

```
cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_example \
 --out-dir target \
 --target web target/wasm32-unknown-unknown/debug/qsim.wasm
python -m http.server 8888
```

## Contribute

1. Fork
2. Commit changes to a branch in your fork
3. Push your code and make a pull request
