use std::fs;
use tokio_postgres::{connect, NoTls, Error};
use tokio;

use crate::config::{
    global::Config,
    change_reset::change_reset,
};


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


pub async fn initdb(
    config: &Config,
) -> Result<(), Error> {

    println!("Creating database...");

    // Reset files uploaded by the user
    fs::remove_dir_all("files")
        .and_then(|_| fs::create_dir("files")).unwrap();
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

    match connect(pre_db_string_auth, NoTls).await {

        Ok((pre_client, connection)) => {
            // The connection object performs the actual communication with
            // the database, so spawn it off to run on its own
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("Connection error: {}", e);
                }
            });

            // Create auxiliary database with user's name and password
            pre_client.batch_execute(&format!(
                "CREATE DATABASE {}_aux",
                &config.database.name,
            )).await.unwrap();
            pre_client.batch_execute(&format!(
                "CREATE USER {} PASSWORD '{}'",
                &config.database.user,
                &config.database.password,
            )).await.unwrap();
            pre_client.batch_execute(&format!(
                "ALTER USER {} WITH SUPERUSER",
                &config.database.user,
            )).await.unwrap();
            pre_client.batch_execute(&format!(
                "ALTER DATABASE {}_aux OWNER TO {}",
                &config.database.name,
                &config.database.user,
            )).await.unwrap();

            //pre_client.close().unwrap();

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

    let (aux_client, connection1) = connect(aux_db_string_auth, NoTls).await?;

    // The connection object performs the actual communication with
    // the database, so spawn it off to run on its own
    tokio::spawn(async move {
        if let Err(e) = connection1.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Delete arbitrary database and user
    aux_client.batch_execute(
        "DROP DATABASE IF EXISTS pretukosmo"
    ).await.unwrap();
    aux_client.batch_execute(
        "DROP USER IF EXISTS pretukosmouser"
    ).await.unwrap();

    // Reset Tukosmo's database
    aux_client.batch_execute(&format!(
        "DROP DATABASE IF EXISTS {}",
        &config.database.name,
    )).await.unwrap();
    aux_client.batch_execute(&format!(
        "CREATE DATABASE {}",
        &config.database.name,
    )).await.unwrap();

    //aux_client.close().unwrap();


    /* TUKOSMO DATABASE
     *======================================================================*/

    let (client, connection2) = connect(&config.dbauth, NoTls).await?;

    // The connection object performs the actual communication with
    // the database, so spawn it off to run on its own
    tokio::spawn(async move {
        if let Err(e) = connection2.await {
            eprintln!("Connection error: {}", e);
        }
    });

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
        "db/tables/t_tags.sql".to_string(),
        "db/tables/t_tag_translations.sql".to_string(),
        "db/tables/t_user_names.sql".to_string(),
        "db/tables/t_files.sql".to_string(),
        "db/tables/t_pages.sql".to_string(),
        "db/tables/t_page_translations.sql".to_string(),
        "db/tables/t_posts.sql".to_string(),
        "db/tables/t_post_translations.sql".to_string(),
        "db/tables/t_tags_of_posts.sql".to_string(),
        "db/tables/t_reset_password_codes.sql".to_string(),
        /*---*/
        "db/tables/stats/by_min/tsm_server.sql".to_string(),
        "db/tables/stats/by_hour/tsh_visits.sql".to_string(),
        "db/tables/stats/by_hour/tsh_visits_visitors.sql".to_string(),
        "db/tables/stats/by_hour/tsh_visits_referrals.sql".to_string(),
        "db/tables/stats/by_hour/tsh_visits_referrals_visitors.sql".to_string(),
        "db/tables/stats/by_hour/tsh_visits_browsers.sql".to_string(),
        "db/tables/stats/by_hour/tsh_visits_platforms.sql".to_string(),
        "db/tables/stats/by_hour/tsh_server.sql".to_string(),
    ]);

    // QUERY AND MUTATION FUNCTIONS
    //-------------------------------------------------------
    // Checks
    append_sql(&mut sql_files, "db/checks/");
    // Errors
    append_sql(&mut sql_files, "db/errors/");
    // Selects
    sql_files.append(&mut vec![
        "db/selects/count/sc_stats_visitors_total_by_referrer.sql".to_string(),
        "db/selects/count/sc_stats_visitors_total_by_browser.sql".to_string(),
        "db/selects/count/sc_stats_visitors_total_by_platform.sql".to_string(),
        "db/selects/s_user_name_by_user_lang.sql".to_string(),
        "db/selects/s_file_by_id.sql".to_string(),
        "db/selects/s_language_code_by_id.sql".to_string(),
        "db/selects/s_language_id_by_code.sql".to_string(),
        "db/selects/s_language_by_id_lang.sql".to_string(),
        "db/selects/s_name_of_language.sql".to_string(),
        "db/selects/s_untrans_tag_name_by_id.sql".to_string(),
        "db/selects/s_tags_of_post.sql".to_string(),
        "db/selects/s_untrans_post_title_by_id.sql".to_string(),
    ]);
    append_sql(&mut sql_files, "db/selects/");
    // Inserts
    append_sql(&mut sql_files, "db/inserts/");
    // Deletes
    append_sql(&mut sql_files, "db/deletes/");
    // Updates
    append_sql(&mut sql_files, "db/updates/");
    // Triggers
    append_sql(&mut sql_files, "db/triggers/");
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

        match client.batch_execute(&sql_code).await {
            Ok(_) => {},
            Err(e) => {
                println!("{}", &e);
                println!("SQL CODE ERROR in {}", &sql_file);
                panic!("Initdb stopped.");
            }
        };
    }

    //client.close().unwrap();

    // Change reset value to false
    //-------------------------------------------------------
    change_reset(config, "false");

    println!("Database created!");

    Ok(())
}

