
CREATE OR REPLACE FUNCTION s_tshv_referral_id_by_visit_referrer(
    visit_id BIGINT,
    referrer_value TEXT
)

RETURNS BIGINT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tshvr_id
FROM tsh_visits_referrals
WHERE tshvr_visit = visit_id
    AND tshvr_referrer = referrer_value
LIMIT 1

$$;
