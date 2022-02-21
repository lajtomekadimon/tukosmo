use markup;


markup::define! {
    AdminPostEditor<'a>(
        name: &'a str,
        init_value: &'a str,
    ) {
        
        textarea[
            id = "editor",
            name = name,
        ] {
            @init_value
        }

    }
}

