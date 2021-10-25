
CREATE TABLE t_language_names (

    tln_id        BIGSERIAL   PRIMARY KEY,
    tln_lang      BIGINT      NOT NULL
                              REFERENCES t_languages ON DELETE CASCADE,
    tln_name      TEXT        NOT NULL
                              CHECK(e_is_lang_name(tln_name)),
    tln_name_lang BIGINT      NOT NULL
                              REFERENCES t_languages ON DELETE CASCADE,
    tln_date      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(tln_lang, tln_name_lang)

);
