use crate::database::types::NameDB;


pub fn get_name_from_names(
    lang_id: i64,
    names: &Vec<NameDB>,
) -> String {
    let mut the_name: String = "".to_string();

    for name in names.iter() {
        if name.lang.id == lang_id {
            the_name = name.name.clone();
            break;
        }
    }

    the_name
}

