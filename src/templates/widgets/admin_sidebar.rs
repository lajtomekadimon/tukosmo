use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::AdminDataDB;


markup::define! {
    AdminSidebar<'a>(
        current_page: &'a str,
        data: &'a AdminDataDB,
        t: &'a TranslateI18N,
    ) {
        aside[
            id = "smenu",
            class = "menu is-hidden-mobile",
        ] {
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
                        @t.dashboard
                    }
                }
            }

            /* General
             * * * * * */
            /*
            p[class = "menu-label"] {
                @t.general
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
                        @t.statistics
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
                        @t.server
                    }
                }
            }
            */

            /* Data
             * * * * */
            p[class = "menu-label"] {
                @t.data
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
                        @t.users
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
                        @t.languages
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
                        @t.posts
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
                                    @t.drafts
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
                                    @t.published_k_posts
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
                                    @t.untranslated_k_posts
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
                                    @t.trash
                                }
                            }
                        }
                    }
                }
                /*
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
                        @t.pages
                    }
                }
                */
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
                        @t.files
                    }
                }
            }

            /* Appearance
             * * * * * */
            p[class = "menu-label"] {
                @t.appearance
            }
            ul[class = "menu-list"] {
                li {
                    a[
                        href = "/{lang}/admin/favicon"
                            .replace("{lang}", &data.lang.code),
                        class = if current_page == &"favicon" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[class = "eos-icons"] { "photo" }
                        " "
                        @t.favicon
                    }
                }

                /*
                li {
                    a[
                        href = "/{lang}/admin/theme"
                            .replace("{lang}", &data.lang.code),
                        class = if current_page == &"theme" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[class = "eos-icons"] { "devices" }
                        " "
                        @t.theme
                    }
                }

                li {
                    a[
                        href = "/{lang}/admin/menus"
                            .replace("{lang}", &data.lang.code),
                        class = if current_page == &"menus" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[class = "eos-icons"] { "list" }
                        " "
                        @t.menus
                    }
                }

                li {
                    a[
                        href = "/{lang}/admin/widgets"
                            .replace("{lang}", &data.lang.code),
                        class = if current_page == &"widgets" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[class = "eos-icons"] { "view_sidebar" }
                        " "
                        @t.widgets
                    }
                }
                */
            }

            /* Settings
             * * * * * * */
            p[class = "menu-label"] {
                @t.settings
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
                        @t.website
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
                        @t.tukosmo
                    }
                }
            }

        }
    }
}

