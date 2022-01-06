use actix_web::web::Form as ActixForm;
use markup;

use crate::handlers::admin::{
    scope_users::edit_get::{
        AgoUsersEdit,
        ra_users_edit_w_id,
    },
    scope_users::edit_post::FormData,
    scope_users::delete_get::ra_users_delete_w_id,
    users_get::ra_users,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_error::ErrorDB,
    get_name_from_names::get_name_from_names,
};
use crate::database::error_codes as ec;
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
    widgets::admin_lang_dropdown::AdminLangDropdown,
};


markup::define! {
    Edit<'a>(
        title: &'a str,
        q: &'a AgoUsersEdit,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {
                    q: q,
                    t: t,
                    error: error,
                    form: form,
                },
                current_page: "edit_user",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a AgoUsersEdit,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.edit_user_w_name
                    .replace("{name}", &q.user_data.name)

                @if q.data.languages.iter().len() > 1 {
                    div[class = "is-pulled-right"] {
                        @AdminLangDropdown {
                            routes: &q.routes,
                            data: &q.data,
                        }
                    }
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

            form[
                method = "post",
                action = ra_users_edit_w_id(
                    &q.data.lang.code,
                    &q.user_data.id,
                ),
            ] {
                input[
                    type = "hidden",
                    name = "csrf_token",
                    value = &q.csrf_token,
                ];

                input[
                    type = "hidden",
                    name = "id",
                    value = &q.user_data.id,
                ];

                div[class = "field"] {
                    label[class = "label"] {
                        @t.email
                    }
                    div[class = "control"] {
                        input[
                            class = if let Some(e) = error {
                                if (e.code == ec::WRONG_EMAIL)
                                    || (e.code == ec::EMAIL_ALREADY_EXISTS)
                                {
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
                            } else { &q.user_data.email },
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.name
                    }
                    div[class = "control"] {
                        input[
                            class = if let Some(e) = error {
                                if e.code == ec::WRONG_USER_NAME {
                                    "input is-danger"
                                } else {
                                    "input"
                                }
                            } else {
                                "input"
                            },
                            type = "text",
                            name = "name",
                            value = if let Some(f) = form {
                                &f.name
                            } else { &q.user_data.name },
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.names_for_each_language

                        " ("
                        @t.optional
                        ")"
                    }
                    p[class = "control"] {
                        @for (i, lang) in q.data.languages.iter().enumerate() {
                            div[class = "field has-addons is-marginless"] {
                                div[class = "control"] {
                                    span[class = "button is-static"] {
                                        @lang.name
                                    }
                                }
                                div[class = "control is-expanded"] {
                                    input[
                                        type = "hidden",
                                        name = "i18n_name_lang",
                                        value = &lang.id.to_string(),
                                    ];

                                    @if let Some(f) = form {
                                        input[
                                            class = if let Some(e) = error {
                                                if e.code ==
                                                ec::SOME_WRONG_I18N_USER_NAME
                                                {
                                                    "input is-danger"
                                                } else {
                                                    "input"
                                                }
                                            } else {
                                                "input"
                                            },
                                            type = "text",
                                            name = "i18n_name",
                                            value = &f.i18n_names[i],
                                        ];
                                    } else {
                                        input[
                                            class = if let Some(e) = error {
                                                if e.code ==
                                                ec::SOME_WRONG_I18N_USER_NAME
                                                {
                                                    "input is-danger"
                                                } else {
                                                    "input"
                                                }
                                            } else {
                                                "input"
                                            },
                                            type = "text",
                                            name = "i18n_name",
                                            value = &get_name_from_names(
                                                lang.id,
                                                &q.i18n_names,
                                            ),
                                        ];
                                    }

                                }
                            }
                        }
                    }
                }

                // TODO
                /*
                div[class = "field"] {
                    div[class = "control"] {
                        label[class = "checkbox"] {
                            input[
                                type = "checkbox",
                                name = "is_admin",
                                value = "yes",
                                checked = if let Some(f) = form {
                                    match &f.is_admin {
                                        Some(_) => true,
                                        None => false,
                                    }
                                } else { false },
                            ];
                            " "
                            @t.admin
                        }
                    }
                }
                */

                div[class = "field is-grouped"] {
                    div[class = "control"] {
                        button[class = "button is-link"] {
                            @t.submit
                        }
                    }
                    div[class = "control"] {
                        a[
                            href = ra_users(&q.data.lang.code),
                            class = "button is-link is-light",
                        ] {
                            @t.cancel
                        }
                    }

                    div[class = "control"] {
                        a[
                            href = ra_users_delete_w_id(
                                &q.data.lang.code,
                                &q.user_data.id,
                            ),
                            class = "button is-danger \
                                     has-text-weight-normal mr-4",
                        ] {
                            @t.delete
                        }
                    }
                }
            }
        }
    }
}

