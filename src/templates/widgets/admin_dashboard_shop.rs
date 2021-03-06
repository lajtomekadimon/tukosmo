use markup;

//use crate::handlers::admin::dashboard_get::AgoDashboard;
use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::widgets::icons;


markup::define! {
    AdminDashboardShop<'a>(
        //q: &'a AgoDashboard,
        t: &'a TranslateI18N,
    ) {

        section[class = "tile is-ancestor mt-3"] {
            div[class = "tile is-6 is-vertical is-parent"] {
                div[class = "card events-card"] {
                    header[class = "card-header"] {
                        p[class = "card-header-title"] {
                            "Last orders"
                        }
                        a[
                            href = "",
                            class = "card-header-icon",
                        ] {
                            span[class = "icon"] {
                                @icons::KeyboardArrowDown {}
                            }
                        }
                    }
                    div[class = "card-table"] {
                        div[class = "content"] {
                            table[
                                class = "table is-fullwidth is-striped",
                            ] {
                                tbody {
                                    @for _n in 1..9 {
                                        tr {
                                            td[width = "5%"] {
                                                @icons::Sell {}
                                            }
                                            td { "Lorum ipsum dolem aire" }
                                            td[class = "level-right"] {
                                                a[
                                                    class = "button is-small \
                                                             is-info",
                                                    href = "",
                                                ] { "Action" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    footer[class = "card-footer"] {
                        a[
                            href = "",
                            class = "card-footer-item",
                        ] {
                            @t.view_all
                        }
                    }
                }
            }

            div[class = "tile is-6 is-vertical is-parent"] {
                div[class = "card"] {
                    header[class = "card-header"] {
                        p[class = "card-header-title"] {
                            "Inventory search"
                        }
                        a[
                            href = "",
                            class = "card-header-icon",
                        ] {
                            span[class = "icon"] {
                                @icons::KeyboardArrowDown {}
                            }
                        }
                    }
                    div[class = "card-content"] {
                        div[class = "content"] {
                            div[class = "control has-icons-left has-icons-right"] {
                                input[
                                    class = "input is-large",
                                    type = "text",
                                    placeholder = "",
                                ];
                                span[class = "icon is-medium is-left"] {
                                    @icons::Search {}
                                }
                                span[class = "icon is-medium is-right"] {
                                    @icons::Check {}
                                }
                            }
                        }
                    }
                }
                div[class = "card"] {
                    header[class = "card-header"] {
                        p[class = "card-header-title"] {
                            "User search"
                        }
                        a[
                            href = "",
                            class = "card-header-icon",
                        ] {
                            span[class = "icon"] {
                                @icons::KeyboardArrowDown {}
                            }
                        }
                    }
                    div[class = "card-content"] {
                        div[class = "content"] {
                            div[class = "control has-icons-left has-icons-right"] {
                                input[
                                    class = "input is-large",
                                    type = "text",
                                    placeholder = "",
                                ];
                                span[class = "icon is-medium is-left"] {
                                    @icons::Search {}
                                }
                                span[class = "icon is-medium is-right"] {
                                    @icons::Check {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }

}

