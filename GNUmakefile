
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

PG_DB=tukosmo  # use the name you have in Tukosmo.toml
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
	source ~/.bashrc  # the shell must be restarted for cargo
	sudo apt install -y golang  # needed for JS minifying
	sudo apt install -y openssl libssl-dev pkg-config  # needed for TLS
else ifeq ($(OS_NAME), fedora)
rust:
	# Install Rust (https://www.rust-lang.org/tools/install)
	sudo dnf install -y curl
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	source ~/.bashrc  # the shell must be restarted for cargo
	sudo dnf install -y golang  # needed for JS minifying
	sudo dnf install -y openssl openssl-devel pkg-config  # needed for TLS
else ifeq ($(OS_NAME), arch)
rust:
	# Install Rust (https://www.rust-lang.org/tools/install)
	sudo pacman -S --noconfirm curl
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	source ~/.bashrc  # the shell must be restarted for cargo
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

db-password:
	sed -i '/password =/d' Tukosmo.toml
	echo "password = \"$(PG_PASSWORD)\"" >> Tukosmo.toml

psql:
	@sudo -i -u postgres psql -q -d $(PG_DB)



##############################################################################
#                                WEB SERVER                                  #
##############################################################################

clean:
	rm -f Cargo.lock
	rm -Rf target

ifeq ($(MODE), development)
install: clean
	# SSL for local development
	openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out \
	cert.pem -days 365 -subj '/CN=localhost'
	# Compile Tukosmo
	cargo build
else ifeq ($(MODE), production)
install: clean
	# Compile Tukosmo
	cargo build --release
endif

install-all: dep installdb db-password install

ifeq ($(MODE), development)
run:
	cargo run
else ifeq ($(MODE), production)
run:
	#cargo run --release
	sudo target/release/tukosmo
endif

# TODO: run-always (startup and in the background)

