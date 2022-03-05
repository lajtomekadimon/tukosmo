
CREATE TABLE t_tags (

    tt_id          BIGSERIAL   PRIMARY KEY,
    tt_date        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    tt_author      BIGINT      NOT NULL
                               REFERENCES t_users ON DELETE CASCADE

);

