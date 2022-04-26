use actix_web::web::Form as ActixForm;
use markup;

use crate::handlers::{
    admin::{
        forgotten_password_get::{
            AgoForgottenPassword,
            ra_forgotten_password,
        },
        forgotten_password_post::FormData,
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
    ForgottenPassword<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a AgoForgottenPassword,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
        success: &'a bool,
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
                    success: success,
                },
                data: &q.data,
                codename: codename,
                t: t,
                routes: &q.routes,
                forgotten_password: &true,
            },
        }
    }

    Content<'a>(
        q: &'a AgoForgottenPassword,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
        success: &'a bool,
    ) {

        @if **success {
            p {
                {&t.reset_password_success_w_email
                    .replace(
                        "{email}",
                        if let Some(f) = form {
                            &f.email
                        } else { "" },
                    )}
            }
        } else {
            div[
                class = "notification is-info",
            ] {
                button[class = "delete"] {}
                @t.forgotten_password_message_info
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

        @if !(**success) {
            @Form {
                q: q,
                t: t,
                error: error,
                form: form,
            }
        }

    }

    Form<'a>(
        q: &'a AgoForgottenPassword,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        form[
            action = ra_forgotten_password(&q.data.lang.code),
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

            button[
                class = "button is-block is-link is-large is-fullwidth",
            ] {
                span {
                    @t.reset_password
                }
                span[class = "icon is-small"] {
                    @icons::VPNKey {}
                }
            }
        }
    }
}

