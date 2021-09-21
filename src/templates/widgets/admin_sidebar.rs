use markup;

use crate::i18n::t::t;
use crate::database::types::DataDB;


markup::define! {
    AdminSidebar<'a>(
        current_page: &'a str,
        data: &'a DataDB,
    ) {
        aside[class = "menu is-hidden-mobile"] {
            /* Dashboard
             * * * * * * */
            ul[class = "menu-list"] {
                li {
                    a[
                        href = "/{lang}/admin/"
                            .replace("{lang}", &data.lang.code),
                        class = if current_page == &"dashboard" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Dashboard", &data.lang.code)}
                    }
                }
            }

            /* General
             * * * * * */
            p[class = "menu-label"] {
                {&t("General", &data.lang.code)}
            }
            ul[class = "menu-list"] {

                li {
                    a[
                        href = "/{lang}/admin/statistics"
                            .replace("{lang}", &data.lang.code),
                        class = if current_page == &"statistics" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Statistics", &data.lang.code)}
                    }
                }
                li {
                    a[
                        href = "/{lang}/admin/server"
                            .replace("{lang}", &data.lang.code),
                        class = if current_page == &"server" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Server", &data.lang.code)}
                    }
                }
            }

            /* Data
             * * * * */
            p[class = "menu-label"] {
                {&t("Data", &data.lang.code)}
            }
            ul[class = "menu-list"] {
                li {
                    a[
                        href = "/{lang}/admin/users"
                            .replace("{lang}", &data.lang.code),
                        class = if current_page == &"users" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Users", &data.lang.code)}
                    }
                }
                li {
                    a[
                        href = "/{lang}/admin/languages"
                            .replace("{lang}", &data.lang.code),
                        class = if current_page == &"languages" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Languages", &data.lang.code)}
                    }
                }
                li {
                    a[
                        href = "/{lang}/admin/posts"
                            .replace("{lang}", &data.lang.code),
                        class = if current_page == &"posts" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Posts", &data.lang.code)}
                    }
                    ul {
                        li {
                            a[
                                href = "/{lang}/admin/posts"
                                    .replace("{lang}", &data.lang.code),
                            ] {
                                {&t("Drafts", &data.lang.code)}
                            }
                        }
                        li {
                            a[
                                href = "/{lang}/admin/posts"
                                    .replace("{lang}", &data.lang.code),
                            ] {
                                {&t("Published [posts]", &data.lang.code)}
                            }
                        }
                        li {
                            a[
                                href = "/{lang}/admin/posts"
                                    .replace("{lang}", &data.lang.code),
                            ] {
                                {&t("Untranslated [posts]", &data.lang.code)}
                            }
                        }
                        li {
                            a[
                                href = "/{lang}/admin/posts"
                                    .replace("{lang}", &data.lang.code),
                            ] {
                                {&t("Trash", &data.lang.code)}
                            }
                        }
                    }
                }
                li {
                    a[
                        href = "/{lang}/admin/pages"
                            .replace("{lang}", &data.lang.code),
                        class = if current_page == &"pages" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Pages", &data.lang.code)}
                    }
                }
                li {
                    a[
                        href = "/{lang}/admin/files"
                            .replace("{lang}", &data.lang.code),
                        class = if current_page == &"files" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Files", &data.lang.code)}
                    }
                }
            }

            /* Settings
             * * * * * * */
            p[class = "menu-label"] {
                {&t("Settings", &data.lang.code)}
            }
            ul[class = "menu-list"] {
                li {
                    a[
                        href = "/{lang}/admin/website"
                            .replace("{lang}", &data.lang.code),
                        class = if current_page == &"website" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Website", &data.lang.code)}
                    }
                }
                li {
                    a[
                        href = "/{lang}/admin/tukosmo"
                            .replace("{lang}", &data.lang.code),
                        class = if current_page == &"tukosmo" {
                            "is-active"
                        } else { "" },
                    ] {
                        {&t("Tukosmo", &data.lang.code)}
                    }
                }
            }

        }
    }
}

