use markup;

use crate::i18n::t::t;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::database::s_posts::s_posts;


markup::define! {
    Posts<'a>(
        title: &'a str,
        lang_code: &'a str,
    ) {
        @AdminLayout {
            title: title,
            lang_code: lang_code,
            content: AdminPanel {
                content: Content {
                    lang_code: lang_code,
                },
                current_page: "posts",
                lang_code: lang_code,
            },
        }
    }

    Content<'a>(
        lang_code: &'a str,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t("Posts", lang_code)}

                a[
                    href = "/{lang}/admin/new_post".replace("{lang}", lang_code),
                    class = "button is-link is-pulled-right has-text-weight-normal",
                ] {
                    {&t("New post", lang_code)}
                }
            }

            table[
                class = "table is-bordered is-hoverable is-fullwidth",
            ] {
                thead {
                    tr {
                        th {
                            {&t("Title", lang_code)}
                        }
                        th {
                            {&t("Status", lang_code)}
                        }
                        th {
                            {&t("Published", lang_code)}
                        }
                        th {
                            {&t("Author", lang_code)}
                        }
                    }
                }
                tbody {
                    @for post in s_posts(lang_code.to_string()) {
                        tr {
                            td {
                                a[
                                    href = "/{lang}/admin/edit_post?id={id}"
                                        .replace("{lang}", lang_code)
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
                                    {&t("Untranslated", lang_code)}
                                } else if post.draft {
                                    {&t("Draft", lang_code)}
                                } else {
                                    {&t("Published", lang_code)}
                                }
                            }
                            td {
                                {t_date(&post.date, lang_code)}

                                @if (post.original_author_name != post.author_name) && !post.untranslated {
                                    " ("
                                    {t_date(&post.date_trans, lang_code)}
                                    ")"
                                }
                            }
                            td {
                                a[
                                    href = "/{lang}/admin/edit_user?id={id}"
                                        .replace("{lang}", lang_code)
                                        .replace(
                                            "{id}",
                                            &post.original_author.to_string()
                                        ),
                                ] {
                                    @post.original_author_name
                                }

                                @if (post.original_author_name != post.author_name) && !post.untranslated {
                                    " (translated by "
                                    a[
                                        href = "/{lang}/admin/edit_user?id={id}"
                                            .replace("{lang}", lang_code)
                                            .replace(
                                                "{id}",
                                                &post.author.to_string()
                                            ),
                                    ] {
                                        @post.author_name
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

