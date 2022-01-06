
## Operating system
## - debian [works for Ubuntu, Mint, etc.]
## - fedora
## - suse
## - arch
OS_NAME=fedora

PROJECT=tukosmo

PG_DB=tukosmo
PG_USER=tukosmouser
PG_PASSWORD=1234
PG_PSQL=sudo -i -u postgres psql -q -d $(PG_DB)



##############################################################################
#                               DEPENDENCIES                                 #
##############################################################################

ifeq ($(OS_NAME), debian)
postgresql:
	sudo apt install -y postgresql postgresql-client \
	postgresql-contrib uuid libpq-dev
	sudo apt clean -y
	sudo passwd postgres
	sudo systemctl enable postgresql
	sudo systemctl start postgresql
else ifeq ($(OS_NAME), fedora)
postgresql:
	sudo dnf -y install postgresql postgresql-server postgresql-contrib \
	postgresql-devel
	sudo postgresql-setup --initdb --unit postgresql
	sudo passwd postgres
	sudo systemctl enable postgresql
	sudo systemctl start postgresql
else ifeq ($(OS_NAME), suse)
postgresql:
	sudo zypper -n install postgresql postgresql-server postgresql-contrib \
	postgresql-devel
	sudo passwd postgres
	sudo systemctl enable postgresql
	sudo systemctl start postgresql
else ifeq ($(OS_NAME), arch)
postgresql:
	sudo pacman -S --noconfirm postgresql
	sudo passwd postgres
	sudo -i -u postgres initdb --locale es_ES.UTF-8 -E UTF8 -D \
	    '/var/lib/postgres/data'
	sudo systemctl enable postgresql
	sudo systemctl start postgresql
endif

ifeq ($(OS_NAME), debian)
rust:
	# Install Rust (https://www.rust-lang.org/tools/install)
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	# The shell must be restarted
	sudo apt install -y golang  # needed for JS minifying
	sudo apt install -y openssl
else ifeq ($(OS_NAME), fedora)
rust:
	# Install Rust (https://www.rust-lang.org/tools/install)
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	# The shell must be restarted
	sudo dnf install -y golang  # needed for JS minifying
	sudo dnf install -y openssl
else ifeq ($(OS_NAME), suse)
rust:
	# Install Rust (https://www.rust-lang.org/tools/install)
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	# The shell must be restarted
	sudo zypper -n install go  # needed for JS minifying
	sudo zypper -n install openssl
else ifeq ($(OS_NAME), arch)
rust:
	# Install Rust (https://www.rust-lang.org/tools/install)
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	# The shell must be restarted
	sudo pacman -S --noconfirm go  # needed for JS minifying
	sudo pacman -S --noconfirm openssl
endif

dep: postgresql rust



##############################################################################
#                                 DATABASE                                   #
##############################################################################

deletedb:
	@echo "Dropping database..."
	sudo -i -u postgres dropdb $(PG_DB) ||:
	sudo -i -u postgres dropuser $(PG_USER) ||:
	@echo "Deleting files..."
	rm files/*
	cp static/img/featured-image-default-post.jpg files/

createdb:
	@echo "Creating database..."
	sudo -i -u postgres createdb $(PG_DB) -E UTF8
	sudo -i -u postgres psql -d $(PG_DB) -c \
	"CREATE USER $(PG_USER) PASSWORD '$(PG_PASSWORD)';"
	sudo -i -u postgres psql -d $(PG_DB) -c \
	"ALTER USER $(PG_USER) WITH SUPERUSER;"
	@echo "Installing extensions..."
	cat db/extensions.sql | $(PG_PSQL)
	@echo "Creating types..."
	cat db/types.sql | $(PG_PSQL)

FUNDB=db/extra/*.sql

gfunctionsdb:
	@echo "Creating extra functions..."
	cat $(FUNDB) | $(PG_PSQL)

STRUDB=db/tables/t_users.sql \
	   db/tables/t_sessions.sql \
	   db/tables/t_languages.sql \
	   db/tables/t_language_names.sql \
	   db/tables/t_user_names.sql \
	   db/tables/t_files.sql \
	   db/tables/t_pages.sql \
	   db/tables/t_page_translations.sql \
	   db/tables/t_posts.sql \
	   db/tables/t_post_translations.sql

structuredb:
	@echo "Creating structure..."
	cat $(STRUDB) | $(PG_PSQL)

DFUNDB=db/checks/*.sql \
	   db/errors/*.sql \
       db/selects/*.sql \
       db/selects/count/*.sql \
       db/inserts/*.sql \
       db/deletes/*.sql \
       db/updates/*.sql \
       db/api/server/*.sql \
       db/api/handlers/website/*.sql \
       db/api/handlers/website/scope_blog/*.sql \
       db/api/handlers/admin/*.sql \
       db/api/handlers/admin/scope_users/*.sql \
       db/api/handlers/admin/scope_languages/*.sql \
       db/api/handlers/admin/scope_posts/*.sql \
       db/api/handlers/admin/scope_files/*.sql \
       db/api/handlers/admin/scope_json/*.sql

dfunctionsdb:
	@echo "Creating query and mutation functions..."
	cat $(DFUNDB) | $(PG_PSQL)

dscriptsdb:
	@echo "Running scripts..."
	cat db/scripts/*.sql | $(PG_PSQL)

installdb: createdb gfunctionsdb structuredb dfunctionsdb dscriptsdb
	@echo "The database is updated and ready!"

resetdb: deletedb installdb
	@echo "The database is reset and ready!"

## To improve
updatedb: gfunctionsdb dfunctionsdb
	@echo "The database is updated!"

psql:
	@sudo -i -u postgres psql -q -d $(PG_DB)



##############################################################################
#                                WEB SERVER                                  #
##############################################################################

clean:
	rm -f Cargo.lock
	rm -Rf target

install: clean
	cargo build

release: clean
	cargo build --release

run:
	cargo run

