use markup;


markup::define! {
    AdminSidebar() {
        aside[class = "menu is-hidden-mobile"] {

            /* General
             * * * * * */
            p[class = "menu-label"] {
                "General"
            }
            ul[class = "menu-list"] {

                li {
                    a[class = "is-active"] { "Dashboard" }
                }
                li {
                    a { "Statistics" }
                }
                li {
                    a { "Server" }
                }
            }

            /* Data
             * * * * */
            p[class = "menu-label"] {
                "Data"
            }
            ul[class = "menu-list"] {
                li {
                    a { "Users" }
                }
                li {
                    a { "Posts" }
                    ul {
                        li {
                            a { "Drafts" }
                        }
                        li {
                            a { "Scheduled" }
                        }
                        li {
                            a { "Published" }
                        }
                        li {
                            a { "Untranslated" }
                        }
                        li {
                            a { "Trash" }
                        }
                    }
                }
                li {
                    a { "Pages" }
                }
                li {
                    a { "Files" }
                }
            }

            /* Settings
             * * * * * * */
            p[class = "menu-label"] {
                "Settings"
            }
            ul[class = "menu-list"] {
                li {
                    a { "Website" }
                }
                li {
                    a { "Tukosmo" }
                }
            }

        }
    }
}

