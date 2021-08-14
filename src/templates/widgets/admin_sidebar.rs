use markup;

use crate::i18n::t::t;


markup::define! {
    AdminSidebar<'a>(
        current_page: &'a str,
        lang_code: &'a str,
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
                    ] {
                        {&t("Dashboard", lang_code)}
                    }
                }
            }

            /* General
             * * * * * */
            p[class = "menu-label"] {
                {&t("General", lang_code)}
            }
            ul[class = "menu-list"] {

                li {
                    a[
                        href = "/en/admin/statistics",  // TODO
                        class = if current_page == &"statistics" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Statistics", lang_code)}
                    }
                }
                li {
                    a[
                        href = "/en/admin/server",  // TODO
                        class = if current_page == &"server" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Server", lang_code)}
                    }
                }
            }

            /* Data
             * * * * */
            p[class = "menu-label"] {
                {&t("Data", lang_code)}
            }
            ul[class = "menu-list"] {
                li {
                    a[
                        href = "/en/admin/users",  // TODO
                        class = if current_page == &"users" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Users", lang_code)}
                    }
                }
                li {
                    a[
                        href = "/en/admin/languages",  // TODO
                        class = if current_page == &"languages" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Languages", lang_code)}
                    }
                }
                li {
                    a[
                        href = "/en/admin/posts",  // TODO
                        class = if current_page == &"posts" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Posts", lang_code)}
                    }
                    ul {
                        li {
                            a[
                                href = "/en/admin/posts",
                            ] {
                                {&t("Drafts", lang_code)}
                            }
                        }
                        li {
                            a[
                                href = "/en/admin/posts",
                            ] {
                                {&t("Scheduled [posts]", lang_code)}
                            }
                        }
                        li {
                            a[
                                href = "/en/admin/posts",
                            ] {
                                {&t("Published [posts]", lang_code)}
                            }
                        }
                        li {
                            a[
                                href = "/en/admin/posts",
                            ] {
                                {&t("Untranslated [posts]", lang_code)}
                            }
                        }
                        li {
                            a[
                                href = "/en/admin/posts",
                            ] {
                                {&t("Trash", lang_code)}
                            }
                        }
                    }
                }
                li {
                    a[
                        href = "/en/admin/pages",  // TODO
                        class = if current_page == &"pages" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Pages", lang_code)}
                    }
                }
                li {
                    a[
                        href = "/en/admin/files",  // TODO
                        class = if current_page == &"files" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Files", lang_code)}
                    }
                }
            }

            /* Settings
             * * * * * * */
            p[class = "menu-label"] {
                {&t("Settings", lang_code)}
            }
            ul[class = "menu-list"] {
                li {
                    a[
                        href = "/en/admin/website",  // TODO
                        class = if current_page == &"website" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Website", lang_code)}
                    }
                }
                li {
                    a[
                        href = "/en/admin/tukosmo",  // TODO
                        class = if current_page == &"tukosmo" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Tukosmo", lang_code)}
                    }
                }
            }

        }
    }
}

