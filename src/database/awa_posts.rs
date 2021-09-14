use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::data::PostDB;


pub fn awa_posts(
    language_of_user: i64
) -> Vec<PostDB> {
    let mut vec = Vec::new();

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(rows) = client.query(
            "SELECT * FROM awa_posts($1)",
            &[&language_of_user,]
        ) {
            for row in rows {
                let post_id: i64 = row.get("id");
                let post_trans_id: i64 = row.get("trans_id");
                let post_lang: i64 = row.get("lang");
                let post_title: String = row.get("title");
                let post_description: String = row.get("description");
                let post_body: String = row.get("body");
                let post_permalink: String = row.get("permalink");
                let post_author: i64 = row.get("author");
                let post_author_name: String = row.get("author_name");
                let post_translator: i64 = row.get("translator");
                let post_translator_name: String = row.get("translator_name");
                let post_date: String = row.get("date");
                let post_date_trans: String = row.get("date_trans");
                let post_draft: bool = row.get("draft");
                let post_deleted: bool = row.get("deleted");

                vec.push(
                    PostDB {
                        id: post_id,
                        trans_id: post_trans_id,
                        lang: post_lang,
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
                        draft: post_draft,
                        deleted: post_deleted,
                    }
                );                
            }
        }
        // TODO: Control the error!
    }

    vec
}

