use markup;

use crate::i18n::t::t;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::database::s_posts::s_posts;
use crate::database::data::DataDB;


markup::define! {
    Posts<'a>(
        title: &'a str,
        data: &'a DataDB,
    ) {
        @AdminLayout {
            title: title,
            lang: &data.lang,
            content: AdminPanel {
                content: Content {
                    data: data,
                },
                current_page: "posts",
                data: data,
            },
        }
    }

    Content<'a>(
        data: &'a DataDB,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t("Posts", &data.lang.code)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: "/admin/posts",
                        data: data,
                    }
                }

                a[
                    href = "/{lang}/admin/new_post"
                        .replace("{lang}", &data.lang.code),
                    class = "button is-link is-pulled-right has-text-weight-normal mr-4",
                ] {
                    {&t("New post", &data.lang.code)}
                }
            }

            table[
                class = "table is-bordered is-hoverable is-fullwidth",
            ] {
                thead {
                    tr {
                        th {
                            {&t("Title", &data.lang.code)}
                        }
                        th {
                            {&t("Status", &data.lang.code)}
                        }
                        th {
                            {&t("Published", &data.lang.code)}
                        }
                        th {
                            {&t("Author", &data.lang.code)}
                        }
                    }
                }
                tbody {
                    @for post in s_posts(data.lang.id) {
                        tr {
                            td {
                                a[
                                    href = "/{lang}/admin/edit_post?id={id}"
                                        .replace("{lang}", &data.lang.code)
                                        .replace(
                                            "{id}",
                                            &post.id.to_string()
                                        ),
                                ] {
                                    @post.title
                                }

                                @if !post.has_all_trans {
                                    " (!)"
                                }
                            }
                            td {
                                @if post.untranslated {
                                    {&t("Untranslated", &data.lang.code)}
                                } else if post.draft {
                                    {&t("Draft", &data.lang.code)}
                                } else {
                                    {&t("Published", &data.lang.code)}
                                }
                            }
                            td {
                                {t_date(&post.date, &data.lang.code)}

                                @if (post.author_name != post.translator_name) && !post.untranslated {
                                    " ("
                                    {t_date(&post.date_trans, &data.lang.code)}
                                    ")"
                                }
                            }
                            td {
                                a[
                                    href = "/{lang}/admin/edit_user?id={id}"
                                        .replace("{lang}", &data.lang.code)
                                        .replace(
                                            "{id}",
                                            &post.author.to_string()
                                        ),
                                ] {
                                    @post.author_name
                                }

                                @if (post.author_name != post.translator_name) && !post.untranslated {
                                    " ("
                                    {
                                        &t("translated by {name}", &data.lang.code)
                                            .replace("{name}", &post.translator_name)
                                    }
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

