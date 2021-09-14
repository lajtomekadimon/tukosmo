use postgres::row::Row;

use crate::database::data::{NameDB, LanguageDB};


pub fn names(
    rows: Vec<Row>
) -> Vec<NameDB> {
    let mut vec = Vec::new();

    for row in rows {
        let name: String = row.get("name");
        let name_lang_id: i64 = row.get("lang_id");
        let name_lang_code: String = row.get("lang_code");
        let name_lang_name: String = row.get("lang_name");
        let name_lang_date: String = row.get("lang_date");
        let name_lang_has_all_names: bool = row.get("lang_has_all_names");

        vec.push(
            NameDB {
                name: name,
                lang: LanguageDB {
                    id: name_lang_id,
                    code: name_lang_code,
                    name: name_lang_name,
                    date: name_lang_date,
                    has_all_names: name_lang_has_all_names,
                },
            }
        );                
    }

    vec
}

