<div align="center">

![Tukosmo](./static/faviconadmin/favicon-96x96.png)

**Tukosmo** (WIP!): Website generator made with Rust and PostgreSQL.

[![License: MIT](
https://img.shields.io/badge/License-MIT-blue.svg)](
https://github.com/lajtomekadimon/tukosmo/blob/main/LICENSE)


</div>

NOTE: This is my first Rust project. I'm still learning the language, so
don't expect high quality code (yet).

## TODO

Technical features:

- [x] Blazingly fast web server using [Actix Web](
https://github.com/actix/actix-web).
- [x] No ORM! Pure SQL code called by native [PostgreSQL driver](
https://github.com/sfackler/rust-postgres).
- [x] "One request, one query" policy.
- [x] Blazingly fast HTML rendering with [markup.rs](
https://github.com/utkarshkukreti/markup.rs).
- [ ] Minimum of allocation and copying in Rust (efficiency!).
- [x] Automatic CSS minifying to one single file.
- [x] Everything works without JavaScript, which is optional.
- [ ] Automatic JavaScript minifying to one single file.
- [ ] Zero external resources (CSS, JavaScript, images, ...).
- [x] Themes are just CSS code; all themes share the same HTML.
- [ ] Responsive design.
- [x] Internationalization (i18n).
- [x] Website's language set by URL (good for SEO).
- [x] Custom languages.
- [x] Cookie-based auth.
- [x] CommonMark-based posts and pages.
- [ ] Self-update server OS and software (optional).
- [ ] Self-update Tukosmo version (optional).

## Roadmap

- [ ] 0.1
    - [ ] Foundations of Tukosmo
    - [x] Users system
    - [x] Custom languages
    - [x] Blog
    - [ ] Pages
    - [ ] Files
    - [x] Admin Panel
- [ ] 0.2
    - [ ] FAQ
    - [ ] Special pages (contact, team, cv, etc.)
    - [ ] RSS
    - [ ] Social links
    - [ ] OpenGraph cards for Facebook, Twitter, Mastodon...
- [ ] 0.3
    - [ ] Custom homepage
    - [ ] Custom menus and widgets
    - [ ] Searcher
- [ ] 0.4
    - [ ] Improved user experience with JavaScript
        - [ ] WYSIWYG CommonMark editor for posts, pages and FAQ
        - [ ] Real-time form validation
        - [ ] Change pages loading just the new content using Ajax
- [ ] 0.5
    - [ ] Statistics
    - [ ] Server info
- [ ] 1.0
    - [ ] Self-update server
    - [ ] Self-update Tukosmo
- [ ] ?.?
    - [ ] Download any post as PDF
    - [ ] Safe payments between users
    - [ ] Membership
    - [ ] Shop
    - [ ] Forum

## Instalation

Dependencies:

- PostgreSQL +13
- Rust 2018

Install dependencies (only on FreeBSD for now):

```sh
make -s dep
# The shell must be restarted after installing Rust
# It's important to add postgresql_enable="yes" to /etc/rc.conf in FreeBSD
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
