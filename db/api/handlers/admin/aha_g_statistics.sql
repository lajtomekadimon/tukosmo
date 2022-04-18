
CREATE TYPE "AgiStatistics" AS (
    req "AdminRequest"
);

CREATE TYPE "AgoStatistics" AS (
    data "AdminDataDB",
    routes "RouteDB"[],

    /*---*/

    visitors_today BIGINT,
    visitors_yesterday BIGINT,
    visitors_last_week BIGINT,
    visitors_last_month BIGINT,
    visitors_last_year BIGINT,
    visitors_total BIGINT,

    visits_today BIGINT,
    visits_yesterday BIGINT,
    visits_last_week BIGINT,
    visits_last_month BIGINT,
    visits_last_year BIGINT,
    visits_total BIGINT,

    /*---*/

    chart_visits_labels TEXT,
    chart_visits_visitors TEXT,
    chart_visits_visits TEXT,

    chart_referrals_labels TEXT,
    chart_referrals_visitors TEXT,
    chart_referrals_visits TEXT,

    chart_browsers_labels TEXT,
    chart_browsers_visitors TEXT,

    chart_platforms_labels TEXT,
    chart_platforms_visitors TEXT,

    chart_countries_labels TEXT,
    chart_countries_visitors TEXT,

    chart_network_labels TEXT,
    chart_network_uploaded TEXT,
    chart_network_downloaded TEXT,

    chart_cpu_labels TEXT,
    chart_cpu_cores TEXT,

    chart_memory_labels TEXT,
    chart_memory_memory TEXT
);


CREATE OR REPLACE FUNCTION aha_g_statistics(
    r "AgiStatistics"
)

RETURNS "AgoStatistics"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    routes "RouteDB"[];

    language_of_user BIGINT;

    /*---*/

    visitors_today BIGINT;
    visitors_yesterday BIGINT;
    visitors_last_week BIGINT;
    visitors_last_month BIGINT;
    visitors_last_year BIGINT;
    visitors_total BIGINT;

    visits_today BIGINT;
    visits_yesterday BIGINT;
    visits_last_week BIGINT;
    visits_last_month BIGINT;
    visits_last_year BIGINT;
    visits_total BIGINT;

    /*---*/

    chart_visits_labels TEXT;
    chart_visits_visitors TEXT;
    chart_visits_visits TEXT;

    chart_referrals_labels TEXT;
    chart_referrals_visitors TEXT;
    chart_referrals_visits TEXT;

    chart_browsers_labels TEXT;
    chart_browsers_visitors TEXT;

    chart_platforms_labels TEXT;
    chart_platforms_visitors TEXT;

    chart_countries_labels TEXT;
    chart_countries_visitors TEXT;

    chart_network_labels TEXT;
    chart_network_uploaded TEXT;
    chart_network_downloaded TEXT;

    chart_cpu_labels TEXT;
    chart_cpu_cores TEXT;

    chart_memory_labels TEXT;
    chart_memory_memory TEXT;

    stats_server_data "StatsServerDB";

    stats_visits_data "StatsVisitsDB";
    stats_visits_referrals_data "StatsVisitsReferralsDB";
    stats_visits_browsers_data "StatsVisitsBrowsersDB";
    stats_visits_platforms_data "StatsVisitsPlatformsDB";

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/statistics',
        language_of_user
    );

    stats_server_data := s_stats_server_in_hours(
        DATE_TRUNC('hour', NOW() - INTERVAL '1 day'),
        DATE_TRUNC('hour', NOW())
    );

    /*---------------------------------------------------*/

    /* TODO: Today and yesterday are, in fact, last 24 and 48 hours.
       I should choose between:
        - Change today to "last 24 hours" and remove yesterday.
        - Change functions and use user's tz to avoid tz's conflicts.
    */

    visitors_today := sc_stats_visitors_today();
    visitors_yesterday := sc_stats_visitors_yesterday();
    visitors_last_week := sc_stats_visitors_last_week();
    visitors_last_month := sc_stats_visitors_last_month();
    visitors_last_year := sc_stats_visitors_last_year();
    visitors_total := sc_stats_visitors_total();

    visits_today := ss_stats_visits_today();
    visits_yesterday := ss_stats_visits_yesterday();
    visits_last_week := ss_stats_visits_last_week();
    visits_last_month := ss_stats_visits_last_month();
    visits_last_year := ss_stats_visits_last_year();
    visits_total := ss_stats_visits_total();

    /*---------------------------------------------------*/

    stats_visits_data := s_stats_visits_in_hours(
        DATE_TRUNC('hour', NOW() - INTERVAL '1 day'),
        DATE_TRUNC('hour', NOW())
    );

    chart_visits_labels := stats_visits_data.date;
    chart_visits_visitors := stats_visits_data.visitors;
    chart_visits_visits := stats_visits_data.visits;

    stats_visits_referrals_data := s_stats_visits_referrals_total();

    IF stats_visits_referrals_data IS NOT NULL THEN

        chart_referrals_labels := stats_visits_referrals_data.referrers;
        chart_referrals_visitors := stats_visits_referrals_data.visitors;
        chart_referrals_visits := stats_visits_referrals_data.visits;

    ELSE

        chart_referrals_labels := 'unknown';
        chart_referrals_visitors := '0';
        chart_referrals_visits := '0';

    END IF;

    stats_visits_browsers_data := s_stats_visits_browsers_total();

    IF stats_visits_browsers_data IS NOT NULL THEN

        chart_browsers_labels := stats_visits_browsers_data.browsers;
        chart_browsers_visitors := stats_visits_browsers_data.visitors;

    ELSE

        chart_browsers_labels := 'unknown';
        chart_browsers_visitors := '0';

    END IF;

    stats_visits_platforms_data := s_stats_visits_platforms_total();

    IF stats_visits_platforms_data IS NOT NULL THEN

        chart_platforms_labels := stats_visits_platforms_data.platforms;
        chart_platforms_visitors := stats_visits_platforms_data.visitors;

    ELSE

        chart_platforms_labels := 'unknown';
        chart_platforms_visitors := '0';

    END IF;

    -- TODO: IP Geolocation
    --       (many comercial tools/databases; need a free and open source one)
    --chart_countries_labels := 'China,India,United States,Spain,France,Japan,South Korea,Mexico';
    --chart_countries_visitors := '81,59,40,36,31,28,19,8';
    chart_countries_labels := 'unknown';
    chart_countries_visitors := '0';

    IF stats_server_data IS NOT NULL THEN

        chart_network_labels := stats_server_data.date;
        chart_network_uploaded := stats_server_data.uploaded;
        chart_network_downloaded := stats_server_data.downloaded;

        chart_cpu_labels := stats_server_data.date;
        chart_cpu_cores := stats_server_data.cpu;

        chart_memory_labels := stats_server_data.date;
        chart_memory_memory := stats_server_data.memory;

    ELSE

        chart_network_labels := NOW()::TEXT;
        chart_network_uploaded := '0';
        chart_network_downloaded := '0';

        chart_cpu_labels := NOW()::TEXT;
        chart_cpu_cores := '0';

        chart_memory_labels := NOW()::TEXT;
        chart_memory_memory := '0';

    END IF;

    /*---------------------------------------------------*/

    RETURN ROW(
        -- data
        d,

        -- routes
        routes,

        /*---*/

        visitors_today,
        visitors_yesterday,
        visitors_last_week,
        visitors_last_month,
        visitors_last_year,
        visitors_total,

        visits_today,
        visits_yesterday,
        visits_last_week,
        visits_last_month,
        visits_last_year,
        visits_total,

        /*---*/

        chart_visits_labels,
        chart_visits_visitors,
        chart_visits_visits,

        chart_referrals_labels,
        chart_referrals_visitors,
        chart_referrals_visits,

        chart_browsers_labels,
        chart_browsers_visitors,

        chart_platforms_labels,
        chart_platforms_visitors,

        chart_countries_labels,
        chart_countries_visitors,

        chart_network_labels,
        chart_network_uploaded,
        chart_network_downloaded,

        chart_cpu_labels,
        chart_cpu_cores,

        chart_memory_labels,
        chart_memory_memory
    );

END;

$$;
