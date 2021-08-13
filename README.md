<div align="center">

![Tukosmo](./static/faviconadmin/favicon-96x96.png)

**Tukosmo** (WIP!): Website generator made with Rust and PostgreSQL.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/lajtomekadimon/tukosmo/blob/main/LICENSE)

</div>

NOTE: This is my first Rust project. I'm still learning the language, so
don't expect high quality code (yet).

## TODO

Technical features:

- [x] No ORM! Pure SQL code called by native PostgreSQL driver.
- [x] Blazingly fast HTML rendering with [markup.rs](https://github.com/utkarshkukreti/markup.rs).
- [x] Automatic CSS minifying to one single file.
- [ ] Automatic JavaScript minifying to one single file.
- [x] Themes are just CSS code; all themes share the same HTML.
- [ ] Internationalization.
- [x] Website's language set by URL (good for SEO).
- [x] Cookie-based auth.

Website features:

- [ ] Homepage.
- [ ] Blog.
- [ ] Pages.
- [ ] Admin panel.
    - [ ] Dashboard.
    - [ ] Statistics.
    - [ ] Server.
    - [ ] Users.
    - [ ] Languages.
    - [ ] Posts.
    - [ ] Pages.
    - [ ] Files.
    - [ ] Website.
    - [ ] Tukosmo.

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
