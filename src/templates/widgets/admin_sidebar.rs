use markup;

use crate::handlers::admin::{
    dashboard_get::ra_dashboard,
    statistics_get::ra_statistics,
    users_get::ra_users,
    languages_get::ra_languages,
    tags_get::ra_tags,
    posts_get::{ra_posts, ra_posts_w_f},
    pages_get::ra_pages,
    files_get::ra_files,
    favicon_get::ra_favicon,
    website_get::ra_website,
    domain_get::ra_domain,
    tukosmo_get::ra_tukosmo,
    server_get::ra_server,
};
use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::AdminDataDB;


markup::define! {
    AdminSidebar<'a>(
        current_page: &'a str,
        data: &'a AdminDataDB,
        t: &'a TranslateI18N,
        blog_enabled: &'a bool,
        gallery_enabled: &'a bool,
        faq_enabled: &'a bool,
        payments_enabled: &'a bool,
        subscriptions_enabled: &'a bool,
        shop_enabled: &'a bool,
    ) {
        aside[
            id = "smenu",
            class = "menu is-hidden-mobile",
        ] {
            /* General
             * * * * * * */
            ul[class = "menu-list"] {
                li {
                    a[
                        href = &ra_dashboard(&data.lang.code),
                        class = if current_page == &"dashboard" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "dashboard" }
                        " "
                        @t.dashboard
                    }
                }
                li {
                    a[
                        href = &ra_statistics(&data.lang.code),
                        class = if current_page == &"statistics" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "bar_chart" }
                        " "
                        @t.statistics
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
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "supervisor_account" }
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
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "translate" }
                        " "
                        @t.languages
                    }
                }
                li {
                    a[
                        href = &ra_pages(&data.lang.code),
                        class = if current_page == &"pages" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "description" }
                        " "
                        @t.pages
                    }
                }
                li {
                    a[
                        href = &ra_tags(&data.lang.code),
                        class = if current_page == &"tags" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "label" }
                        " "
                        @t.tags
                    }
                }
                li {
                    a[
                        href = &ra_files(&data.lang.code),
                        class = if current_page == &"files" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "file_present" }
                        " "
                        @t.files
                    }
                }
            }

            /* Modules
             * * * * * */
            @if **blog_enabled || **gallery_enabled
                || **faq_enabled || **payments_enabled
                || **subscriptions_enabled || **shop_enabled
            {
                p[class = "menu-label"] {
                    @t.modules
                }
                ul[class = "menu-list"] {
                    @if **blog_enabled {
                        li {
                            a[
                                href = &ra_posts(&data.lang.code),
                                class = if current_page == &"posts" {
                                    "is-active"
                                } else { "" },
                            ] {
                                i[
                                    class = "eos-icons notranslate",
                                    translate = "no",
                                ] { "article" }
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
                                            class = if current_page ==
                                                &"posts-drafts"
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
                                            class = if current_page ==
                                                &"posts-published"
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
                                            class = if current_page ==
                                                &"posts-untranslated"
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
                                            class = if current_page ==
                                                &"posts-deleted"
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
                    }

                    @if **gallery_enabled {
                        li {
                            a[
                                //href = &ra_gallery(&data.lang.code),
                                class = if current_page == &"gallery" {
                                    "is-active"
                                } else { "" },
                            ] {
                                i[
                                    class = "eos-icons notranslate",
                                    translate = "no",
                                ] { "photo_library" }
                                " "
                                @t.gallery
                            }
                        }
                    }

                    @if **faq_enabled {
                        li {
                            a[
                                //href = &ra_faq(&data.lang.code),
                                class = if current_page == &"faq" {
                                    "is-active"
                                } else { "" },
                            ] {
                                i[
                                    class = "eos-icons notranslate",
                                    translate = "no",
                                ] { "quiz" }
                                " "
                                @t.faq
                            }
                        }
                    }

                    @if **payments_enabled {
                        li {
                            a[
                                //href = &ra_payments(&data.lang.code),
                                class = if current_page == &"payments" {
                                    "is-active"
                                } else { "" },
                            ] {
                                i[
                                    class = "eos-icons notranslate",
                                    translate = "no",
                                ] { "payments" }
                                " "
                                @t.payments
                            }
                        }
                    }

                    @if **subscriptions_enabled {
                        li {
                            a[
                                //href = &ra_subscriptions(&data.lang.code),
                                class = if current_page == &"subscriptions" {
                                    "is-active"
                                } else { "" },
                            ] {
                                i[
                                    class = "eos-icons notranslate",
                                    translate = "no",
                                ] { "product_subscriptions" }
                                " "
                                @t.subscriptions
                            }
                        }
                    }

                    @if **shop_enabled {
                        li {
                            a[
                                //href = &ra_shop(&data.lang.code),
                                class = if current_page == &"shop" {
                                    "is-active"
                                } else { "" },
                            ] {
                                i[
                                    class = "eos-icons notranslate",
                                    translate = "no",
                                ] { "store" }
                                " "
                                @t.shop
                            }
                        }
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
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "photo" }
                        " "
                        @t.favicon
                    }
                }

                li {
                    a[
                        //href = &ra_theme(&data.lang.code),
                        class = if current_page == &"theme" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "devices" }
                        " "
                        @t.theme
                    }
                }

                li {
                    a[
                        //href = &ra_menus(&data.lang.code),
                        class = if current_page == &"menus" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "list" }
                        " "
                        @t.menus
                    }
                }

                li {
                    a[
                        //href = &ra_widgets(&data.lang.code),
                        class = if current_page == &"widgets" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "view_sidebar" }
                        " "
                        @t.widgets
                    }
                }
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
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "web" }
                        " "
                        @t.website
                    }
                }
                li {
                    a[
                        href = &ra_domain(&data.lang.code),
                        class = if current_page == &"domain" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "dns" }
                        " "
                        @t.domain_k_web
                    }
                }
                li {
                    a[
                        href = &ra_tukosmo(&data.lang.code),
                        class = if current_page == &"tukosmo" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "settings_suggest" }
                        " "
                        @t.tukosmo
                    }
                }
                li {
                    a[
                        href = &ra_server(&data.lang.code),
                        class = if current_page == &"server" {
                            "is-active"
                        } else { "" },
                    ] {
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "database" }
                        " "
                        @t.server
                    }
                }
            }

        }
    }
}

