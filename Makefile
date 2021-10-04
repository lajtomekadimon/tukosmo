
PROJECT=tukosmo

PG_DB=tukosmo
PG_USER=tukosmouser
PG_PASSWORD=1234



##############################################################################
#                               DEPENDENCIES                                 #
##############################################################################

postgresql:
	pkg install postgresql13-server postgresql13-contrib
	#sysrc postgresql_enable=yes
	/usr/local/etc/rc.d/postgresql initdb
	passwd postgres
	service postgresql start
	service postgresql status

rust:
	# Install Rust (https://www.rust-lang.org/tools/install)
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	# The shell must be restarted

dep: postgresql rust



##############################################################################
#                                 DATABASE                                   #
##############################################################################

createdb:
	@echo "Dropping database (if exists)..."
	su -m postgres -c 'dropdb $(PG_DB) ||:'
	su -m postgres -c 'dropuser $(PG_USER) ||:'
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
	su -m postgres -c "cat db/types/handlers.sql | psql -q -d $(PG_DB)"
	su -m postgres -c "cat db/types/requests.sql | psql -q -d $(PG_DB)"

FUNDB=db/extra/*.sql

gfunctionsdb:
	@echo "Creating extra functions..."
	su -m postgres -c "cat $(FUNDB) | psql -q -d $(PG_DB)"

STRUDB=db/tables/t_users.sql \
	   db/tables/t_sessions.sql \
	   db/tables/t_languages.sql \
	   db/tables/t_language_names.sql \
	   db/tables/t_pages.sql \
	   db/tables/t_page_translations.sql \
	   db/tables/t_posts.sql \
	   db/tables/t_post_translations.sql

structuredb:
	@echo "Creating structure..."
	su -m postgres -c "cat $(STRUDB) | psql -q -d $(PG_DB)"

DFUNDB=db/checks/*.sql \
       db/selects/*.sql \
       db/selects/count/*.sql \
       db/inserts/*.sql \
       db/deletes/*.sql \
       db/updates/*.sql \
       db/api/server/*.sql \
       db/api/web/website/*.sql \
       db/api/web/admin/*.sql

dfunctionsdb:
	@echo "Creating query and mutation functions..."
	su -m postgres -c "cat $(DFUNDB) | psql -q -d $(PG_DB)"

dscriptsdb:
	@echo "Running scripts..."
	su -m postgres -c "cat db/scripts/*.sql | psql -q -d $(PG_DB)"

installdb: createdb gfunctionsdb structuredb dfunctionsdb dscriptsdb
	@echo "The database is updated and ready!"

resetdb:
	su -m root -c 'make installdb'
	@echo "The database is reset and ready!"

## To improve
updatedb: gfunctionsdb dfunctionsdb
	@echo "The database is updated!"

psql:
	@su -m postgres -c "psql -q -d $(PG_DB)"



##############################################################################
#                                WEB SERVER                                  #
##############################################################################

install: clean frontend
	cargo build

run:
	cargo run



##############################################################################
#                                   EXTRA                                    #
##############################################################################

clean:
	rm -f Cargo.lock
	rm -Rf target
