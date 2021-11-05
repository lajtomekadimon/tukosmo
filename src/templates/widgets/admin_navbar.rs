use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::AdminDataDB;


markup::define! {
    AdminNavbar<'a>(
        data: &'a AdminDataDB,
        t: &'a TranslateI18N,
    ) {
        nav[class = "navbar is-white"] {
            div[class = "container"] {
                div[class = "navbar-brand"] {
                    a[
                        class = "navbar-item brand-text tap-logo",
                        href = "/{lang}/admin/"
                            .replace("{lang}", &data.lang.code),
                        title = t.tukosmo_admin_panel,
                    ] {}
                    div[class = "navbar-burger burger"] {
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
                            href = "/{lang}/"
                                .replace("{lang}", &data.lang.code),
                            target = "_blank",
                            title = t.visit_website,
                        ] {
                            i[class = "eos-icons mr-1"] { "home" }
                            " "
                            @data.website_title
                        }
                        a[
                            class = "navbar-item",
                            href = "/",  // link to official Tukosmo's help
                            target = "_blank",
                        ] {
                            @t.help
                        }
                    }

                    div[class = "navbar-end"] {
                        div[class = "navbar-item has-dropdown is-hoverable"] {
                            a[class = "navbar-link"] {
                                @data.userd.name
                                " ("
                                @data.userd.email
                                ")"
                            }

                            div[class = "navbar-dropdown is-right"] {
                                a[
                                    class = "navbar-item",
                                    href = "/{lang}/admin/account"
                                        .replace("{lang}", &data.lang.code)
                                    ,
                                ] {
                                    @t.account
                                }

                                a[
                                    class = "navbar-item",
                                    href = "/{lang}/admin/sessions"
                                        .replace("{lang}", &data.lang.code)
                                    ,
                                ] {
                                    @t.sessions
                                }

                                hr[class = "navbar-divider"];

                                a[
                                    href = "/{lang}/admin/logout"
                                        .replace("{lang}", &data.lang.code)
                                    ,
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
}

