use markup;
use chrono::{
    DateTime,
    offset::Utc,
};

use crate::handlers::admin::statistics_get::AgoStatistics;
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_date_chrono::t_date_chrono,
    t_duration::t_duration,
    t_bytesize::t_bytesize,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Statistics<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a AgoStatistics,
        t: &'a TranslateI18N,
        start_date: &'a DateTime<Utc>,
        secs_running: &'a i64,
        disk_used: &'a i64,
        disk_free: &'a i64,
        disk_total: &'a i64,
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
                    start_date: start_date,
                    secs_running: secs_running,
                    disk_used: disk_used,
                    disk_free: disk_free,
                    disk_total: disk_total,
                },
                current_page: "statistics",
                codename: codename,
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        q: &'a AgoStatistics,
        t: &'a TranslateI18N,
        start_date: &'a DateTime<Utc>,
        secs_running: &'a i64,
        disk_used: &'a i64,
        disk_free: &'a i64,
        disk_total: &'a i64,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.statistics
            }

            div[class = "tabs is-medium is-centered is-boxed"] {
                ul {
                    li[class = "is-active"] {
                        a {
                            span[class = "icon is-small"] {
                                i[
                                    class = "eos-icons notranslate",
                                    translate = "no",
                                ] { "open_in_browser" }
                            }
                            span {
                                @t.website
                            }
                        }
                    }
                }
            }

            div[id = "stats-website"] {
                div[class = "columns is-desktop"] {
                    div[class = "column is-4 content pr-6"] {
                        p {
                            {&t.tukosmo_has_been_running_for_w_duration
                                .replace(
                                    "{duration}",
                                    &t_duration(secs_running, t),
                                )}
                            br[];
                            {&t.last_start_w_date
                                .replace("{date}", &t_date_chrono(
                                    start_date,
                                    &q.data.lang.code,
                                ))}
                        }
                        p {
                            b {
                                @t.disk_usage
                                ":"
                            }
                            " "
                            {&t.disk_size_w_used_free_total_u1_u2_u3
                                .replace("{used}", &(f64::trunc(t_bytesize(disk_used).0 * 100.0) / 100.0).to_string())
                                .replace("{free}", &(f64::trunc(t_bytesize(disk_free).0 * 100.0) / 100.0).to_string())
                                .replace("{total}", &(f64::trunc(t_bytesize(disk_total).0 * 100.0) / 100.0).to_string())
                                .replace("{u1}", &t_bytesize(disk_used).1)
                                .replace("{u2}", &t_bytesize(disk_free).1)
                                .replace("{u3}", &t_bytesize(disk_total).1)}
                        }
                        table[class = "table"] {
                            thead {
                                tr {
                                    th {}
                                    th {
                                        @t.visitors
                                    }
                                    th {
                                        @t.visits
                                    }
                                }
                            }
                            tbody {
                                tr {
                                    th {
                                        @t.today
                                    }
                                    td {
                                        @q.visitors_today
                                    }
                                    td {
                                        @q.visits_today
                                    }
                                }
                                tr {
                                    th {
                                        @t.yesterday
                                    }
                                    td {
                                        @q.visitors_yesterday
                                    }
                                    td {
                                        @q.visits_yesterday
                                    }
                                }
                                tr {
                                    th {
                                        @t.last_7_days_week
                                    }
                                    td {
                                        @q.visitors_last_week
                                    }
                                    td {
                                        @q.visits_last_week
                                    }
                                }
                                tr {
                                    th {
                                        @t.last_30_days_month
                                    }
                                    td {
                                        @q.visitors_last_month
                                    }
                                    td {
                                        @q.visits_last_month
                                    }
                                }
                                tr {
                                    th {
                                        @t.last_365_days_year
                                    }
                                    td {
                                        @q.visitors_last_year
                                    }
                                    td {
                                        @q.visits_last_year
                                    }
                                }
                                tr {
                                    th {
                                        @t.total
                                    }
                                    td {
                                        @q.visitors_total
                                    }
                                    td {
                                        @q.visits_total
                                    }
                                }
                            }
                        }
                    }
                    div[class = "column is-8 chartcol p-6"] {
                        canvas[
                            id = "chart-visits",
                            class = "chartcol-wide",
                        ] {}
                        div[
                            id = "chart-visits-data",
                            "data-title" = &t.number_of_visits,
                            "data-labels" = &q.chart_visits_labels,
                            "data-lvisitors" = &t.visitors,
                            "data-visitors" = &q.chart_visits_visitors,
                            "data-lvisits" = &t.visits,
                            "data-visits" = &q.chart_visits_visits,
                        ] {}
                        /*
                        div[class = "has-text-centered mt-3 mb-5"] {
                            a[
                                class = "button is-small",
                                href = "",
                            ] {
                                @t.show_more
                            }
                        }
                        */
                    }
                }

                div[class = "columns is-desktop"] {
                    div[class = "column is-8 chartcol p-6"] {
                        canvas[
                            id = "chart-referrals",
                            class = "chartcol-wide",
                        ] {}
                        div[
                            id = "chart-referrals-data",
                            "data-title" = &t.external_referrals,
                            "data-labels" = &q.chart_referrals_labels,
                            "data-lvisitors" = &t.visitors,
                            "data-visitors" = &q.chart_referrals_visitors,
                            "data-lvisits" = &t.visits,
                            "data-visits" = &q.chart_referrals_visits,
                        ] {}
                    }
                    div[class = "column is-4 chartcol p-6"] {
                        canvas[
                            id = "chart-browsers",
                            class = "chartcol-centered",
                        ] {}
                        div[
                            id = "chart-browsers-data",
                            "data-title" = &t.browsers_k_web,
                            "data-labels" = &q.chart_browsers_labels,
                            "data-lvisitors" = &t.visitors,
                            "data-visitors" = &q.chart_browsers_visitors,
                        ] {}
                    }
                }

                div[class = "columns is-desktop"] {
                    div[class = "column is-4 chartcol p-6"] {
                        canvas[
                            id = "chart-platforms",
                            class = "chartcol-centered",
                        ] {}
                        div[
                            id = "chart-platforms-data",
                            "data-title" = &t.platforms_k_os,
                            "data-labels" = &q.chart_platforms_labels,
                            "data-lvisitors" = &t.visitors,
                            "data-visitors" = &q.chart_platforms_visitors,
                        ] {}
                    }
                    div[class = "column is-8 chartcol p-6"] {
                        // TODO: IP Geolocation in PostgreSQL
                        /*
                        canvas[
                            id = "chart-countries",
                            class = "chartcol-wide",
                        ] {}
                        div[
                            id = "chart-countries-data",
                            "data-title" = &t.countries,
                            "data-labels" = &q.chart_countries_labels,
                            "data-lvisitors" = &t.visitors,
                            "data-visitors" = &q.chart_countries_visitors,
                        ] {}
                        */
                    }
                }
            }

            /*---*/

            div[class = "tabs is-medium is-centered is-boxed mt-6"] {
                ul {
                    li[class = "is-active"] {
                        a {
                            span[class = "icon is-small"] {
                                i[
                                    class = "eos-icons notranslate",
                                    translate = "no",
                                ] { "memory" }
                            }
                            span {
                                @t.server
                            }
                        }
                    }
                }
            }

            div[
                id = "stats-server",
            ] {
                div[class = "columns is-desktop"] {
                    div[class = "column is-8 chartcol p-6"] {
                        canvas[
                            id = "chart-network",
                            class = "chartcol-wide",
                        ] {}
                        div[
                            id = "chart-network-data",
                            "data-title" = &t.network_w_unit
                                .replace("{unit}", "MiB"),
                            "data-labels" = &q.chart_network_labels,
                            "data-lupload" = &t.upload_k_noun,
                            "data-uploaded" = &q.chart_network_uploaded,
                            "data-ldownload" = &t.download_k_noun,
                            "data-downloaded" = &q.chart_network_downloaded,
                        ] {}
                    }
                    div[class = "column is-4 chartcol p-6"] {
                        canvas[
                            id = "chart-disk",
                            class = "chartcol-centered",
                        ] {}
                        div[
                            id = "chart-disk-data",
                            "data-title" = &t.disk_usage_w_unit
                                .replace("{unit}", "GiB"),
                            "data-lused" = &t.used_k_disk,
                            "data-used" = (**disk_used as f64) / 1073741824.0,
                            "data-lfree" = &t.free_k_disk,
                            "data-free" = (**disk_free as f64) / 1073741824.0,
                        ] {}
                    }
                }

                div[class = "columns is-desktop"] {
                    div[class = "column chartcol p-6"] {
                        canvas[
                            id = "chart-cpu",
                            class = "chartcol-wide",
                        ] {}
                        div[
                            id = "chart-cpu-data",
                            "data-title" = &t.cpu_usage_percent,
                            "data-labels" = &q.chart_cpu_labels,
                            "data-lcore" = &t.core_k_cpu,
                            "data-cores" = &q.chart_cpu_cores,
                        ] {}
                    }
                    div[class = "column chartcol p-6"] {
                        canvas[
                            id = "chart-memory",
                            class = "chartcol-wide",
                        ] {}
                        div[
                            id = "chart-memory-data",
                            "data-title" = &t.memory_usage_percent,
                            "data-labels" = &q.chart_memory_labels,
                            "data-lmemory" = &t.memory_k_ram,
                            "data-memory" = &q.chart_memory_memory,
                        ] {}
                    }
                }
            }
        }
    }
}

