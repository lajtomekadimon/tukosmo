use postgres::row::Row;

use crate::database::data::LanguageDB;


pub fn languages(
    rows: Vec<Row>
) -> Vec<LanguageDB> {
    let mut vec = Vec::new();

    for row in rows {
        let lang_id: i64 = row.get("id");
        let lang_code: String = row.get("code");
        let lang_name: String = row.get("name");
        let date_value: String = row.get("date");
        let has_all_names: bool = row.get("has_all_names");

        vec.push(
            LanguageDB {
                id: lang_id,
                code: lang_code,
                name: lang_name,
                date: date_value,
                has_all_names: has_all_names,
            }
        );                
    }

    vec
}

