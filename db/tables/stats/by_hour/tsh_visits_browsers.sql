
CREATE TABLE tsh_visits_browsers (

    tshvb_id      BIGSERIAL PRIMARY KEY,
    tshvb_visit   BIGINT    NOT NULL REFERENCES tsh_visits,
    tshvb_browser TEXT      NOT NULL,
    tshvb_number  BIGINT    NOT NULL,

    UNIQUE(tshvb_visit, tshvb_browser)

);
