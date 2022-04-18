
CREATE TABLE tsh_visits_referrals (

    tshvr_id       BIGSERIAL PRIMARY KEY,
    tshvr_visit    BIGINT    NOT NULL REFERENCES tsh_visits,
    tshvr_referrer TEXT      NOT NULL,
    tshvr_visitors BIGINT    NOT NULL,
    tshvr_visits   BIGINT    NOT NULL,

    UNIQUE(tshvr_visit, tshvr_referrer)

);
