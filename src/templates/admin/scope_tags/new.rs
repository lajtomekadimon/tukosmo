use actix_web::web::Form as ActixForm;
use markup;

use crate::handlers::admin::{
    scope_tags::new_get::{
        AgoTagsNew,
        ra_tags_new,
    },
    scope_tags::new_post::FormData,
    tags_get::ra_tags,
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
    New<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a AgoTagsNew,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
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
                    error: error,
                    form: form,
                },
                current_page: "new_tag",
                codename: codename,
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        q: &'a AgoTagsNew,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.new_tag
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
                action = ra_tags_new(&q.data.lang.code),
            ] {
                input[
                    type = "hidden",
                    name = "csrf_token",
                    value = &q.csrf_token,
                ];

                @for (i, lang) in q.data.languages.iter().enumerate() {
                    div[class = "field is-horizontal"] {
                        div[class = "field-label is-normal"] {
                            label[class = "label"] {
                                @lang.name
                            }

                            input[
                                type = "hidden",
                                name = "lang_id",
                                value = &lang.id.to_string(),
                            ];
                        }

                        div[class = "field-body"] {
                            div[class = "field"] {
                                p[
                                    class = "control is-expanded \
                                             has-icons-left"
                                ] {
                                    input[
                                        class = if let Some(e) = error {
                                            if e.code ==
                                                ec::SOME_WRONG_TAG_NAME
                                            {
                                                "input is-danger"
                                            } else {
                                                "input"
                                            }
                                        } else {
                                            "input"
                                        },
                                        type = "text",
                                        placeholder = &t.name,
                                        name = "tag_name",
                                        value = if let Some(f) = form {
                                            &f.tag_names[i]
                                        } else { "" },
                                    ];
                                    span[class = "icon is-small is-left"] {
                                        i[
                                            class = "eos-icons notranslate",
                                            translate = "no",
                                        ] { "label" }
                                    }
                                }
                            }

                            div[class = "field"] {
                                p[
                                    class = "control is-expanded \
                                             has-icons-left"
                                ] {
                                    input[
                                        class = if let Some(e) = error {
                                            if e.code ==
                                                ec::SOME_WRONG_TAG_PERMALINK
                                            {
                                                "input is-danger"
                                            } else {
                                                "input"
                                            }
                                        } else {
                                            "input"
                                        },
                                        type = "text",
                                        placeholder = &t.permalink,
                                        name = "tag_permalink",
                                        value = if let Some(f) = form {
                                            &f.tag_permalinks[i]
                                        } else { "" },
                                    ];
                                    span[class = "icon is-small is-left"] {
                                        i[
                                            class = "eos-icons notranslate",
                                            translate = "no",
                                        ] { "link" }
                                    }
                                }
                            }
                        }
                    }
                }

                div[class = "field is-grouped"] {
                    div[class = "control"] {
                        button[class = "button is-link"] {
                            @t.submit
                        }
                    }
                    div[class = "control"] {
                        a[
                            href = ra_tags(&q.data.lang.code),
                            class = "button is-link is-light",
                        ] {
                            @t.cancel
                        }
                    }
                }
            }
        }
    }
}

