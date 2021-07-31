# Tukosmo (this is a work in progress!)
Website generator made with Rust and PostgreSQL.

This is my first Rust project. I'm still learning the language,
so don't expect high quality code (yet).

## Instalation

Dependencies:

- PostgreSQL +13
- Rust 2018

Install dependencies (only FreeBSD for now):

```sh
make -s dep
# The shell must be restarted after installing Rust
```

Create (or reset) database:

```sh
make -s resetdb
```

Install web server:

```sh
make -s install
```

## Run server

Run web server:

```sh
make -s run
```
