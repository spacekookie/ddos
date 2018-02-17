# DDOS

DDOS (**D**ynamic **Do**main **S**erver) is a dynamic DNS client which is configurable in toml files or via a RESTful API. It is a very minimal dynamic DNS setup where a home-server can report it's IP address to a DNS in the cloud which will route all future requests to said IP.

It is written in Rust and uses a very minimal DNS server in the backend, written in C. This program has not been security tested and shouldn't be used on systems that are critical to work (aka production!)

## How to use

```
git clone https://github.com/spacekookie/ddos
cd ddos/
```

You will need to have a Rust 1.x toolchain installed to build ddos and `cmake` and a modern `gcc` (version 5.0+). You can adjust the `ddos.toml` configuration file to your liking before launching the server. You will also have to generate a secret key into the `auth` directory. Recommended is you use sha256 to generate some hash and save it in a file. The name will be the username, the contents the key. Use this key when authenticating via the RESTful API.

Compile the application with `cargo build --release` then run it via `target/release/ddos start` and you're all set 😊
