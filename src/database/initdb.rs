use std::fs;
use postgres::{Client, NoTls, Error};

use crate::config::global::Config;


pub fn append_sql(
    path_vector: &mut Vec<String>,
    files_dir: &str,
) {
    let mut post_path_vector = Vec::new();

    let paths = fs::read_dir(files_dir).unwrap();
    for path in paths {
        let the_path = path.unwrap().path();

        if the_path.is_dir() {
            post_path_vector.push(the_path.to_str().unwrap().to_string());
        } else if the_path.is_file() {
            if let Some(extension) = the_path.extension() {
                if extension == "sql" {
                    path_vector.push(the_path.to_str().unwrap().to_string());
                }
            }
        }
    }

    for post_path in post_path_vector {
        append_sql(path_vector, &post_path);
    }
}


pub fn initdb(
    config: &Config,
) -> Result<(), Error> {

    println!("Creating database...");

    /*
    deletedb:
        @echo "Dropping database..."
        sudo -i -u postgres dropdb $(PG_DB) ||:
        sudo -i -u postgres dropuser $(PG_USER) ||:

    createdb:
        @echo "Creating database..."
        sudo -i -u postgres createdb $(PG_DB) -E UTF8
        sudo -i -u postgres psql -d $(PG_DB) -c \
        "CREATE USER $(PG_USER) PASSWORD '$(PG_PASSWORD)';"
        sudo -i -u postgres psql -d $(PG_DB) -c \
        "ALTER USER $(PG_USER) WITH SUPERUSER;"
    */

    // Reset files uploaded by the user
    match fs::remove_dir_all("files/") {
        Ok(_) => {
            fs::create_dir("files/").unwrap();
        },
        Err(_) => {
            fs::create_dir("files/").unwrap();
        },
    };
    fs::copy(
        "static/img/featured-image-default-post.jpg",
        "files/featured-image-default-post.jpg",
    ).unwrap();


    /* ARBITRARY DATABASE
     *======================================================================*/

    let pre_db_string_auth = &format!(
        "host={} dbname=pretukosmo user=pretukosmouser password=123456",
        &config.database.host,
    );

    match Client::connect(pre_db_string_auth, NoTls) {

        Ok(mut pre_client) => {

            // Create auxiliary database with user's name and password
            pre_client.batch_execute(&format!(
                "CREATE DATABASE {}_aux",
                &config.database.name,
            )).unwrap();
            pre_client.batch_execute(&format!(
                "CREATE USER {} PASSWORD '{}'",
                &config.database.user,
                &config.database.password,
            )).unwrap();
            pre_client.batch_execute(&format!(
                "ALTER USER {} WITH SUPERUSER",
                &config.database.user,
            )).unwrap();
            pre_client.batch_execute(&format!(
                "ALTER DATABASE {}_aux OWNER TO {}",
                &config.database.name,
                &config.database.user,
            )).unwrap();

            pre_client.close().unwrap();

        },

        Err(_) => {},

    }


    /* AUXILIARY DATABASE
     *======================================================================*/

    let aux_db_string_auth = &format!(
        "host={} dbname={}_aux user={} password={}",
        &config.database.host,
        &config.database.name,
        &config.database.user,
        &config.database.password,
    );

    let mut aux_client = Client::connect(aux_db_string_auth, NoTls)?;

    // Delete arbitrary database and user
    aux_client.batch_execute("DROP DATABASE IF EXISTS pretukosmo").unwrap();
    aux_client.batch_execute("DROP USER IF EXISTS pretukosmouser").unwrap();

    // Reset Tukosmo's database
    aux_client.batch_execute(&format!(
        "DROP DATABASE IF EXISTS {}",
        &config.database.name,
    )).unwrap();
    aux_client.batch_execute(&format!(
        "CREATE DATABASE {}",
        &config.database.name,
    )).unwrap();

    aux_client.close().unwrap();


    /* TUKOSMO DATABASE
     *======================================================================*/

    let mut client = Client::connect(&config.dbauth, NoTls)?;

    let mut sql_files = Vec::new();

    // Extensions
    //-------------------------------------------------------
    sql_files.append(&mut vec![
        "db/extensions.sql".to_string(),
    ]);

    // Types
    //-------------------------------------------------------
    sql_files.append(&mut vec![
        "db/types.sql".to_string(),
    ]);

    // Extra
    //-------------------------------------------------------
    append_sql(&mut sql_files, "db/extra/");

    // Tables
    //-------------------------------------------------------
    sql_files.append(&mut vec![
        "db/tables/t_users.sql".to_string(),
        "db/tables/t_sessions.sql".to_string(),
        "db/tables/t_languages.sql".to_string(),
        "db/tables/t_language_names.sql".to_string(),
        "db/tables/t_user_names.sql".to_string(),
        "db/tables/t_files.sql".to_string(),
        "db/tables/t_pages.sql".to_string(),
        "db/tables/t_page_translations.sql".to_string(),
        "db/tables/t_posts.sql".to_string(),
        "db/tables/t_post_translations.sql".to_string(),
        "db/tables/t_reset_password_codes.sql".to_string(),
    ]);

    // QUERY AND MUTATION FUNCTIONS
    //-------------------------------------------------------
    // Checks
    append_sql(&mut sql_files, "db/checks/");
    // Errors
    append_sql(&mut sql_files, "db/errors/");
    // Selects
    sql_files.append(&mut vec![
        "db/selects/s_user_name_by_user_lang.sql".to_string(),
        "db/selects/s_file_by_id.sql".to_string(),
        "db/selects/s_language_code_by_id.sql".to_string(),
        "db/selects/s_language_id_by_code.sql".to_string(),
        "db/selects/s_name_of_language.sql".to_string(),
        "db/selects/s_untrans_post_title_by_id.sql".to_string(),
    ]);
    append_sql(&mut sql_files, "db/selects/");
    // Inserts
    append_sql(&mut sql_files, "db/inserts/");
    // Deletes
    append_sql(&mut sql_files, "db/deletes/");
    // Updates
    append_sql(&mut sql_files, "db/updates/");
    // API
    append_sql(&mut sql_files, "db/api/");

    // SCRIPTS
    //-------------------------------------------------------
    sql_files.append(&mut vec![
        "db/scripts/01-init.sql".to_string(),
        "db/scripts/02-example-post.sql".to_string(),
    ]);

    for sql_file in sql_files {
        let sql_code = fs::read_to_string(&sql_file)
            .expect(&format!("Something went wrong reading {}", &sql_file));

        match client.batch_execute(&sql_code) {
            Ok(_) => {},
            Err(e) => {
                println!("{}", &e);
                println!("SQL CODE ERROR in {}", &sql_file);
                panic!("Initdb stopped.");
            }
        };
    }

    client.close().unwrap();

    Ok(())
}

