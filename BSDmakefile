
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
DOMAIN=example.com

PG_DB=tukosmo
PG_USER=tukosmouser
PG_PASSWORD=1234



##############################################################################
#                               DEPENDENCIES                                 #
##############################################################################

postgresql:
	pkg install postgresql13-server postgresql13-contrib
	sysrc postgresql_enable=yes
	/usr/local/etc/rc.d/postgresql initdb
	passwd postgres
	service postgresql start
	service postgresql status

rust:
	# Install Rust (https://www.rust-lang.org/tools/install)
	pkg install ftp/curl
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	### The shell must be restarted
	pkg install lang/go  # needed for JS minifying
	pkg install security/openssl security/openssl-devel
	# Certbot (automated SSL certificates)
	pkg install security/py-certbot

dep: postgresql rust



##############################################################################
#                                 DATABASE                                   #
##############################################################################

deletedb:
	@echo "Dropping database..."
	sudo -i -u postgres dropdb $(PG_DB) ||:
	sudo -i -u postgres dropuser $(PG_USER) ||:
	@echo "Deleting files..."
	rm -Rf files/

createdb:
	@echo "Creating files directory..."
	mkdir files
	cp static/img/featured-image-default-post.jpg files/
	@echo "Creating database..."
	su -m postgres -c 'createdb $(PG_DB) -E UTF8'
	su -m postgres -c "psql -q -d $(PG_DB) \
	-c \"CREATE USER $(PG_USER) PASSWORD '$(PG_PASSWORD)';\""
	su -m postgres -c "psql -q -d $(PG_DB) \
	-c \"ALTER USER $(PG_USER) WITH SUPERUSER;\""
	@echo "Installing extensions..."
	su -m postgres -c "cat db/extensions.sql | psql -q -d $(PG_DB)"
	@echo "Creating types..."
	su -m postgres -c "cat db/types.sql | psql -q -d $(PG_DB)"

FUNDB=db/extra/*.sql

gfunctionsdb:
	@echo "Creating extra functions..."
	su -m postgres -c "cat $(FUNDB) | psql -q -d $(PG_DB)"

STRUDB=db/tables/t_users.sql \
	   db/tables/t_sessions.sql \
	   db/tables/t_languages.sql \
	   db/tables/t_language_names.sql \
	   db/tables/t_user_names.sql \
	   db/tables/t_files.sql \
	   db/tables/t_pages.sql \
	   db/tables/t_page_translations.sql \
	   db/tables/t_posts.sql \
	   db/tables/t_post_translations.sql \
	   db/tables/t_reset_password_codes.sql

structuredb:
	@echo "Creating structure..."
	su -m postgres -c "cat $(STRUDB) | psql -q -d $(PG_DB)"

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
	su -m postgres -c "cat $(DFUNDB) | psql -q -d $(PG_DB)"

dscriptsdb:
	@echo "Running scripts..."
	su -m postgres -c "cat db/scripts/*.sql | psql -q -d $(PG_DB)"

installdb: createdb gfunctionsdb structuredb dfunctionsdb dscriptsdb
	@echo "The database is updated and ready!"

resetdball: deletedb installdb
	@echo "The database is reset and ready!"

resetdb:
	su -m root -c 'make resetdball'

## To improve
updatedb: gfunctionsdb dfunctionsdb
	@echo "The database is updated!"

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
	su -m root -c "certbot certonly --standalone -d $(DOMAIN)"
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

