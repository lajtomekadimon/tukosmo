
## Operating system
## - debian [works for Ubuntu, Mint, etc.]
## - fedora
## - arch
## - opensuse (TODO)
## - gentoo (TODO)
OS_NAME=fedora

## Mode
## - development
## - production
MODE=development

DOMAIN=localhost
USER_EMAIL=test@test.com

PG_DB=tukosmo  # use the name you have in etc/Tukosmo.toml
PG_PASSWORD=$(shell pwgen -s -1 32)
# PG_PASSWORD is only used in the installation



##############################################################################
#                               DEPENDENCIES                                 #
##############################################################################

ifeq ($(OS_NAME), debian)
postgresql:
	sudo apt install -y postgresql postgresql-client \
	postgresql-contrib uuid libpq-dev pwgen
	sudo apt clean -y
	#sudo passwd postgres
	sudo systemctl enable postgresql
	sudo systemctl start postgresql
else ifeq ($(OS_NAME), fedora)
postgresql:
	sudo dnf -y install postgresql postgresql-server postgresql-contrib \
	postgresql-devel pwgen
	sudo postgresql-setup --initdb --unit postgresql
	#sudo passwd postgres
	## TODO: Change all 'ident' to 'trust' in pg_hbd.conf
	sudo systemctl enable postgresql
	sudo systemctl start postgresql
else ifeq ($(OS_NAME), arch)
postgresql:
	sudo pacman -S --noconfirm postgresql pwgen
	#sudo passwd postgres
	sudo -i -u postgres initdb --locale es_ES.UTF-8 -E UTF8 -D \
	    '/var/lib/postgres/data'
	sudo systemctl enable postgresql
	sudo systemctl start postgresql
endif

ifeq ($(OS_NAME), debian)
rust:
	# Install Rust (https://www.rust-lang.org/tools/install)
	sudo apt install -y curl
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	sudo apt install -y golang  # needed for JS minifying
	sudo apt install -y openssl libssl-dev pkg-config  # needed for TLS
else ifeq ($(OS_NAME), fedora)
rust:
	# Install Rust (https://www.rust-lang.org/tools/install)
	sudo dnf install -y curl
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	sudo dnf install -y golang  # needed for JS minifying
	sudo dnf install -y openssl openssl-devel pkg-config  # needed for TLS
else ifeq ($(OS_NAME), arch)
rust:
	# Install Rust (https://www.rust-lang.org/tools/install)
	sudo pacman -S --noconfirm curl
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	sudo pacman -S --noconfirm go  # needed for JS minifying
	sudo pacman -S --noconfirm openssl pkgconf  # needed for TLS
endif

dep: postgresql rust



##############################################################################
#                                 DATABASE                                   #
##############################################################################

installdb:
	@echo "Creating database..."
	sudo -i -u postgres createdb pretukosmo -E UTF8
	sudo -i -u postgres psql -d pretukosmo -c \
	"CREATE USER pretukosmouser PASSWORD '123456';"
	sudo -i -u postgres psql -d pretukosmo -c \
	"ALTER USER pretukosmouser WITH SUPERUSER;"

psql:
	@sudo -i -u postgres psql -q -d $(PG_DB)



##############################################################################
#                                WEB SERVER                                  #
##############################################################################

clean:
	rm -f Cargo.lock
	rm -Rf target
	rm -Rf static/bundles
	mkdir -p static/bundles

ifeq ($(MODE), development)
install: clean
	sed -i 's/mode = \"production\"/mode = \"development\"/g' etc/Tukosmo.toml
	sed -i 's/domain = \"localhost\"/domain = \"$(DOMAIN)\"/g' etc/Tukosmo.toml
	sed -i \
	's/user_email = \"test@test.com\"/user_email = \"$(USER_EMAIL)\"/g' \
	etc/Tukosmo.toml
	sed -i 's/reset = \"false\"/reset = \"true\"/g' etc/Tukosmo.toml
	sed -i 's/password = \"1234\"/password = \"$(PG_PASSWORD)\"/g' \
	etc/Tukosmo.toml
	# SSL for local development
	openssl req -x509 -newkey rsa:4096 -nodes -keyout etc/pkey.pem -out \
	etc/cert.pem -days 365 -subj '/CN=localhost'
	# Compile Tukosmo
	cargo build
	# Create /temp dir
	mkdir -p temp
	# Create /static/bundles dir
	mkdir -p static/bundles
else ifeq ($(MODE), production)
install: clean
	sed -i 's/mode = \"development\"/mode = \"production\"/g' etc/Tukosmo.toml
	sed -i 's/domain = \"localhost\"/domain = \"$(DOMAIN)\"/g' etc/Tukosmo.toml
	sed -i \
	's/user_email = \"test@test.com\"/user_email = \"$(USER_EMAIL)\"/g' \
	etc/Tukosmo.toml
	sed -i 's/reset = \"false\"/reset = \"true\"/g' etc/Tukosmo.toml
	sed -i 's/password = \"1234\"/password = \"$(PG_PASSWORD)\"/g' \
	etc/Tukosmo.toml
	# Set hostname (TODO: do it when Tukosmo changes its domain)
	sudo hostnamectl set-hostname $(DOMAIN)
	# Compile Tukosmo
	cargo build --release
	# Create /temp dir
	mkdir -p temp
	# Create /static/bundles dir
	mkdir -p static/bundles
endif

install-all: installdb install

resetdb:
	sed -i 's/reset = \"false\"/reset = \"true\"/g' etc/Tukosmo.toml

ifeq ($(MODE), development)
run:
	cargo run
else ifeq ($(MODE), production)
run:
	#cargo run --release
	sudo target/release/tukosmo
endif

ifeq ($(MODE), production)
run-service:
	sudo cp etc/tukosmo.service /etc/systemd/system/
	sudo systemctl daemon-reload
	sudo systemctl enable tukosmo.service
	sudo systemctl start tukosmo.service
	sudo systemctl status tukosmo.service
endif

ifeq ($(MODE), production)
stop-service:
	sudo systemctl stop tukosmo.service
	sudo systemctl status tukosmo.service
endif

