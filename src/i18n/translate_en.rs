//use crate::i18n::translate_en::translate_en;


pub fn translate_en(text_value: &str) -> &str {
    match text_value {
        "(website_name)" => "MyExample",
        "{n} posts" => "{n} posts",
        "{n} result of {m}" => "{n} result of {m}",
        "{n} results of {m}" => "{n} results of {m}",
        "{name} © {year} [copyright]" => "{name} © {year}",
        "Account" => "Account",
        "Add language" => "Add language",
        "Are you sure that you want to delete this language?" =>
            "Are you sure that you want to delete this language?",
        "Are you sure that you want to delete this post?" =>
            "Are you sure that you want to delete this post?",
        "Are you sure that you want to delete this user?" =>
            "Are you sure that you want to delete this user?",
        "Author" => "Author",
        "Autocomplete for:" => "Autocomplete for:",
        "Blog" => "Blog",
        "Cancel" => "Cancel",
        "Code" => "Code",
        "Dashboard" => "Dashboard",
        "Data" => "Data",
        "Date" => "Date",
        "Delete" => "Delete",
        "Delete (permanent)" => "Delete (permanent)",
        "Deleted [post]" => "Deleted",
        "Deleted posts" => "Deleted posts",
        "Delete language: {name}" => "Delete language: {name}",
        "Delete post: '{title}'" => "Delete post: '{title}'",
        "Delete user: {name}" => "Delete user: {name}",
        "Description" => "Description",
        "Documentation" => "Documentation",
        "Draft" => "Draft",
        "Draft posts" => "Draft posts",
        "Drafts" => "Drafts",
        "Edit" => "Edit",
        "Edit language: {name}" => "Edit language: {name}",
        "Edit post: '{title}'" => "Edit post: '{title}'",
        "Edit user: '{name}'" => "Edit user: '{name}'",
        "Email" => "Email",
        "Error" => "Error",
        "ERROR:email_already_exists" => "That email is already in use.",
        "ERROR:field_is_not_lang_code" => "The language code is not valid.",
        "ERROR:lang_code_already_exists" =>
            "That language code already exists.",
        "ERROR:passwords_do_not_match" => "Passwords do not match.",
        "ERROR:some_wrong_i18n_user_name" =>
            "Some of the user names for each language are not valid.",
        "ERROR:some_wrong_lang_id_of_name" =>
            "Some of the language IDs are not correct.",
        "ERROR:some_wrong_lang_name" =>
            "Some of the language names are not valid.",
        "ERROR:some_wrong_name_for_lang" =>
            "Some of the names (in the new language) are not valid.",
        "ERROR:unknown_error" => "Unknown error.",
        "ERROR:user_not_logged_in" => "You must be logged in.",
        "ERROR:wrong_body_text" => "Body text can't be empty.",
        "ERROR:wrong_description" =>
            "Description can't start or begin with empty space; \
            it can't have more than 600 characters either.",
        "ERROR:wrong_email" => "That email is not valid.",
        "ERROR:wrong_lang_code" => "The language code is not correct.",
        "ERROR:wrong_lang_id" => "The language ID is not correct.",
        "ERROR:wrong_own_lang_name" =>
            "The language name (in the new language) is not valid.",
        "ERROR:wrong_password" => "That password is not valid; \
                                   it should have 8 or more characters.",
        "ERROR:wrong_permalink" => "The permalink is not valid.",
        "ERROR:wrong_post_id" => "The post ID is not correct.",
        "ERROR:wrong_post_permalink" => "That post doesn't exist.",
        "ERROR:wrong_title" => "The title is not valid.",
        "ERROR:wrong_user_email" => "No user has that email.",
        "ERROR:wrong_user_name" => "That user name is not valid.",
        "ERROR:wrong_user_password" => "The password is not correct.",
        "Examples: English, Español..." => "Examples: English, Español...",
        "Examples: en, en-us..." => "Examples: en, en-us...",
        "Files" => "Files",
        "Forgotten password?" => "Forgotten password?",
        "General" => "General",
        "Hello, {name}." => "Hello, {name}.",
        "Help [noun]" => "Help",
        "I hope you are having a great day!" =>
            "I hope you are having a great day!",
        "LANG:en" => "English",
        "LANG:es" => "Spanish",
        "LANG:ja" => "Japanese",
        "Language" => "Language",
        "Language name" => "Language name",
        "Language names" => "Language names",
        "Languages" => "Languages",
        "Last update" => "Last update",
        "Login [noun]" => "Login",
        "Login [verb]" => "Login",
        "Logout [verb]" => "Logout",
        "Name" => "Name",
        "Name (in the new language)" => "Name (in the new language)",
        "Names for each language" => "Names for each language",
        "Names (in the new language) for each language" =>
            "Names (in the new language) for each language",
        "New post" => "New post",
        "New user" => "New user",
        "Next [page]" => "Next",
        "optional" => "optional",
        "Original author" => "Original author",
        "Page {n}" => "Page {n}",
        "Pages" => "Pages",
        "Password" => "Password",
        "Permalink" => "Permalink",
        "Posts" => "Posts",
        "Post's body" => "Post's body",
        "Previous [page]" => "Previous",
        "Published" => "Published",
        "Published [posts]" => "Published",
        "Published posts" => "Published posts",
        "Read more" => "Read more",
        "Repeat password" => "Repeat password",
        "Scheduled [posts]" => "Scheduled",
        "Server" => "Server",
        "Sessions" => "Sessions",
        "Settings" => "Settings",
        "Sign up [verb]" => "Sign up",
        "Statistics" => "Statistics",
        "Status" => "Status",
        "Submit" => "Submit",
        "The post will be permanent deleted." =>
            "The post will be permanent deleted.",
        "The post will be sent to trash." =>
            "The post will be sent to trash.",
        "There are languages without names." =>
            "There are languages without names.",
        "Title" => "Title",
        "translated by {name}" => "translated by {name}",
        "Trash" => "Trash",
        "Tukosmo" => "Tukosmo",
        "Tukosmo Admin Panel" => "Tukosmo Admin Panel",
        "Untranslated" => "Untranslated",
        "Untranslated [posts]" => "Untranslated",
        "Untranslated posts" => "Untranslated posts",
        "untranslated" => "untranslated",
        "Users" => "Users",
        "Visit website" => "Visit website",
        "Website" => "Website",
        "Your email" => "Your email",
        "Your password" => "Your password",
        "Your website languages were successfully updated." =>
            "Your website languages were successfully updated.",
        "Your website posts were successfully updated." =>
            "Your website posts were successfully updated.",
        "Your website users were successfully updated." =>
            "Your website users were successfully updated.",

        //--------------//

        _ => "[untranslated]"  // TODO: Include text_value string
    }
}

