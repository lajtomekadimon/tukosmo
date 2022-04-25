<div align="center">

![Tukosmo](./static/faviconadmin/favicon-96x96.png)

**Tukosmo**: Website generator made with Rust and PostgreSQL.

[![License: MIT](
https://img.shields.io/badge/License-MIT-blue.svg)](
https://github.com/lajtomekadimon/tukosmo/blob/main/LICENSE)

</div>

NOTE: This is my first Rust project. **Work in progress!**

## TODO

Technical features:

- [x] Works on FreeBSD and GNU/Linux (full list of distros on GNUmakefile).
- [x] Blazingly fast web server using [Actix Web](
https://github.com/actix/actix-web).
- [x] Automatic TLS certificates in production using [Let's Encrypt](
https://letsencrypt.org).
- [x] Always using HTTPS; redirects all HTTP traffic to HTTPS.
- [x] No ORM! Pure SQL code called by native [PostgreSQL driver](
https://github.com/sfackler/rust-postgres).
- [x] "One request, one query" policy.
- [x] Blazingly fast HTML rendering with [markup.rs](
https://github.com/utkarshkukreti/markup.rs).
- [ ] Minimum of allocation and copying in Rust (efficiency!).
- [x] Automatic CSS minifying to one single file.
- [x] Automatic JavaScript minifying to one single file.
- [x] Zero external resources (CSS, JavaScript, images, ...).
- [x] Themes are just CSS code; all themes share the same HTML.
- [x] Responsive design.
- [x] Internationalization (i18n).
- [x] Website's language set by URL (good for SEO).
- [x] Custom languages.
- [x] Cookie-based auth.
- [x] Icons by [EOS icons](https://eos-icons.com/).
- [x] Charts by [Chart.js](https://github.com/chartjs/Chart.js).
- [x] System data by [systemstat](https://github.com/unrelentingtech/systemstat).
- [x] Admin Panel; styles using [Bulma](https://bulma.io/).
- [x] WYSIWYG post editor using [TinyMCE](
https://github.com/tinymce/tinymce).
- [x] Automatic Open Graph cards.
- [x] Automatic RSS feed.
- [ ] Self-update server OS and software (optional).
- [ ] Self-update Tukosmo version (optional).

Roadmap:

- [ ] 0.1
    - [ ] Foundations of Tukosmo
    - [x] Statistics
    - [ ] Server
    - [x] Users
    - [x] Languages
    - [x] Tags
    - [x] Posts
    - [ ] Pages
    - [x] Files
    - [x] Favicon
    - [ ] Theme
    - [ ] Menus
    - [ ] Social links
    - [ ] Widgets
- [ ] ?.?
    - [ ] Download any post as PDF
    - [ ] Safe payments between users
    - [ ] Membership
    - [ ] Shop
    - [ ] Forum

## Pre-installation

On a freshly installed server, run your OS commands:

```sh
# FreeBSD
pkg upgrade
freebsd-update fetch
freebsd-update install
pkg install devel/git

# Debian [works for Ubuntu, Mint, etc.]
sudo apt update
sudo apt upgrade
sudo apt install git make

# Fedora
sudo dnf update
sudo dnf install git make

# Arch
sudo pacman -Syu
sudo pacman -S git make
```

Basically, you need an updated system and `git` and `make` installed.
**If you are using a desktop system or a server with more software,
please follow step-by-step installation and check every command you run.**

## Installation

```sh
git clone https://github.com/lajtomekadimon/tukosmo

# If you are in development (local)
cd tukosmo

# If you are in production on FreeBSD
su -m root -c 'mv tukosmo /srv/tukosmo'
cd /srv/tukosmo

# If you are in production on GNU/Linux
sudo mv tukosmo /srv/tukosmo
cd /srv/tukosmo
```

Edit BSDmakefile and GNUmakefile and change `OS_NAME` and `MODE`. If you are
in production, change `DOMAIN` and `USER_EMAIL` as well.

Install dependencies:

```sh
make -s dep
```

Reboot server.

Install Tukosmo:

```sh
cd tukosmo  # /srv/tukosmo in production
make -s install-all
```

## Step-by-step installation

Download Tukosmo's repository:

```sh
git clone https://github.com/lajtomekadimon/tukosmo

# If you are in development (local)
cd tukosmo

# If you are in production
mv tukosmo /srv/tukosmo
cd /srv/tukosmo
```

Edit BSDmakefile and GNUmakefile and change `OS_NAME` and `MODE`. If you are
in production, change `DOMAIN` and `USER_EMAIL` as well.

Dependencies:

- PostgreSQL >= 13
- Rust 2018
- Go >= 1.13

Install dependencies:

**(skip this if you already have PostgreSQL and Rust on your system!)**

```sh
make -s dep
# Install just PostgreSQL: make -s postgresql
# Install just Rust: make -s rust
# The shell must be restarted after installing Rust
```

Reboot server.

Install Tukosmo:

```sh
cd tukosmo  # /srv/tukosmo in production
make -s installdb

# If you're using Fedora, you should also do:
#sudo systemctl stop postgresql
#sudo vi /var/lib/pgsql/data/pg_hba.conf  # change all 'ident' to 'trust'
#sudo systemctl start postgresql

make -s install
```

## Run server

Run web server:

```sh
make -s run
```

Run web server in the background, starting on every boot:

```sh
make -s run-service

# If you want to stop it:
make -s stop-service
```

