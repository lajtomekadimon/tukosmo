use markup;

use crate::i18n::t::t;
use crate::database::data::DataDB;


markup::define! {
    AdminNavbar<'a>(
        lang_code: &'a str,
        data: &'a DataDB,
    ) {
        nav[class = "navbar is-white"] {
            div[class = "container"] {
                div[class = "navbar-brand"] {
                    a[
                        class = "navbar-item brand-text tap-logo",
                        href = "/{lang}/admin/".replace("{lang}", &lang_code),
                    ] {
                        //{&t("Tukosmo Admin Panel", lang_code)}
                    }
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
                            href = "/{lang}/".replace("{lang}", &lang_code),
                            target = "_blank",
                        ] {
                            {&t("Visit website", lang_code)}
                        }
                        a[
                            class = "navbar-item",
                            href = "/",  // link to official Tukosmo's docs
                            target = "_blank",
                        ] {
                            {&t("Documentation", lang_code)}
                        }
                        a[
                            class = "navbar-item",
                            href = "/",  // link to official Tukosmo's help
                            target = "_blank",
                        ] {
                            {&t("Help [noun]", lang_code)}
                        }
                    }

                    div[class = "navbar-end"] {
                        div[class = "navbar-item has-dropdown is-hoverable"] {
                            a[class = "navbar-link"] {
                                @data.user.name
                                " ("
                                @data.user.email
                                ")"
                            }

                            div[class = "navbar-dropdown is-right"] {
                                a[
                                    class = "navbar-item",
                                    href = "/{lang}/admin/account"
                                        .replace("{lang}", &lang_code)
                                    ,
                                ] {
                                    {&t("Account", lang_code)}
                                }

                                a[
                                    class = "navbar-item",
                                    href = "/{lang}/admin/sessions"
                                        .replace("{lang}", &lang_code)
                                    ,
                                ] {
                                    {&t("Sessions", lang_code)}
                                }

                                hr[class = "navbar-divider"];

                                a[
                                    href = "/{lang}/admin/logout"
                                        .replace("{lang}", &lang_code)
                                    ,
                                    class = "navbar-item",
                                ] {
                                    {&t("Logout [verb]", lang_code)}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

