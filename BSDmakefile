
## Operating system
## - freebsd
## - openbsd (TODO)
## - dragonflybsd (TODO)
## - netbsd (TODO)
OS_NAME=freebsd

## Mode
## - development
## - production
MODE=development

PG_DB=tukosmo  # use the name you have in Tukosmo.toml



##############################################################################
#                               DEPENDENCIES                                 #
##############################################################################

postgresql:
	pkg install postgresql13-server postgresql13-contrib
	sysrc postgresql_enable=yes
	/usr/local/etc/rc.d/postgresql initdb
	#passwd postgres
	service postgresql start
	service postgresql status

rust:
	# Install Rust (https://www.rust-lang.org/tools/install)
	pkg install ftp/curl
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	source .cshrc  # the shell must be restarted for cargo (TODO: test)
	pkg install lang/go  # needed for JS minifying
	pkg install security/openssl security/openssl-devel
	# Certbot (automated SSL certificates)
	pkg install security/py-certbot

dep: postgresql rust



##############################################################################
#                                 DATABASE                                   #
##############################################################################

preinstalldb:
	@echo "Creating database..."
	su -m postgres -c 'createdb pretukosmo -E UTF8'
	su -m postgres -c "psql -q -d pretukosmo \
	-c \"CREATE USER pretukosmouser PASSWORD '123456';\""
	su -m postgres -c "psql -q -d pretukosmo \
	-c \"ALTER USER pretukosmouser WITH SUPERUSER;\""

installdb:
	su -m root -c 'make preinstalldb'

psql:
	@su -m postgres -c "psql -q -d $(PG_DB)"



##############################################################################
#                                WEB SERVER                                  #
##############################################################################

clean:
	rm -f Cargo.lock
	rm -Rf target

.if ${MODE} == development
ssl: clean
	# SSL for local development
	openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out \
	cert.pem -days 365 -subj '/CN=localhost'
.endif
.if ${MODE} == production
ssl: clean
	# SSL for production server (THIS HAS NOT BEEN TESTED!!!)
	su -m root -c "certbot certonly --standalone -d $(DOMAIN) --agree-tos \
	--register-unsafely-without-email --noninteractive"
	ln -s /etc/letsencrypt/live/$(DOMAIN)/fullchain.pem cert.pem
	ln -s /etc/letsencrypt/live/$(DOMAIN)/privkey.pem key.pem
.endif

.if ${MODE} == development
install: clean ssl
	cargo build
.endif
.if ${MODE} == production
install: clean ssl
	cargo build --release
.endif

install-all: dep installdb install ssl

.if ${MODE} == development
run:
	cargo run
.endif
.if ${MODE} == production
run:
	#cargo run --release
	su -m root -c "target/release/tukosmo"  # Bad idea...
.endif

# TODO: run-always (startup and in the background)

