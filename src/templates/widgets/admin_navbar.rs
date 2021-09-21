use markup;

use crate::i18n::t::t;
use crate::database::types::DataDB;


markup::define! {
    AdminNavbar<'a>(
        data: &'a DataDB,
    ) {
        nav[class = "navbar is-white"] {
            div[class = "container"] {
                div[class = "navbar-brand"] {
                    a[
                        class = "navbar-item brand-text tap-logo",
                        href = "/{lang}/admin/"
                            .replace("{lang}", &data.lang.code),
                        title = &t("Tukosmo Admin Panel", &data.lang.code),
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
                        ] {
                            {&t("Visit website", &data.lang.code)}
                        }
                        a[
                            class = "navbar-item",
                            href = "/",  // link to official Tukosmo's docs
                            target = "_blank",
                        ] {
                            {&t("Documentation", &data.lang.code)}
                        }
                        a[
                            class = "navbar-item",
                            href = "/",  // link to official Tukosmo's help
                            target = "_blank",
                        ] {
                            {&t("Help [noun]", &data.lang.code)}
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
                                    {&t("Account", &data.lang.code)}
                                }

                                a[
                                    class = "navbar-item",
                                    href = "/{lang}/admin/sessions"
                                        .replace("{lang}", &data.lang.code)
                                    ,
                                ] {
                                    {&t("Sessions", &data.lang.code)}
                                }

                                hr[class = "navbar-divider"];

                                a[
                                    href = "/{lang}/admin/logout"
                                        .replace("{lang}", &data.lang.code)
                                    ,
                                    class = "navbar-item",
                                ] {
                                    {&t("Logout [verb]", &data.lang.code)}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

