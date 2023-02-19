# Rust web slideshow

A comparison of Rust web app libraries using a simple slideshow as an example.


## Backend

To start the backend use the IntelliJ run configuration or `cargo run --bin backend`.

## Sdk

The glue to make sure frontend and backend are compatible.


## Libraries


### Dioxus (dye•ox•us)

[Website](https://dioxuslabs.com/)

[dioxus-app](https://github.com/patrick-dedication/rust-web-slideshow/tree/main/dioxus-app)

#### Pros

- typesafe
- very similar to [React](https://reactjs.org/)
- great [Guide](https://dioxuslabs.com/docs/0.3/guide/en/index.html)
- very little magic, leverages Rust i.e. `UseState` is just a smart `Rc`
- ergonomic macros i.e. `#[inline_props]`
- dedicated batteries included [cli](https://github.com/DioxusLabs/cli) (`dioxus serve`)
- hot reloading
- cli (`dioxus fmt`) to format code in macros 
- highly performant and lightweight 
- one codebase, every platform
- large community


#### Cons

- not fully mature yet
- heavy reliance on Procedural Macros
