
CREATE TABLE t_posts (

    tp_id        BIGSERIAL   PRIMARY KEY,
    tp_post      BIGINT      NOT NULL REFERENCES t_post_ids,
    tp_lang      BIGINT      NOT NULL REFERENCES t_languages,
    tp_title     TEXT        NOT NULL,
    tp_body      TEXT        NOT NULL,
    tp_permalink TEXT        NOT NULL UNIQUE,  /* TODO: Check max length */
    tp_author    BIGINT      NOT NULL REFERENCES t_users,
    tp_date      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    tp_draft     BOOL        NOT NULL,
    tp_deleted   BOOL        NOT NULL,

    UNIQUE(tp_post, tp_lang)

);
