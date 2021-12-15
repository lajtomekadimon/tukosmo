use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::AdminDataDB;
use crate::handlers::admin::dashboard::ra_dashboard;
use crate::handlers::website::home::rw_home;
use crate::handlers::admin::account::ra_account;
use crate::handlers::admin::sessions::ra_sessions;
use crate::handlers::admin::logout::ra_logout;


markup::define! {
    AdminNavbar<'a>(
        data: &'a AdminDataDB,
        t: &'a TranslateI18N,
    ) {
        nav[class = "navbar is-white"] {
            div[class = "navbar-brand"] {
                a[
                    class = "navbar-item brand-text tap-logo",
                    href = &ra_dashboard(&data.lang.code),
                    title = t.tukosmo_admin_panel,
                ] {}
                div[
                    id = "navbar-burger",
                    class = "navbar-burger burger",
                ] {
                    span {}
                    span {}
                    span {}
                }
            }

            div[
                id = "navMenu",
                class = "navbar-menu",
            ] {
                div[class = "navbar-start"] {
                    a[
                        class = "navbar-item",
                        href = &rw_home(&data.lang.code),
                        target = "_blank",
                        title = t.visit_website,
                    ] {
                        i[class = "eos-icons mr-1"] { "home" }
                        " "
                        @data.website_title
                    }
                    /*
                    a[
                        class = "navbar-item",
                        href = "/",  // link to official Tukosmo's help
                        target = "_blank",
                    ] {
                        @t.help
                    }
                    */
                }

                div[class = "navbar-end"] {
                    div[
                        id = "navbar-dropdown",
                        class = "navbar-item has-dropdown",
                    ] {
                        a[
                            id = "navbar-link",
                            class = "navbar-link",
                        ] {
                            @data.userd.name
                            " ("
                            @data.userd.email
                            ")"
                        }

                        div[class = "navbar-dropdown is-right"] {
                            a[
                                class = "navbar-item",
                                href = &ra_account(&data.lang.code),
                            ] {
                                @t.account
                            }

                            a[
                                class = "navbar-item",
                                href = &ra_sessions(&data.lang.code),
                            ] {
                                @t.sessions
                            }

                            hr[class = "navbar-divider"];

                            a[
                                href = &ra_logout(&data.lang.code),
                                class = "navbar-item",
                            ] {
                                @t.logout_k_verb
                            }
                        }
                    }
                }
            }
        }
    }
}

