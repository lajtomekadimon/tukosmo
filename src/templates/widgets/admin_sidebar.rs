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
                        href = "/{lang}/admin/".replace("{lang}", &lang_code),
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
                        href = "/{lang}/admin/statistics".replace("{lang}", &lang_code),
                        class = if current_page == &"statistics" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Statistics", lang_code)}
                    }
                }
                li {
                    a[
                        href = "/{lang}/admin/server".replace("{lang}", &lang_code),
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
                        href = "/{lang}/admin/users".replace("{lang}", &lang_code),
                        class = if current_page == &"users" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Users", lang_code)}
                    }
                }
                li {
                    a[
                        href = "/{lang}/admin/languages".replace("{lang}", &lang_code),
                        class = if current_page == &"languages" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Languages", lang_code)}
                    }
                }
                li {
                    a[
                        href = "/{lang}/admin/posts".replace("{lang}", &lang_code),
                        class = if current_page == &"posts" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Posts", lang_code)}
                    }
                    ul {
                        li {
                            a[
                                href = "/{lang}/admin/posts".replace("{lang}", &lang_code),
                            ] {
                                {&t("Drafts", lang_code)}
                            }
                        }
                        li {
                            a[
                                href = "/{lang}/admin/posts".replace("{lang}", &lang_code),
                            ] {
                                {&t("Published [posts]", lang_code)}
                            }
                        }
                        li {
                            a[
                                href = "/{lang}/admin/posts".replace("{lang}", &lang_code),
                            ] {
                                {&t("Untranslated [posts]", lang_code)}
                            }
                        }
                        li {
                            a[
                                href = "/{lang}/admin/posts".replace("{lang}", &lang_code),
                            ] {
                                {&t("Trash", lang_code)}
                            }
                        }
                    }
                }
                li {
                    a[
                        href = "/{lang}/admin/pages".replace("{lang}", &lang_code),
                        class = if current_page == &"pages" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Pages", lang_code)}
                    }
                }
                li {
                    a[
                        href = "/{lang}/admin/files".replace("{lang}", &lang_code),
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
                        href = "/{lang}/admin/website".replace("{lang}", &lang_code),
                        class = if current_page == &"website" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Website", lang_code)}
                    }
                }
                li {
                    a[
                        href = "/{lang}/admin/tukosmo".replace("{lang}", &lang_code),
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

