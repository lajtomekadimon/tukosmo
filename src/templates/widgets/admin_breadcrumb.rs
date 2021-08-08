use markup;


markup::define! {
    AdminBreadcrumb() {
        nav[class = "breadcrumb"] {
            ul {
                li {
                    a[href = "#"] { "Bulma" }
                }
                li {
                    a[href = "#"] { "Templates" }
                }
                li {
                    a[href = "#"] { "Examples" }
                }
                li[class = "is-active"] {
                    a[href = "#"] { "Admin" }
                }
            }
        }
    }
}

