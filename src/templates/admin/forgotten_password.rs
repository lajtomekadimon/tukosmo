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
    types::{AdminDataDB, UserDB},
    error_codes as ec,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_login::AdminLogin,
};


markup::define! {
    ForgottenPassword<'a>(
        title: &'a str,
        q: &'a AgoForgottenPassword,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
        success: &'a bool,
    ) {
        @AdminLayout {
            title: title,
            data: &AdminDataDB {
                userd: UserDB {
                    id: 0,
                    email: "".to_string(),
                    name: "".to_string(),
                    date: "".to_string(),
                },
                lang: q.data.lang.clone(),
                languages: q.data.languages.clone(),
                website_title: q.data.website_title.clone(),
                website_subtitle: q.data.website_subtitle.clone(),
                copyright_owner: q.data.copyright_owner.clone(),
            },
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
                t: t,
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
                @t.reset_password
                " "
                i[class = "eos-icons ml-2"] { "vpn_key" }
            }
        }
    }
}
