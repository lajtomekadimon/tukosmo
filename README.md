<div align="center">

![Tukosmo](./static/faviconadmin/favicon-96x96.png)

**Tukosmo**: Website generator made with Rust and PostgreSQL.

**(this is a work in progress!)**

[![License: MIT](
https://img.shields.io/badge/License-MIT-blue.svg)](
https://github.com/lajtomekadimon/tukosmo/blob/main/LICENSE)

</div>

NOTE: This is my first Rust project. I'm still learning the language, so
don't expect high quality code (yet).

## TODO

Technical features:

- [x] Works on FreeBSD and GNU/Linux (full list of distros on GNUmakefile).
- [x] Blazingly fast web server using [Actix Web](
https://github.com/actix/actix-web).
- [x] Always using HTTPS.
- [x] No ORM! Pure SQL code called by native [PostgreSQL driver](
https://github.com/sfackler/rust-postgres).
- [x] "One request, one query" policy.
- [x] Blazingly fast HTML rendering with [markup.rs](
https://github.com/utkarshkukreti/markup.rs).
- [ ] Minimum of allocation and copying in Rust (efficiency!).
- [x] Automatic CSS minifying to one single file.
- [x] Automatic JavaScript minifying to one single file.
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

Roadmap:

- [ ] 0.1
    - [ ] Foundations of Tukosmo
    - [x] Users system
    - [x] Custom languages
    - [x] Blog
    - [x] Files
    - [x] Admin Panel
- [ ] 0.2
    - [ ] Improved user experience with JavaScript
        - [ ] WYSIWYG CommonMark editor for posts
        - [ ] Real-time form validation
        - [ ] Change pages loading just the new content using Ajax
- [ ] 0.3
    - [ ] Custom menus and widgets
    - [ ] Pages
    - [ ] Special pages (contact, team, cv, homepage?, etc.)
    - [ ] Custom homepage
- [ ] 0.4
    - [ ] FAQ
    - [ ] RSS
    - [ ] Searcher
    - [ ] Social links
    - [ ] OpenGraph cards for Facebook, Twitter, Mastodon...
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

## Installation

Dependencies:

- PostgreSQL >= 13
- Rust 2018
- Go >= 1.13

Install dependencies (only on FreeBSD and GNU/Linux for now; if you are using
a GNU/Linux distribution, change the `OS_NAME` parameter on GNUmakefile file):

**(skip this if you already have PostgreSQL and Rust on your system!)**

```sh
make -s dep
# Install just PostgreSQL: make -s postgresql
# Install just Rust: make -s rust
# The shell must be restarted after installing Rust
# It's important to add postgresql_enable="yes" to /etc/rc.conf in FreeBSD
```

Create (or reset) database:

```sh
make -s resetdb

# If you're using Fedora, you should also do:
sudo systemctl stop postgresql
sudo vi /var/lib/pgsql/data/pg_hba.conf  # change all 'ident' to 'trust'
sudo systemctl start postgresql
```

Install web server:

```sh
make -s install
```

## Run server

Before running the web server, you need an SSL certificate. In localhost, you
can generate it like this:

```sh
openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out \
cert.pem -days 365 -subj '/CN=localhost'
```

Run web server:

```sh
make -s run
```
