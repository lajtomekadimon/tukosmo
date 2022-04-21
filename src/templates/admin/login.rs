use actix_web::web::Form as ActixForm;
use markup;

use crate::handlers::{
    admin::{
        login_get::{
            AgoLogin,
            ra_login,
        },
        login_post::FormData,
    },
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_error::ErrorDB,
};
use crate::database::{
    types::websitedata_to_admindata,
    error_codes as ec,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::{
        admin_login::AdminLogin,
        icons,
    },
};


markup::define! {
    Login<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a AgoLogin,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        @AdminLayout {
            domain: domain,
            codename: codename,
            title: title,
            data: &websitedata_to_admindata(&q.data),
            routes: &q.routes,
            content: AdminLogin {
                content: Content {
                    q: q,
                    t: t,
                    error: error,
                    form: form,
                },
                data: &q.data,
                codename: codename,
                t: t,
                routes: &q.routes,
                forgotten_password: &false,
            },
        }
    }

    Content<'a>(
        q: &'a AgoLogin,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {

        @if let Some(e) = error {
            div[
                class = "notification is-danger",
            ] {
                button[class = "delete"] {}
                @e.message
            }
        }

        @Form {
            q: q,
            t: t,
            error: error,
            form: form,
        }

    }

    Form<'a>(
        q: &'a AgoLogin,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        form[
            action = ra_login(&q.data.lang.code),
            method = "post",
        ] {
            div[class = "field"] {
                div[class = "control"] {
                    input[
                        class = if let Some(e) = error {
                            if e.code == ec::WRONG_USER_EMAIL {
                                "input is-large is-danger"
                            } else {
                                "input is-large"
                            }
                        } else {
                            "input is-large"
                        },
                        name = "email",
                        type = "email",
                        placeholder = t.your_email,
                        value = if let Some(f) = form {
                            &f.email
                        } else { "" },
                        autofocus = if let Some(e) = error {
                            e.code == ec::WRONG_USER_EMAIL
                        } else {
                            true
                        },
                    ];
                }
            }

            div[class = "field"] {
                div[class = "control"] {
                    input[
                        class = if let Some(e) = error {
                            if e.code == ec::WRONG_USER_PASSWORD {
                                "input is-large is-danger"
                            } else {
                                "input is-large"
                            }
                        } else {
                            "input is-large"
                        },
                        name = "password",
                        type = "password",
                        placeholder = t.your_password,
                        autofocus = if let Some(e) = error {
                            e.code == ec::WRONG_USER_PASSWORD
                        } else {
                            false
                        },
                    ];
                }
            }

            button[
                class = "button is-block is-link is-large is-fullwidth",
            ] {
                @t.login_k_verb
                " "
                span[class = "ml-2"] {
                    @icons::Login {}
                }
            }
        }
    }
}

