use markup;


markup::define! {
    Sidebar() {
        aside[class = "site-sidebar"] {
            @Widget {}
        }
    }

    Widget() {
        section[class = "widget widget-html"] {
            "Made with Tukosmo"
        }
    }
}

