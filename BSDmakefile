
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

DOMAIN=localhost
USER_EMAIL=test@test.com

PG_DB=tukosmo  # use the name you have in etc/Tukosmo.toml
PG_PASSWORD=$(shell pwgen -s -1 32)
# PG_PASSWORD is only used in the installation (TODO: test)



##############################################################################
#                               DEPENDENCIES                                 #
##############################################################################

postgresql:
	pkg install postgresql13-server postgresql13-contrib sysutils/pwgen
	sysrc postgresql_enable=yes
	/usr/local/etc/rc.d/postgresql initdb
	#passwd postgres
	service postgresql start
	service postgresql status

rust:
	# Install Rust (https://www.rust-lang.org/tools/install)
	pkg install ftp/curl
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	pkg install lang/go  # needed for JS minifying
	pkg install security/openssl security/openssl-devel  # needed for TLS
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
	rm -Rf static/bundles
	mkdir -p static/bundles

.if ${MODE} == development
install: clean
	sed -r -i 's/mode = \"production\"/mode = \"development\"/g' \
	etc/Tukosmo.toml
	sed -r -i 's/domain = \"localhost\"/domain = \"$(DOMAIN)\"/g' \
	etc/Tukosmo.toml
	sed -r -i \
	's/user_email = \"test@test.com\"/user_email = \"$(USER_EMAIL)\"/g' \
	etc/Tukosmo.toml
	sed -r -i 's/reset = \"false\"/reset = \"true\"/g' etc/Tukosmo.toml
	sed -r -i 's/password = \"1234\"/password = \"$(PG_PASSWORD)\"/g' \
	etc/Tukosmo.toml
	# SSL for local development
	openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out \
	cert.pem -days 365 -subj '/CN=localhost'
	# Compile Tukosmo
	cargo build
	# Create /temp dir
	mkdir -p temp
	# Create /static/bundles dir
	mkdir -p static/bundles
	# Unzip TinyMCE
	unzip static/js/external/tinymce.zip -d static/js/external/
	# Unzip EOS-icons
	unzip static/fonts/eos-icons.zip -d static/fonts/
.endif
.if ${MODE} == production
install: clean
	sed -r -i 's/mode = \"development\"/mode = \"production\"/g' \
	etc/Tukosmo.toml
	sed -r -i 's/domain = \"localhost\"/domain = \"$(DOMAIN)\"/g' \
	etc/Tukosmo.toml
	sed -r -i \
	's/user_email = \"test@test.com\"/user_email = \"$(USER_EMAIL)\"/g' \
	etc/Tukosmo.toml
	sed -r -i 's/reset = \"false\"/reset = \"true\"/g' etc/Tukosmo.toml
	sed -r -i 's/password = \"1234\"/password = \"$(PG_PASSWORD)\"/g' \
	etc/Tukosmo.toml
	# Set hostname (TODO: do it when Tukosmo changes its domain)
	su -m root -c 'hostname $(DOMAIN)'
	su -m root -c 'sysrc hostname=$(DOMAIN)'
	# TODO: vi /etc/hosts
	#       (from 127.0.0.1 old-hostname to 127.0.0.1 new-hostname)
	# Compile Tukosmo
	cargo build --release
	# Create /temp dir
	mkdir -p temp
	# Create /static/bundles dir
	mkdir -p static/bundles
	# Unzip TinyMCE
	unzip static/js/external/tinymce.zip -d static/js/external/
	# Unzip EOS-icons
	unzip static/fonts/eos-icons.zip -d static/fonts/
.endif

install-all: installdb install

resetdb:
	sed -r -i 's/reset = \"false\"/reset = \"true\"/g' etc/Tukosmo.toml

.if ${MODE} == development
run:
	cargo run
.endif
.if ${MODE} == production
run:
	#cargo run --release
	su -m root -c "target/release/tukosmo"  # Bad idea...
.endif

reset-run: resetdb run

.if ${MODE} == production
run-service:
	rm -f /usr/local/etc/rc.d/tukosmo
	cp etc/tukosmo-freebsd.sh /usr/local/etc/rc.d/tukosmo
	sysrc tukosmo_enable=yes
	service tukosmo start
	service tukosmo status
.endif

.if ${MODE} == production
stop-service:
	service tukosmo stop
	service tukosmo status
.endif

