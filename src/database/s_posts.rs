use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;


pub struct PostDB {
    pub id: i64,
    pub trans_id: i64,
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
    pub has_all_trans: bool,
    pub draft: bool,
    pub untranslated: bool,
}

pub fn s_posts(
    lang_code: String
) -> Vec<PostDB> {
    let mut vec = Vec::new();

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(rows) = client.query(
            "SELECT * FROM s_posts(s_language_id_by_code($1))",
            &[&lang_code,]
        ) {
            for row in rows {
                let post_id: i64 = row.get("id");
                let post_trans_id: i64 = row.get("trans_id");
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
                let post_has_all_trans: bool = row.get("has_all_trans");
                let post_draft: bool = row.get("draft");
                let post_untranslated: bool = row.get("untranslated");

                vec.push(
                    PostDB {
                        id: post_id,
                        trans_id: post_trans_id,
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
                        has_all_trans: post_has_all_trans,
                        draft: post_draft,
                        untranslated: post_untranslated,
                    }
                );                
            }
        }
        // TODO: Control the error!
    }

    vec
}

