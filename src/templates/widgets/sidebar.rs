use markup;


markup::define! {
    Sidebar() {
        aside[
            class = "site-sidebar",
        ] {
            @Widget {}
            @Widget {}
            @Widget {}
            @Widget {}
            @Widget {}
        }
    }

    Widget() {
        section[
            class = "widget widget-html",
        ] {
            "Bla"
        }
    }
}

