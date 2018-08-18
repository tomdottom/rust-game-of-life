## Background

https://rustwasm.github.io/book/


## Setup

### Install Rust and dependencies

```
rustup update
rustup install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

```
cargo +nightly install wasm-bindgen-cli
```


## Build Projects
```
./build.sh
```

## Run Projects

*rust binary executable*
```
./dist/rust/cgol
```

*Python app using rust compiled python package*
```
python ./dist/python/main.py
```
