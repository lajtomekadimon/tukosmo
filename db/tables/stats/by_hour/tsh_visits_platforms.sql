
CREATE TABLE tsh_visits_platforms (

    tshvp_id       BIGSERIAL PRIMARY KEY,
    tshvp_visit    BIGINT    NOT NULL REFERENCES tsh_visits,
    tshvp_platform TEXT      NOT NULL,
    tshvp_number   BIGINT    NOT NULL,

    UNIQUE(tshvp_visit, tshvp_platform)

);
