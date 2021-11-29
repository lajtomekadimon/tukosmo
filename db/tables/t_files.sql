
CREATE TABLE t_files (

    tf_id        BIGSERIAL   PRIMARY KEY,
    tf_name      TEXT        NOT NULL UNIQUE CHECK (e_is_filename(tf_name)),
    tf_ext       TEXT,
    tf_author    BIGINT      NOT NULL
                             REFERENCES t_users ON DELETE CASCADE,
    tf_date      TIMESTAMPTZ NOT NULL DEFAULT NOW()

);

