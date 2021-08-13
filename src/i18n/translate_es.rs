

pub fn translate_es(text_value: &str) -> &str {
    match text_value {
        "(website_name)" => "MyExample",
        "Blog" => "Blog",
        "Tukosmo Admin Panel" => "Panel de Administración de Tukosmo",

        //--------------//

        _ => "[sin traducir]"
    }
}

