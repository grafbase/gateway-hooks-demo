# HTTP and logging with gateway hooks

In this example we show how to run async HTTP requests and write logs from gateway hooks.

## The components of this example

- `authorized-subgraph` has a simple subgraph server, with a dumb endpoint we can call from the hooks
- `demo-hooks` contains the code for WebAssembly hooks as a Rust project
- `federated-schema.graphql` is the federated GraphQL schema
- `grafbase.toml` has the configuration for the Grafbase Gateway

## Dependencies

To run this example, you need the Grafbase Gateway version 0.4.0 or later, read more how to install it from:

https://grafbase.com/docs/self-hosted-gateway

Additionally, the following tools are needed:

- A C compiler, such as clang together with pkg-config (install based on your system, `cc` command is required)
- If on linux, cargo-component depends on openssl (`libssl-dev` on Debian)
- Rust compiler ([install docs](https://www.rust-lang.org/learn/get-started))
- Cargo component ([install docs](https://github.com/bytecodealliance/cargo-component?tab=readme-ov-file#installation))
- A GraphQL client, such as [Altair](https://altair-gql.sirmuel.design/)

For the advanced users using nix with flakes support:

```
nix develop
```

## Running the example

First, start the subgraph in one terminal:

```bash
cd authorized-subgraph
cargo run --release
```

Then, compile the WebAssembly hook functions into a Wasm component in another terminal:

```bash
cd demo-hooks
cargo component build --release
```

After a successful build, the component can be found from `target/wasm32-wasip1/release/demo_hooks.wasm`.
This file must exist for us to continue.

Finally start the `grafbase-gateway`:

```bash
grafbase-gateway --schema federated-schema.graphql --config grafbase.toml
```

Now open up the GraphQL client and start firing some queries.

## Example query

```graphql
query {
  getUser(id: 2) {
    id
    name
    address {
      street
    }
    secret {
      socialSecurityNumber
    }
  }
  getSecret(id: 1) {
    socialSecurityNumber
  }
}
```
