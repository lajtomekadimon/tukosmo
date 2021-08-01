# Tukosmo (this is a work in progress!)
Website generator made with Rust and PostgreSQL.

This is my first Rust project. I'm still learning the language,
so don't expect high quality code (yet).

## TODO

Technical aspects:

- [x] Zero unsafe code.
- [x] No ORM! Pure SQL code called by native PostgreSQL driver.
- [x] Blazingly fast HTML rendering with [markup.rs](https://github.com/utkarshkukreti/markup.rs).
- [x] Automatic CSS minifying to one single file.
- [ ] Automatic JavaScript minifying to one single file.
- [x] Common HTML to all themes; a Tukosmo theme is just CSS code!
- [x] i18n implemented in routes (good for SEO).

Website features:

- [ ] Homepage.
- [ ] Blog.
- [ ] Pages.
- [ ] Admin panel.

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
