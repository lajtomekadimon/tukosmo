
CREATE TABLE t_languages (

    tl_id        BIGSERIAL   PRIMARY KEY,
    tl_lang_code BIGINT      NOT NULL REFERENCES t_lang_codes,
    tl_name      TEXT        NOT NULL,
    tl_lang      BIGINT      NOT NULL REFERENCES t_lang_codes,
    tl_date      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(tl_lang_code, tl_lang)

);
