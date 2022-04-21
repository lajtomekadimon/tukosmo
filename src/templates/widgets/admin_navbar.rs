use markup;

use crate::files::static_files::{
    staticf_route,
    TUKOSMO_LOGO_TAP_40,
};
use crate::handlers::{
    admin::dashboard_get::ra_dashboard,
    admin::account_get::ra_account,
    admin::sessions_get::ra_sessions,
    admin::logout_get::ra_logout,
    website::home_get::rw_home,
};
use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::AdminDataDB;
use crate::templates::widgets::icons;


markup::define! {
    AdminNavbar<'a>(
        codename: &'a str,
        data: &'a AdminDataDB,
        t: &'a TranslateI18N,
    ) {
        nav[class = "navbar is-white"] {
            div[class = "navbar-brand"] {
                a[
                    class = "navbar-item brand-text tap-logo",
                    style = "background-image:url({url})".replace(
                        "{url}", &staticf_route(TUKOSMO_LOGO_TAP_40, codename),
                    ),
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
                        span[class = "mr-1"] {
                            @icons::HomeSIZE6 {}
                        }
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
                    @if data.languages.iter().len() > 1 {
                        a[
                            id = "button-select-language",
                            class = "navbar-item",
                            title = t.select_a_language,
                        ] {
                            span[class = "mr-1"] {
                                @icons::LanguageSIZE6 {}
                            }
                            {data.lang.code.to_uppercase()}
                        }
                    }

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

