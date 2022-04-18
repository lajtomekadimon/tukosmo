
CREATE OR REPLACE FUNCTION c_same_tshvr_visitor(
    referral_id BIGINT,
    ip_value TEXT
)

RETURNS BOOL

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT EXISTS(
    SELECT 1
    FROM tsh_visits_referrals_visitors
    WHERE tshvrv_referral = referral_id
        AND tshvrv_ip = ip_value
    LIMIT 1
)

$$;
