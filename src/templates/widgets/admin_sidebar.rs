use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::AdminDataDB;
use crate::handlers::admin::dashboard::ra_dashboard;
use crate::handlers::admin::statistics::ra_statistics;
use crate::handlers::admin::server::ra_server;
use crate::handlers::admin::users::ra_users;
use crate::handlers::admin::languages::ra_languages;
use crate::handlers::admin::posts::{ra_posts, ra_posts_w_f};
use crate::handlers::admin::pages::ra_pages;
use crate::handlers::admin::files::ra_files;
use crate::handlers::admin::favicon::ra_favicon;
use crate::handlers::admin::website::ra_website;
use crate::handlers::admin::tukosmo::ra_tukosmo;


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
                        href = &ra_dashboard(&data.lang.code),
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
            p[class = "menu-label"] {
                @t.general
            }
            ul[class = "menu-list"] {

                li {
                    a[
                        href = &ra_statistics(&data.lang.code),
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
                        href = &ra_server(&data.lang.code),
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

            /* Data
             * * * * */
            p[class = "menu-label"] {
                @t.data
            }
            ul[class = "menu-list"] {
                li {
                    a[
                        href = &ra_users(&data.lang.code),
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
                        href = &ra_languages(&data.lang.code),
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
                        href = &ra_posts(&data.lang.code),
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
                                    href = &ra_posts_w_f(
                                        &data.lang.code,
                                        "drafts",
                                    ),
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
                                    href = &ra_posts_w_f(
                                        &data.lang.code,
                                        "published",
                                    ),
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
                                    href = &ra_posts_w_f(
                                        &data.lang.code,
                                        "untranslated",
                                    ),
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
                                    href = &ra_posts_w_f(
                                        &data.lang.code,
                                        "deleted",
                                    ),
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
                li {
                    a[
                        href = &ra_pages(&data.lang.code),
                        class = if current_page == &"pages" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[class = "eos-icons"] { "description" }
                        " "
                        @t.pages
                    }
                }
                li {
                    a[
                        href = &ra_files(&data.lang.code),
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
                        href = &ra_favicon(&data.lang.code),
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
                        href = &ra_theme(&data.lang.code),
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
                        href = &ra_menus(&data.lang.code),
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
                        href = &ra_widgets(&data.lang.code),
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
                        href = &ra_website(&data.lang.code),
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
                        href = &ra_tukosmo(&data.lang.code),
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

