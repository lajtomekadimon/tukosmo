
/* TODO:
 * Split into many functions.
 */

CREATE OR REPLACE FUNCTION i_stats_visit(
    lang_value TEXT,
    route_value TEXT,
    ip_value TEXT,
    referrer_value TEXT,
    browser_value TEXT,
    platform_value TEXT
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    date_value TIMESTAMPTZ;

    visit_id BIGINT;
    visit_browser_id BIGINT;
    visit_platform_id BIGINT;
    referral_id BIGINT;

BEGIN

    date_value := DATE_TRUNC('hour', NOW());

    visit_id := s_tsh_visit_id_by_date_lang_route(
        date_value,
        lang_value,
        route_value
    );

    IF visit_id IS NULL THEN

        INSERT INTO tsh_visits (
            --tshv_id,
            tshv_date,
            tshv_lang,
            tshv_route,
            tshv_visitors,
            tshv_visits
        )
        VALUES (
            --BIGSERIAL,
            date_value,
            lang_value,
            route_value,
            1,
            1
        )
        RETURNING tshv_id INTO visit_id;

        INSERT INTO tsh_visits_visitors (
            tshvv_visit,
            tshvv_ip,
            tshvv_browser,
            tshvv_platform
        )
        VALUES (
            visit_id,
            ip_value,
            browser_value,
            platform_value
        );

        INSERT INTO tsh_visits_referrals (
            --tshvr_id,
            tshvr_visit,
            tshvr_referrer,
            tshvr_visitors,
            tshvr_visits
        )
        VALUES (
            --BIGSERIAL,
            visit_id,
            referrer_value,
            1,
            1
        )
        RETURNING tshvr_id INTO referral_id;

        INSERT INTO tsh_visits_referrals_visitors (
            tshvrv_referral,
            tshvrv_ip
        )
        VALUES (
            referral_id,
            ip_value
        );

        INSERT INTO tsh_visits_browsers (
            --tshvb_id,
            tshvb_visit,
            tshvb_browser,
            tshvb_number
        )
        VALUES (
            --BIGSERIAL,
            visit_id,
            browser_value,
            1
        );

        INSERT INTO tsh_visits_platforms (
            --tshvp_id,
            tshvp_visit,
            tshvp_platform,
            tshvp_number
        )
        VALUES (
            --BIGSERIAL,
            visit_id,
            platform_value,
            1
        );

    ELSE

        IF c_same_tshv_visitor(visit_id, ip_value) THEN

            UPDATE tsh_visits
            SET tshv_visits = tshv_visits + 1
            WHERE tshv_id = visit_id;

        ELSE

            UPDATE tsh_visits
            SET tshv_visitors = tshv_visitors + 1,
                tshv_visits = tshv_visits + 1
            WHERE tshv_id = visit_id;

            INSERT INTO tsh_visits_visitors (
                tshvv_visit,
                tshvv_ip,
                tshvv_browser,
                tshvv_platform
            )
            VALUES (
                visit_id,
                ip_value,
                browser_value,
                platform_value
            );

            visit_browser_id := s_tshv_browser_id_by_visit_browser(
                visit_id,
                browser_value
            );

            IF visit_browser_id IS NULL THEN

                INSERT INTO tsh_visits_browsers (
                    --tshvb_id,
                    tshvb_visit,
                    tshvb_browser,
                    tshvb_number
                )
                VALUES (
                    --BIGSERIAL,
                    visit_id,
                    browser_value,
                    1
                );

            ELSE

                UPDATE tsh_visits_browsers
                SET tshvb_number = tshvb_number + 1
                WHERE tshvb_id = visit_browser_id;

            END IF;

            visit_platform_id := s_tshv_platform_id_by_visit_platform(
                visit_id,
                platform_value
            );

            IF visit_platform_id IS NULL THEN

                INSERT INTO tsh_visits_platforms (
                    --tshvp_id,
                    tshvp_visit,
                    tshvp_platform,
                    tshvp_number
                )
                VALUES (
                    --BIGSERIAL,
                    visit_id,
                    platform_value,
                    1
                );

            ELSE

                UPDATE tsh_visits_platforms
                SET tshvp_number = tshvp_number + 1
                WHERE tshvp_id = visit_platform_id;

            END IF;

        END IF;

        referral_id := s_tshv_referral_id_by_visit_referrer(
            visit_id,
            referrer_value
        );

        IF referral_id IS NULL THEN

            INSERT INTO tsh_visits_referrals (
                --tshvr_id,
                tshvr_visit,
                tshvr_referrer,
                tshvr_visitors,
                tshvr_visits
            )
            VALUES (
                -- BIGSERIAL,
                visit_id,
                referrer_value,
                1,
                1
            )
            RETURNING tshvr_id INTO referral_id;

            INSERT INTO tsh_visits_referrals_visitors (
                tshvrv_referral,
                tshvrv_ip
            )
            VALUES (
                referral_id,
                ip_value
            );

        ELSE

            IF c_same_tshvr_visitor(referral_id, ip_value) THEN

                UPDATE tsh_visits_referrals
                SET tshvr_visits = tshvr_visits + 1
                WHERE tshvr_id = referral_id;

            ELSE

                UPDATE tsh_visits_referrals
                SET tshvr_visitors = tshvr_visitors + 1,
                    tshvr_visits = tshvr_visits + 1
                WHERE tshvr_id = referral_id;

                INSERT INTO tsh_visits_referrals_visitors (
                    tshvrv_referral,
                    tshvrv_ip
                )
                VALUES (
                    referral_id,
                    ip_value
                );

            END IF;

        END IF;

    END IF;

    -- Store global statistics too
    IF lang_value <> '' AND route_value <> '' THEN

        PERFORM i_stats_visit(
            '',
            route_value,
            ip_value,
            referrer_value,
            browser_value,
            platform_value
        );

        PERFORM i_stats_visit(
            lang_value,
            '',
            ip_value,
            referrer_value,
            browser_value,
            platform_value
        );

        PERFORM i_stats_visit(
            '',
            '',
            ip_value,
            referrer_value,
            browser_value,
            platform_value
        );

    END IF;

END;

$$;
