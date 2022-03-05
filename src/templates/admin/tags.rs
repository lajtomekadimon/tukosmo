use markup;

use crate::handlers::{
    admin::{
        tags_get::AgoTags,
        scope_tags::new_get::ra_tags_new,
        scope_tags::edit_get::ra_tags_edit_w_id,
        scope_users::edit_get::ra_users_edit_w_id,
    },
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_date::t_date,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Tags<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a AgoTags,
        t: &'a TranslateI18N,
        success: &'a bool,
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
                    success: success,
                },
                current_page: "tags",
                codename: codename,
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        q: &'a AgoTags,
        t: &'a TranslateI18N,
        success: &'a bool,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.tags

                a[
                    href = ra_tags_new(&q.data.lang.code),
                    class = "button is-link is-pulled-right \
                             has-text-weight-normal mr-4",
                ] {
                    @t.new_tag
                }
            }

            @if **success {
                div[
                    class = "notification is-success",
                ] {
                    button[class = "delete"] {}
                    @t.your_website_tags_were_successfully_updated
                }
            }

            table[
                class = "table is-bordered is-hoverable is-fullwidth",
            ] {
                thead {
                    tr {
                        th {
                            @t.name
                        }
                        th {
                            @t.author
                        }
                        th {
                            @t.last_update
                        }
                    }
                }
                tbody {
                    @if q.tags.len() == 0 {
                        tr {
                            td { "-" }
                            td { "-" }
                            td { "-" }
                        }
                    } else {
                        @for tag in q.tags.iter() {
                            tr[
                                class = if tag.translator == 0 {
                                    "has-background-danger-light"
                                } else {
                                    ""
                                },
                            ] {
                                td {
                                    a[
                                        href = ra_tags_edit_w_id(
                                            &q.data.lang.code,
                                            &tag.id,
                                        ),
                                        class = if tag.translator == 0 {
                                            "tag is-danger is-normal"
                                        } else {
                                            "tag is-link is-normal"
                                        },
                                    ] {
                                        @tag.name
                                    }
                                }
                                td {
                                    a[
                                        href = ra_users_edit_w_id(
                                            &q.data.lang.code,
                                            &tag.author,
                                        ),
                                        class = if tag.translator == 0 {
                                            "has-text-danger"
                                        } else {
                                            ""
                                        },
                                    ] {
                                        @tag.author_name
                                    }

                                    @if (tag.author_name
                                         != tag.translator_name)
                                        && (tag.translator != 0)
                                    {
                                        " ("
                                        {t.translated_by_user
                                            .replace(
                                                "{name}",
                                                &tag.translator_name,
                                            )}
                                        ")"
                                    }
                                }
                                td {
                                    {t_date(&tag.date, &q.data.lang.code)}

                                    @if (tag.author_name
                                         != tag.translator_name)
                                        && (tag.translator != 0)
                                    {
                                        " ("
                                        {t_date(
                                            &tag.date_trans,
                                            &q.data.lang.code
                                        )}
                                        ")"
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

