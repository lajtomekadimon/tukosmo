use actix_web::web::Form as ActixForm;
use markup;

use crate::config::global::{
    Config,
    TUKOSMO_VERSION,
};
use crate::files::static_files::{
    staticf_route,
    FAVICONADMIN_96X96,
};
use crate::handlers::admin::{
    tukosmo_get::{AgoTukosmo, ra_tukosmo, ra_tukosmo_success},
    tukosmo_post::FormData,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_error::ErrorDB,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::{admin_panel::AdminPanel, icons},
};


markup::define! {
    Tukosmo<'a>(
        domain: &'a str,
        codename: &'a str,
        config: &'a Config,
        title: &'a str,
        q: &'a AgoTukosmo,
        t: &'a TranslateI18N,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
        new_version: &'a Option<String>,
    ) {
        @AdminLayout {
            domain: domain,
            codename: codename,
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {
                    codename: codename,
                    config: config,
                    q: q,
                    t: t,
                    success: success,
                    error: error,
                    form: form,
                    new_version: new_version,
                },
                current_page: "tukosmo",
                codename: codename,
                config: config,
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        codename: &'a str,
        config: &'a Config,
        q: &'a AgoTukosmo,
        t: &'a TranslateI18N,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
        new_version: &'a Option<String>,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.tukosmo
            }

            @if **success {
                div[
                    class = "notification is-success",
                ] {
                    button[class = "delete"] {}
                    @t.your_tukosmo_config_has_been_successfully_updated
                }
            }

            @if let Some(e) = error {
                div[
                    class = "notification is-danger",
                ] {
                    button[class = "delete"] {}
                    @e.message
                }
            }

            div[
                class = "notification is-info",
            ] {
                button[class = "delete"] {}
                @t.tukosmo_will_automatically_restart_itself_apply_changes
            }

            div[class = "columns"] {

                div[class = "column"] {

                    h2[class = "title is-4"] {
                        @t.activated_modules
                    }

                    form[
                        id = "form-tukosmo",
                        method = "post",
                        action = ra_tukosmo(&q.data.lang.code),
                    ] {
                        input[
                            type = "hidden",
                            name = "csrf_token",
                            value = &q.csrf_token,
                        ];

                        div[class = "field"] {
                            div[class = "control"] {
                                label[class = "checkbox"] {
                                    input[
                                        type = "checkbox",
                                        name = "module_blog",
                                        value = "yes",
                                        checked = if let Some(f) = form {
                                            f.module_blog.is_some()
                                        } else {
                                            &config.modules.blog.enabled == "yes"
                                        },
                                    ];
                                    " "
                                    @t.blog
                                }
                            }
                        }

                        div[class = "field"] {
                            div[class = "control"] {
                                label[class = "checkbox"] {
                                    input[
                                        type = "checkbox",
                                        name = "module_gallery",
                                        value = "yes",
                                        checked = if let Some(f) = form {
                                            f.module_gallery.is_some()
                                        } else {
                                            &config.modules.gallery.enabled == "yes"
                                        },
                                    ];
                                    " "
                                    @t.gallery
                                }
                            }
                        }

                        div[class = "field"] {
                            div[class = "control"] {
                                label[class = "checkbox"] {
                                    input[
                                        type = "checkbox",
                                        name = "module_faq",
                                        value = "yes",
                                        checked = if let Some(f) = form {
                                            f.module_faq.is_some()
                                        } else {
                                            &config.modules.faq.enabled == "yes"
                                        },
                                    ];
                                    " "
                                    @t.faq
                                }
                            }
                        }

                        div[class = "field"] {
                            div[class = "control"] {
                                label[class = "checkbox"] {
                                    input[
                                        type = "checkbox",
                                        name = "module_payments",
                                        value = "yes",
                                        checked = if let Some(f) = form {
                                            f.module_payments.is_some()
                                        } else {
                                            &config.modules.payments.enabled == "yes"
                                        },
                                    ];
                                    " "
                                    @t.payments
                                }
                            }
                        }

                        div[class = "field"] {
                            div[class = "control"] {
                                label[class = "checkbox"] {
                                    input[
                                        type = "checkbox",
                                        name = "module_subscriptions",
                                        value = "yes",
                                        checked = if let Some(f) = form {
                                            f.module_subscriptions.is_some()
                                        } else {
                                            &config.modules.subscriptions.enabled
                                                == "yes"
                                        },
                                    ];
                                    " "
                                    @t.subscriptions
                                }
                            }
                        }

                        div[class = "field"] {
                            div[class = "control"] {
                                label[class = "checkbox"] {
                                    input[
                                        type = "checkbox",
                                        name = "module_shop",
                                        value = "yes",
                                        checked = if let Some(f) = form {
                                            f.module_shop.is_some()
                                        } else {
                                            &config.modules.shop.enabled == "yes"
                                        },
                                    ];
                                    " "
                                    @t.shop
                                }
                            }
                        }
                    }  // end form

                    div[class = "field is-grouped"] {
                        div[class = "control"] {
                            button[
                                id = "form-tukosmo-button",
                                type = "button",
                                class = "button is-link mt-3",
                                "data-nexturl" = ra_tukosmo_success(
                                    &q.data.lang.code
                                ),
                            ] {
                                @t.submit
                            }
                        }
                    }

                    div[
                        id = "form-tukosmo-progress",
                        class = "is-hidden",
                    ] {
                        progress[
                            class = "progress is-large is-link",
                            value = "80",
                            max = "100",
                        ] {}
                    }

                }

                div[class = "column"] {
                    div[class = "card"] {
                        div[class = "card-content has-text-centered"] {
                            img[
                                src = staticf_route(
                                    FAVICONADMIN_96X96,
                                    codename,
                                ),
                                alt = "tukosmo",
                            ];

                            p[class = "is-size-3"] {
                                {&t.tukosmo_w_version.replace(
                                    "{version}",
                                    TUKOSMO_VERSION,
                                )}
                            }
                        }
                        div[class = "card-footer"] {
                            div[class = "card-footer-item"] {
                                span {
                                    @if let Some(new_version_value) = new_version {
                                        // TODO: "New version available!"
                                        button[
                                            id = "",  // TODO: Update Tukosmo
                                            class = "button is-medium is-link",
                                        ] {
                                            span {
                                                {&t.update_tukosmo_to_w_version.replace(
                                                    "{version}",
                                                    new_version_value,
                                                )}
                                            }
                                            span[class = "icon is-small"] {
                                                @icons::SyncI {}
                                            }
                                        }
                                        // TODO: Show message below
                                        // "A new version will be downloaded and
                                        // compiled; after that, the server will
                                        // restart itself
                                    } else {
                                        button[
                                            class = "button is-medium is-link",
                                            disabled = true,
                                        ] {
                                            span {
                                                @t.updated_k_tukosmo
                                            }
                                            span[class = "icon is-small"] {
                                                @icons::SyncDisabled {}
                                            }
                                        }

                                        p[class = "mt-2"] {
                                            @t.update_is_not_necessary
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

