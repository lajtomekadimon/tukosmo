use markup;

use crate::i18n::t::t;


markup::define! {
    AdminNavbar<'a>(
        lang_code: &'a str,
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
                                "Lajto (test@test.com)"
                            }

                            div[class = "navbar-dropdown is-right"] {
                                a[class = "navbar-item"] {
                                    {&t("Account", lang_code)}
                                }
                                a[class = "navbar-item"] {
                                    {&t("Sessions", lang_code)}
                                }
                                a[class = "navbar-item"] {
                                    {&t("Settings", lang_code)}
                                }

                                hr[class = "navbar-divider"];

                                a[
                                    href = "/logout?lang={lang}".replace("{lang}", &lang_code),
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

