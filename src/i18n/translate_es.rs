use crate::i18n::translate_en::translate_en;


pub fn translate_es(text_value: &str) -> &str {
    match text_value {
        "(website_name)" => "MyExample",
        "{n} posts" => "{n} entradas",
        "{n} result of {m}" => "{n} resultado de {m}",
        "{n} results of {m}" => "{n} resultados de {m}",
        "{name} © {year} [copyright]" => "{name} © {year}",
        "Account" => "Cuenta",
        "Add language" => "Añadir idioma",
        "Are you sure that you want to delete this language?" =>
            "¿Estás seguro de que quieres eliminar este idioma?",
        "Are you sure that you want to delete this post?" =>
            "¿Estás seguro de que quieres eliminar esta entrada?",
        "Author" => "Autor",
        "Autocomplete for:" => "Autocompletar para:",
        "Blog" => "Blog",
        "Cancel" => "Cancelar",
        "Code" => "Código",
        "Dashboard" => "Pantalla principal",
        "Data" => "Datos",
        "Date" => "Fecha",
        "Delete" => "Eliminar",
        "Deleted [post]" => "Eliminada",
        "Deleted posts" => "Entradas eliminadas",
        "Delete language: {name}" => "Eliminar idioma: {name}",
        "Delete post: '{title}'" => "Eliminar entrada: '{title}'",
        "Description" => "Descripción",
        "Documentation" => "Documentación",
        "Draft" => "Borrador",
        "Draft posts" => "Borradores de entradas",
        "Drafts" => "Borradores",
        "Edit" => "Editar",
        "Edit language: {name}" => "Editar idioma: {name}",
        "Edit post: '{title}'" => "Editar entrada: '{title}'",
        "Edit user: '{name}'" => "Editar usuario: '{name}'",
        "Email" => "Correo electrónico",
        "Error" => "Error",
        "ERROR:email_already_exists" => "Ese correo electrónico ya está en \
                                        uso.",
        "ERROR:field_is_not_lang_code" => "El código del idioma no es válido.",
        "ERROR:lang_code_already_exists" => "Ese código de idioma ya existe.",
        "ERROR:passwords_do_not_match" => "Las contraseñas no coinciden.",
        "ERROR:some_wrong_i18n_user_name" => "Algunos de los nombres del \
                                              usuario para cada idioma no son \
                                              válidos.",
        "ERROR:some_wrong_lang_id_of_name" =>
            "Algunas de las ID de los idiomas no son correctas.",
        "ERROR:some_wrong_lang_name" =>
            "Algunos de los nombres del idioma no son válidos.",
        "ERROR:some_wrong_name_for_lang" =>
            "Algunos de los nombres (en el nuevo idioma) no son válidos.",
        "ERROR:unknown_error" => "Error desconocido.",
        "ERROR:user_not_logged_in" => "Debes estar conectado con tu usuario.",
        "ERROR:wrong_body_text" => "El texto del cuerpo no puede estar vacío.",
        "ERROR:wrong_description" =>
            "La descripción no puede empezar o terminar en espacios; \
            tampoco puede exceder los 600 caracteres.",
        "ERROR:wrong_email" => "Ese correo electrónico no es válido.",
        "ERROR:wrong_lang_code" => "El código del idioma no es correcto.",
        "ERROR:wrong_lang_id" => "La ID del idioma no es correcta.",
        "ERROR:wrong_own_lang_name" =>
            "El nombre del idioma (en el nuevo idioma) no es válido.",
        "ERROR:wrong_password" => "Esa contraseña no es válida; \
                                   debe tener 8 o más caracteres.",
        "ERROR:wrong_permalink" => "El permalink no es válido.",
        "ERROR:wrong_post_id" => "La ID de la entrada no es correcta.",
        "ERROR:wrong_post_permalink" => "Esa entrada no existe.",
        "ERROR:wrong_title" => "El título no es válido.",
        "ERROR:wrong_user_email" => "Ningún usuario tiene ese email.",
        "ERROR:wrong_user_name" => "Ese nombre de usuario no es válido.",
        "ERROR:wrong_user_password" => "La contraseña no es correcta.",
        "Examples: English, Español..." => "Ejemplos: English, Español...",
        "Examples: en, en-us..." => "Ejemplos: es, es-es...",
        "Files" => "Archivos",
        "Forgotten password?" => "¿Contraseña olvidada?",
        "General" => "General",
        "Hello, {name}." => "Hola, {name}.",
        "Help [noun]" => "Ayuda",
        "I hope you are having a great day!" =>
            "¡Espero que tengas un buen día!",
        "LANG:en" => "Inglés",
        "LANG:es" => "Español",
        "LANG:ja" => "Japonés",
        "Language" => "Idioma",
        "Language name" => "Nombre del idioma",
        "Language names" => "Nombres del idioma",
        "Languages" => "Idiomas",
        "Last update" => "Última actualización",
        "Login [noun]" => "Inicio de sesión",
        "Login [verb]" => "Iniciar sesión",
        "Logout [verb]" => "Cerrar sesión",
        "Name" => "Nombre",
        "Name (in the new language)" => "Nombre (en el nuevo idioma)",
        "Names for each language" => "Nombres para cada idioma",
        "Names (in the new language) for each language" =>
            "Nombres (en el nuevo idioma) para cada idioma",
        "New post" => "Nueva entrada",
        "New user" => "Nuevo usuario",
        "Next [page]" => "Siguiente",
        "optional" => "opcional",
        "Original author" => "Autor original",
        "Page {n}" => "Página {n}",
        "Pages" => "Páginas",
        "Password" => "Contraseña",
        "Permalink" => "Permalink",
        "Posts" => "Entradas",
        "Post's body" => "Cuerpo de la entrada",
        "Previous [page]" => "Anterior",
        "Published" => "Publicado",
        "Published [posts]" => "Publicadas",
        "Published posts" => "Entradas publicadas",
        "Read more" => "Leer más",
        "Repeat password" => "Repite la contraseña",
        "Scheduled [posts]" => "Programadas",
        "Server" => "Servidor",
        "Sessions" => "Sesiones",
        "Settings" => "Ajustes",
        "Sign up [verb]" => "Registrarse",
        "Statistics" => "Estadísticas",
        "Status" => "Estado",
        "Submit" => "Enviar",
        "There are languages without names." =>
            "Hay idiomas sin nombres.",
        "Title" => "Título",
        "translated by {name}" => "traducido por {name}",
        "Trash" => "Papelera",
        "Tukosmo" => "Tukosmo",
        "Tukosmo Admin Panel" => "Panel de Administración de Tukosmo",
        "Untranslated" => "Sin traducir",
        "Untranslated [posts]" => "Sin traducir",
        "Untranslated posts" => "Entradas sin traducir",
        "untranslated" => "sin traducir",
        "Users" => "Usuarios",
        "Visit website" => "Visitar página web",
        "Website" => "Página web",
        "Your email" => "Tu correo electrónico",
        "Your password" => "Tu contraseña",
        "Your website languages were successfully updated." =>
            "Los idiomas de tu página web se actualizaron con éxito.",
        "Your website posts were successfully updated." =>
            "Las entradas de tu página web se actualizaron con éxito.",
        "Your website users were successfully updated." =>
            "Los usuarios de tu página web se actualizaron con éxito.",

        //--------------//

        _ => translate_en(text_value)
    }
}

