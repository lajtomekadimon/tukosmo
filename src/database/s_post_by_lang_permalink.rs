use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;


pub struct PostDB {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub body: String,
    pub permalink: String,
    pub author: i64,
    pub author_name: String,
    pub translator: i64,
    pub translator_name: String,
    pub date: String,
    pub date_trans: String,
}

pub fn s_post_by_lang_permalink(
    lang_code: String,
    permalink_value: String
) -> Option<PostDB> {
    let mut post_struct: Option<PostDB> = None;

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(rows) = client.query(
            "SELECT * FROM s_post_by_lang_permalink(s_language_id_by_code($1), $2)",
            &[&lang_code, &permalink_value]
        ) {
            for row in rows {
                let post_id: i64 = row.get("tp_id");
                let post_title: String = row.get("tp_title");
                let post_description: String = row.get("tp_description");
                let post_body: String = row.get("tp_body");
                let post_permalink: String = row.get("tp_permalink");
                let post_author: i64 = row.get("tp_author");
                let post_author_name: String = row.get("tp_author_name");
                let post_translator: i64 = row.get("tp_translator");
                let post_translator_name: String = row.get("tp_translator_name");
                let post_date: String = row.get("tp_date");
                let post_date_trans: String = row.get("tp_date_trans");

                post_struct = Some(
                    PostDB {
                        id: post_id,
                        title: post_title,
                        description: post_description,
                        body: post_body,
                        permalink: post_permalink,
                        author: post_author,
                        author_name: post_author_name,
                        translator: post_translator,
                        translator_name: post_translator_name,
                        date: post_date,
                        date_trans: post_date_trans,
                    }
                );
            }
        }
        // TODO: Control the error!
    }

    post_struct
}

