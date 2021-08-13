

pub fn translate_en(text_value: &str) -> &str {
    match text_value {
        "(website_name)" => "MyExample",
        "Blog" => "Blog",
        "Tukosmo Admin Panel" => "Tukosmo Admin Panel",

        //--------------//

        _ => "[unstranslated]"
    }
}

