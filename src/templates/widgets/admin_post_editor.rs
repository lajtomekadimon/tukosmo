use markup;


markup::define! {
    AdminPostEditor<'a>(
        name: &'a str,
        init_value: &'a str,
    ) {

        input[
            id = "body-text",
            type = "hidden",
            name = name,
            value = init_value,
        ];

        // TODO: FOOTNOTES DON'T WORK IN WYSIWYG MODE!!!
        div[id = "editor"] {}

        // TODO: i18n .js files
        // https://github.com/nhn/tui.editor/blob/master/docs/en/i18n.md
        // TODO: LaTeX, videos, etc.
        // https://github.com/nhn/tui.editor/blob/master/docs/en/custom-block.md

    }
}

