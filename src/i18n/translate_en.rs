

pub fn translate_en(text_value: &str) -> &str {
    match text_value {
        "(website_name)" => "MyExample",
        "Account" => "Account",
        "Add language" => "Add language",
        "Blog" => "Blog",
        "Cancel" => "Cancel",
        "Code" => "Code",
        "Dashboard" => "Dashboard",
        "Data" => "Data",
        "Documentation" => "Documentation",
        "Drafts" => "Drafts",
        "Edit language: '{lang}'" => "Edit language: '{lang}'",
        "Example: en" => "Example: en",
        "Files" => "Files",
        "Flag" => "Flag",
        "Forgotten password?" => "Forgotten password?",
        "General" => "General",
        "Hello, {name}." => "Hello, {name}.",
        "Help [noun]" => "Help",
        "I hope you are having a great day!" => "I hope you are having a great day!",
        "Language" => "Language",
        "Language name" => "Language name",
        "Languages" => "Languages",
        "Last update" => "Last update",
        "Login [noun]" => "Login",
        "Login [verb]" => "Login",
        "Logout [verb]" => "Logout",
        "Pages" => "Pages",
        "Posts" => "Posts",
        "Published [posts]" => "Published",
        "Read more" => "Read more",
        "Scheduled [posts]" => "Scheduled",
        "Server" => "Server",
        "Sessions" => "Sessions",
        "Settings" => "Settings",
        "Sign up [verb]" => "Sign up",
        "Statistics" => "Statistics",
        "Submit" => "Submit",
        "Trash" => "Trash",
        "Tukosmo" => "Tukosmo",
        "Tukosmo Admin Panel" => "Tukosmo Admin Panel",
        "Untranslated [posts]" => "Untranslated",
        "Users" => "Users",
        "Visit website" => "Visit website",
        "Website" => "Website",
        "Your email" => "Your email",
        "Your password" => "Your password",

        //--------------//

        _ => "[unstranslated]"
    }
}

