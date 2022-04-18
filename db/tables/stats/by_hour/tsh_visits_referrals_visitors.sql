
CREATE TABLE tsh_visits_referrals_visitors (

    tshvrv_referral BIGINT NOT NULL REFERENCES tsh_visits_referrals,
    tshvrv_ip       TEXT   NOT NULL,

    UNIQUE(tshvrv_referral, tshvrv_ip)

);

CREATE INDEX idx_tshvrv_ip ON tsh_visits_referrals_visitors (tshvrv_ip);
