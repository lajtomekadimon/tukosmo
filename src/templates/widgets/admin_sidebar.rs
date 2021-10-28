use markup;

use crate::i18n::t::t;
use crate::database::types::AdminDataDB;


markup::define! {
    AdminSidebar<'a>(
        current_page: &'a str,
        data: &'a AdminDataDB,
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
                        i[class = "eos-icons"] { "dashboard" }
                        " "
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
                        i[class = "eos-icons"] { "bar_chart" }
                        " "
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
                        i[class = "eos-icons"] { "database" }
                        " "
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
                        i[class = "eos-icons"] { "supervisor_account" }
                        " "
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
                        i[class = "eos-icons"] { "translate" }
                        " "
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
                        i[class = "eos-icons"] { "article" }
                        " "
                        {&t("Posts", &data.lang.code)}
                    }

                    @if current_page == &"posts"
                        || current_page == &"posts-drafts"
                        || current_page == &"posts-published"
                        || current_page == &"posts-untranslated"
                        || current_page == &"posts-deleted"
                    {
                        ul {
                            li {
                                a[
                                    href = "/{lang}/admin/posts?f=drafts"
                                        .replace("{lang}", &data.lang.code),
                                    class = if
                                        current_page == &"posts-drafts"
                                    {
                                        "is-active"
                                    } else { "" },
                                ] {
                                    {&t("Drafts", &data.lang.code)}
                                }
                            }
                            li {
                                a[
                                    href = "/{lang}/admin/posts?f=published"
                                        .replace("{lang}", &data.lang.code),
                                    class = if
                                        current_page == &"posts-published"
                                    {
                                        "is-active"
                                    } else { "" },
                                ] {
                                    {&t("Published [posts]", &data.lang.code)}
                                }
                            }
                            li {
                                a[
                                    href = "/{lang}/admin/posts?f=untranslated"
                                        .replace("{lang}", &data.lang.code),
                                    class = if
                                        current_page == &"posts-untranslated"
                                    {
                                        "is-active"
                                    } else { "" },
                                ] {
                                    {&t(
                                        "Untranslated [posts]",
                                        &data.lang.code,
                                    )}
                                }
                            }
                            li {
                                a[
                                    href = "/{lang}/admin/posts?f=deleted"
                                        .replace("{lang}", &data.lang.code),
                                    class = if
                                        current_page == &"posts-deleted"
                                    {
                                        "is-active"
                                    } else { "" },
                                ] {
                                    {&t("Trash", &data.lang.code)}
                                }
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
                        i[class = "eos-icons"] { "description" }
                        " "
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
                        i[class = "eos-icons"] { "file_present" }
                        " "
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
                        i[class = "eos-icons"] { "web" }
                        " "
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
                        i[class = "eos-icons"] { "settings_suggest" }
                        " "
                        {&t("Tukosmo", &data.lang.code)}
                    }
                }
            }

        }
    }
}

