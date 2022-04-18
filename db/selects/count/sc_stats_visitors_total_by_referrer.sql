
CREATE OR REPLACE FUNCTION sc_stats_visitors_total_by_referrer(
    referrer_value TEXT
)

RETURNS BIGINT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT COUNT(*)
FROM (
    SELECT DISTINCT tshvrv_ip
    FROM tsh_visits_referrals
    INNER JOIN tsh_visits_referrals_visitors
    ON tshvr_id = tshvrv_referral
        AND tshvr_referrer = referrer_value
) temp

$$;
