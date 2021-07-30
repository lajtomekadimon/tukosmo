# Tukosmo (this is a work in progress!)
Modern CMS made with PostgreSQL and Rust.

## Instalation

Dependencies:

- PostgreSQL +13
- Rust 2018

Install dependencies (only FreeBSD for now):

```sh
make -s dep
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
