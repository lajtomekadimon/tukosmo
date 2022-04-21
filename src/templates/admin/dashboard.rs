use markup;

use crate::config::global::Config;
use crate::config::global::TUKOSMO_VERSION;
use crate::handlers::admin::{
    dashboard_get::AgoDashboard,
    tukosmo_get::ra_tukosmo,
    server_get::ra_server,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_bytesize::t_bytesize,
    t_os::t_os,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::{
        admin_panel::AdminPanel,
        admin_dashboard_shop::AdminDashboardShop,
    },
};


markup::define! {
    Dashboard<'a>(
        domain: &'a str,
        codename: &'a str,
        config: &'a Config,
        title: &'a str,
        t: &'a TranslateI18N,
        q: &'a AgoDashboard,
        server_os: &'a str,
        disk_used: &'a i64,
        disk_total: &'a i64,
        disk_used_percent: &'a str,
        blog_enabled: &'a bool,
        gallery_enabled: &'a bool,
        faq_enabled: &'a bool,
        payments_enabled: &'a bool,
        subscriptions_enabled: &'a bool,
        shop_enabled: &'a bool,
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
                    server_os: server_os,
                    disk_used: disk_used,
                    disk_total: disk_total,
                    disk_used_percent: disk_used_percent,
                    blog_enabled: blog_enabled,
                    gallery_enabled: gallery_enabled,
                    faq_enabled: faq_enabled,
                    payments_enabled: payments_enabled,
                    subscriptions_enabled: subscriptions_enabled,
                    shop_enabled: shop_enabled,
                },
                current_page: "dashboard",
                codename: codename,
                config: config,
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        q: &'a AgoDashboard,
        t: &'a TranslateI18N,
        server_os: &'a str,
        disk_used: &'a i64,
        disk_total: &'a i64,
        disk_used_percent: &'a str,
        blog_enabled: &'a bool,
        gallery_enabled: &'a bool,
        faq_enabled: &'a bool,
        payments_enabled: &'a bool,
        subscriptions_enabled: &'a bool,
        shop_enabled: &'a bool,
    ) {
        section[class = "hero is-info welcome is-small"] {
            div[class = "hero-body"] {
                div[class = "container"] {
                    h1[class = "title"] {
                        @t.hello_user
                            .replace("{name}", &q.data.userd.name)
                    }
                    h2[class = "subtitle"] {
                        @t.i_hope_you_are_having_a_great_day
                    }
                }
            }
        }

        section[class = "tile is-ancestor has-text-centered mt-3"] {
            div[class = "tile is-parent"] {
                article[class = "tile is-child box"] {
                    p[class = "title is-4"] {
                        {&t.n_visitors_w_n.replace(
                            "{n}",
                            &q.visitors_last_month.to_string(),
                        )}
                    }
                    p[class = "subtitle"] {
                        @t.in_the_last_month
                    }
                }
            }

            div[class = "tile is-parent"] {
                article[class = "tile is-child box"] {
                    p[class = "title is-4"] {
                        a[
                            class = "has-text-dark",
                            href = &ra_tukosmo(&q.data.lang.code),
                        ] {
                            {&t.tukosmo_w_version.replace(
                                "{version}",
                                TUKOSMO_VERSION,
                            )}
                        }
                    }
                    p[class = "subtitle"] {
                        @t.updated_k_tukosmo
                        // TODO: Show minor pending updates in orange
                        // TODO: Show critical pending updates in red
                    }
                }
            }

            div[class = "tile is-parent"] {
                article[class = "tile is-child box"] {
                    p[class = "title is-4"] {
                        a[
                            class = "has-text-dark",
                            href = &ra_server(&q.data.lang.code),
                        ] {
                            {&t.server_w_os.replace("{os}", &t_os(server_os))}
                        }
                    }
                    p[class = "subtitle"] {
                        @t.updated_k_server
                        // TODO: Show minor pending updates in orange
                        // TODO: Show critical pending updates in red
                        // TODO: Add custom OS option, so everything is
                        // manually done by server admin
                    }
                }
            }

            div[class = "tile is-parent"] {
                article[class = "tile is-child box"] {
                    p[class = "title is-4"] {
                        {&t.disk_at_w_used.replace(
                            "{used}",
                            disk_used_percent,
                        )}
                    }
                    p[class = "subtitle"] {
                        {&t.disk_size_w_used_total_u1_u2
                            .replace(
                                "{used}",
                                &(f64::trunc(
                                    t_bytesize(disk_used).0 * 10.0
                                ) / 10.0).to_string(),
                            )
                            .replace(
                                "{total}",
                                &(f64::trunc(
                                    t_bytesize(disk_total).0 * 10.0
                                ) / 10.0).to_string(),
                            )
                            .replace("{u1}", &t_bytesize(disk_used).1)
                            .replace("{u2}", &t_bytesize(disk_total).1)
                        }
                    }
                }
            }
        }

        @if **blog_enabled {
            ""
        }

        @if **gallery_enabled {
            ""
        }

        @if **faq_enabled {
            ""
        }

        @if **payments_enabled {
            ""
        }

        @if **subscriptions_enabled {
            ""
        }

        @if **shop_enabled {
            @AdminDashboardShop {
                //q: q,
                t: t,
            }
        }
    }
}

