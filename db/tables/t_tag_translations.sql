
CREATE TABLE t_tag_translations (

    ttt_id         BIGSERIAL   PRIMARY KEY,
    ttt_tag        BIGINT      NOT NULL
                               REFERENCES t_tags ON DELETE CASCADE,
    ttt_lang       BIGINT      NOT NULL
                               REFERENCES t_languages ON DELETE CASCADE,
    ttt_name       TEXT        NOT NULL
                               CHECK(e_is_tag_name(ttt_name)),
    ttt_permalink  TEXT        NOT NULL
                               CHECK(e_is_permalink(ttt_permalink)),
    ttt_translator BIGINT      NOT NULL
                               REFERENCES t_users ON DELETE CASCADE,
    ttt_date       TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(ttt_tag, ttt_lang),
    UNIQUE(ttt_lang, ttt_permalink)

);
