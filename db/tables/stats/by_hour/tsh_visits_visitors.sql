
CREATE TABLE tsh_visits_visitors (

    tshvv_visit    BIGINT NOT NULL REFERENCES tsh_visits,
    tshvv_ip       TEXT   NOT NULL,
    tshvv_browser  TEXT   NOT NULL,
    tshvv_platform TEXT   NOT NULL,

    UNIQUE(tshvv_visit, tshvv_ip)

);

CREATE INDEX idx_tshvv_ip ON tsh_visits_visitors (tshvv_ip);
