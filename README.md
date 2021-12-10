```
                     _
 _ __ ___   ___ _ __(_)_ __   ___
| '_ ` _ \ / _ \ '__| | '_ \ / _ \
| | | | | |  __/ |  | | | | | (_) |
|_| |_| |_|\___|_|  |_|_| |_|\___/
```

**A `SOCKS5` Proxy server written in Rust**

[![Crates.io](https://img.shields.io/crates/v/merino.svg)](https://crates.io/crates/merino)
[![stego](https://docs.rs/merino/badge.svg)](https://docs.rs/merino)
[![License](https://img.shields.io/crates/l/pbr.svg)](https://github.com/ajmwagar/merino/blob/master/LICENSE.md)
[![Build Status](https://travis-ci.org/ajmwagar/merino.svg?branch=master)](https://travis-ci.org/ajmwagar/merino)
[![dependency status](https://deps.rs/repo/github/ajmwagar/merino/status.svg)](https://deps.rs/repo/github/ajmwagar/merino)

## 🎁 Features

- Written in **100% Safe Rust**
- Multi-threaded connection handler
- Lightweight (Less than 0.6% CPU usage while surfing the web/streaming YouTube)
- Standalone binary (no system dependencies)
- `1+ Gb/second` connection speeds (**On Gigabit LAN network over ethernet. Results may vary!**)
- Tunable logging (try `export RUST_LOG=merino=DEBUG`)
- `SOCKS5` Compatible Authentication methods:
  - `NoAuth`
  - Username & Password
  - `GSSAPI` Coming Soon!

## 📦 Installation & 🏃 Usage

### Installation

```bash
cargo install merino
```

OR

```bash
git clone https://github.com/ajmwagar/merino
cd merino
cargo install --path .
```

### Usage

```bash
# Start a SOCKS5 Proxy server listening on port 1080 without authentication
merino --no-auth

# Use username/password authentication and read users from users.csv
merino --users users.csv

# Display a help menu
merino --help 
```

### Docker
# build
`docker build -t rust-merino-socks5proxy:latest .`
# run detached; there is no way to exit once running unless you kill the shell
`docker run -d --name rm-s5 rust-merino-socks5proxy:latest merino --no-auth`
# get logs
`
                      _
  _ __ ___   ___ _ __(_)_ __   ___
 | '_ ` _ \ / _ \ '__| | '_ \ / _ \
 | | | | | |  __/ |  | | | | | (_) |
 |_| |_| |_|\___|_|  |_|_| |_|\___/

 A SOCKS5 Proxy server written in Rust

 2021-12-10T07:43:10.253 INFO  merino > Listening on 0.0.0.0:1080
 2021-12-10T07:43:10.253 INFO  merino > Serving Connections...`

# get shell
`docker exec -it rm-s5 bash`


# 🚥 Roadmap

- [x] IPV6 Support
- [ ] `SOCKS5` Authentication Methods
  - [x] `NOAUTH` 
  - [x] `USERPASS`
  - [ ] `GSSAPI` Coming Soon!
- [ ] Custom plugin/middleware support
- [ ] `SOCKS5` Commands
  - [x] `CONNECT`
  - [ ] `BIND`
  - [ ] `ASSOCIATE` 
- [ ] Benchmarks & Unit tests
- [ ] [Actix](https://github.com/actix-rs/actix) based backend
- [ ] `SOCKS4`/`SOCKS4a` Support
