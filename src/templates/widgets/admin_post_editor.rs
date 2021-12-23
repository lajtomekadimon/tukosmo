use markup;

use crate::files::static_files;


markup::define! {
    AdminPostEditor<'a>(
        name: &'a str,
        init_value: &'a str,
    ) {

        link[
            rel = "stylesheet",
            // TODO: Use a local file
            href = "https://uicdn.toast.com/editor/latest/toastui-editor.min.css",
        ];

        input[
            id = "body-text",
            type = "hidden",
            name = name,
            value = init_value,
        ];

        div[id = "editor"] {}

        script[
            // TODO: Use a local file
            src = "https://uicdn.toast.com/editor/latest/toastui-editor-all.min.js",
        ] {}

        script[
            src = static_files::JS_EDITOR,
        ] {}

    }
}

