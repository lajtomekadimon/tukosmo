use markup;


markup::define! {
    AdminSidebar<'a>(
        current_page: &'a str,
    ) {
        aside[class = "menu is-hidden-mobile"] {
            /* Dashboard
             * * * * * * */
            ul[class = "menu-list"] {
                li {
                    a[
                        href = "/en/admin/",  // TODO
                        class = if current_page == &"dashboard" {
                            "is-active"
                        } else { "" },
                    ] { "Dashboard" }
                }
            }

            /* General
             * * * * * */
            p[class = "menu-label"] {
                "General"
            }
            ul[class = "menu-list"] {

                li {
                    a[
                        href = "/en/admin/statistics",  // TODO
                        class = if current_page == &"statistics" {
                            "is-active"
                        } else { "" },
                    ] { "Statistics" }
                }
                li {
                    a[
                        href = "/en/admin/server",  // TODO
                        class = if current_page == &"server" {
                            "is-active"
                        } else { "" },
                    ] { "Server" }
                }
            }

            /* Data
             * * * * */
            p[class = "menu-label"] {
                "Data"
            }
            ul[class = "menu-list"] {
                li {
                    a[
                        href = "/en/admin/users",  // TODO
                        class = if current_page == &"users" {
                            "is-active"
                        } else { "" },
                    ] { "Users" }
                }
                li {
                    a[
                        href = "/en/admin/languages",  // TODO
                        class = if current_page == &"languages" {
                            "is-active"
                        } else { "" },
                    ] { "Languages" }
                }
                li {
                    a[
                        href = "/en/admin/posts",  // TODO
                        class = if current_page == &"posts" {
                            "is-active"
                        } else { "" },
                    ] { "Posts" }
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
                    a[
                        href = "/en/admin/pages",  // TODO
                        class = if current_page == &"pages" {
                            "is-active"
                        } else { "" },
                    ] { "Pages" }
                }
                li {
                    a[
                        href = "/en/admin/files",  // TODO
                        class = if current_page == &"files" {
                            "is-active"
                        } else { "" },
                    ] { "Files" }
                }
            }

            /* Settings
             * * * * * * */
            p[class = "menu-label"] {
                "Settings"
            }
            ul[class = "menu-list"] {
                li {
                    a[
                        href = "/en/admin/website",  // TODO
                        class = if current_page == &"website" {
                            "is-active"
                        } else { "" },
                    ] { "Website" }
                }
                li {
                    a[
                        href = "/en/admin/tukosmo",  // TODO
                        class = if current_page == &"tukosmo" {
                            "is-active"
                        } else { "" },
                    ] { "Tukosmo" }
                }
            }

        }
    }
}

