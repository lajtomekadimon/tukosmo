
CREATE TABLE tsh_visits (

    tshv_id       BIGSERIAL   PRIMARY KEY,
    tshv_date     TIMESTAMPTZ NOT NULL,  -- Example: 2022-03-22 19:00
    tshv_lang     TEXT        NOT NULL,  -- '' for global
    tshv_route    TEXT        NOT NULL,  -- '' for global
    tshv_visitors BIGINT      NOT NULL,
    tshv_visits   BIGINT      NOT NULL,

    UNIQUE(tshv_date, tshv_lang, tshv_route)

);

CREATE INDEX idx_tsv_lang_route ON tsh_visits (tshv_lang, tshv_route);
