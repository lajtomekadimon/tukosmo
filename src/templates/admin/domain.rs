use actix_web::web::Form as ActixForm;
use markup;

use crate::handlers::admin::{
    domain_get::{ra_domain, AgoDomain},
    domain_post::FormData,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_error::ErrorDB,
};
use crate::database::error_codes as ec;
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Domain<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a AgoDomain,
        t: &'a TranslateI18N,
        nochange: &'a bool,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
        user_email: &'a str,
        is_development: &'a bool,
    ) {
        @AdminLayout {
            domain: domain,
            codename: codename,
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {
                    q: q,
                    t: t,
                    nochange: nochange,
                    error: error,
                    form: form,
                    domain: domain,
                    user_email: user_email,
                    is_development: is_development,
                },
                current_page: "domain",
                codename: codename,
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        q: &'a AgoDomain,
        t: &'a TranslateI18N,
        nochange: &'a bool,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
        domain: &'a str,
        user_email: &'a str,
        is_development: &'a bool,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.domain_k_web
            }

            p[class = "mt-3 mb-3 is-size-4"] {
                {t.your_current_domain_is_w_domain
                    .replace("{domain}", domain)}
            }

            hr[];

            @if **is_development {
                div[
                    class = "notification is-info",
                ] {
                    button[class = "delete"] {}
                    @t.domain_cant_be_changed_in_development_mode
                }
            }

            @if **nochange {
                div[
                    class = "notification is-info",
                ] {
                    button[class = "delete"] {}
                    @t.same_domain_recieved_nothing_changed
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
                class = "notification is-warning",
            ] {
                button[class = "delete"] {}
                @t.warning_domain_paragraph
            }

            form[
                id = "form-domain",
                method = "post",
                action = ra_domain(&q.data.lang.code),
            ] {
                input[
                    type = "hidden",
                    name = "csrf_token",
                    value = &q.csrf_token,
                ];

                div[class = "field"] {
                    label[class = "label"] {
                        @t.new_domain_k_web
                    }
                    div[class = "control"] {
                        input[
                            id = "form-domain-domain",
                            class = if let Some(e) = error {
                                if e.code == ec::WRONG_DOMAIN {
                                    "input is-danger"
                                } else {
                                    "input"
                                }
                            } else {
                                "input"
                            },
                            type = "text",
                            name = "domain",
                            value = if let Some(f) = form {
                                &f.domain
                            } else { *domain },
                            disabled = is_development,
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.email
                    }
                    div[class = "control"] {
                        input[
                            class = if let Some(e) = error {
                                if e.code == ec::WRONG_EMAIL {
                                    "input is-danger"
                                } else {
                                    "input"
                                }
                            } else {
                                "input"
                            },
                            type = "email",
                            name = "email",
                            value = if let Some(f) = form {
                                &f.email
                            } else { *user_email },
                            disabled = is_development,
                        ];
                    }
                }

                p[class = "mb-4"] {
                    @t.youll_receive_any_expiry_notices_of_tls_certificates
                }

            }

            div[class = "field is-grouped"] {
                div[class = "control"] {
                    button[
                        id = if **is_development {
                            "form-domain-button"
                        } else { "" },
                        class = "button is-link",
                        disabled = is_development,
                    ] {
                        @t.submit
                    }
                }
            }

            div[
                id = "form-domain-progress",
                class = "is-hidden",
            ] {
                progress[
                    class = "progress is-large is-link",
                    value = "80",
                    max = "100",
                ] {}
            }
        }
    }
}

