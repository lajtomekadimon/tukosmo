use crate::i18n::translate_en::translate_en;


pub fn translate_es(text_value: &str) -> &str {
    match text_value {
        "(website_name)" => "MyExample",
        "Account" => "Cuenta",
        "Blog" => "Blog",
        "Dashboard" => "Pantalla principal",
        "Data" => "Datos",
        "Documentation" => "Documentación",
        "Drafts" => "Borradores",
        "Files" => "Archivos",
        "Forgotten password?" => "¿Contraseña olvidada?",
        "General" => "General",
        "Hello, {name}." => "Hola, {name}.",
        "Help [noun]" => "Ayuda",
        "I hope you are having a great day!" => "¡Espero que tengas un buen día!",
        "Languages" => "Idiomas",
        "Login [noun]" => "Inicio de sesión",
        "Login [verb]" => "Iniciar sesión",
        "Logout [verb]" => "Cerrar sesión",
        "Pages" => "Páginas",
        "Posts" => "Entradas",
        "Published [posts]" => "Publicadas",
        "Read more" => "Leer más",
        "Scheduled [posts]" => "Programadas",
        "Server" => "Servidor",
        "Sessions" => "Sesiones",
        "Settings" => "Ajustes",
        "Sign up [verb]" => "Registrarse",
        "Statistics" => "Estadísticas",
        "Trash" => "Papelera",
        "Tukosmo" => "Tukosmo",
        "Tukosmo Admin Panel" => "Panel de Administración de Tukosmo",
        "Untranslated [posts]" => "Sin traducir",
        "Users" => "Usuarios",
        "Visit website" => "Visitar página web",
        "Website" => "Página web",
        "Your email" => "Tu correo electrónico",
        "Your password" => "Tu contraseña",

        //--------------//

        _ => translate_en(text_value)
    }
}

