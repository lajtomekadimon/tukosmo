# Tukosmo (this is a work in progress!)
Website generator made with Rust and PostgreSQL.

This is my first Rust project. I'm still learning the language,
so don't expect high quality code (yet).

## TODO

[x] Zero unsafe code.
[x] No ORM! Pure SQL code run by PostgreSQL driver.
[x] Blazingly fast HTML rendering with markup.rs.
[x] Automatic CSS minifying system.
[ ] Automatic JavaScript minifying system.
[ ] Common HTML to all themes. A new theme is a single CSS file!
[ ] Blog system.
[ ] Pages system.
[ ] Contact form.
[ ] Admin panel.

## Instalation

Dependencies:

- PostgreSQL +13
- Rust 2018

Install dependencies (only on FreeBSD for now):

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
