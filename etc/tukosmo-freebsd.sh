#!/bin/sh

# PROVIDE: tukosmo
# REQUIRE: DAEMON postgresql

. /etc/rc.subr

name=tukosmo
rcvar=tukosmo_enable

command="/srv/tukosmo/target/release/tukosmo >> /srv/tukosmo/etc/server.log"

pidfile="/var/run/${name}.pid"

required_files="/srv/tukosmo/target/release/tukosmo"

load_rc_config $name
run_rc_command "$1"
