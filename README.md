# serve-env

This tool is a small web server that serves the machine's environment variables to the web. 

## Why?

Single page applications made by [create-react-app](https://create-react-app.dev/) uses env variables quite similar to NodeJS applications. However, here environment variables are set on build and cannot be changed dynamically after build. This project attempts to partially solve this problem by serving env variables to the client application. The client application then needs to call this server and set its response as it environment variables.

## Run locally

Prerequisites: [Rust and Cargo](https://www.rust-lang.org/tools/install)

```
# Clone repo
$ git clone https://github.com/kongand/serve-env.git

# Install dependencies
$ cd serve-env
$ cargo build

# Run
$ cargo run
```

If you want live reloads upon code changes (for development), then install ``cargo-watch``:

```
# Install cargo-watch
$ cargo install cargo-watch

# Run
$ cargo watch -x run
```
