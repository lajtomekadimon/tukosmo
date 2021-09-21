
CREATE TABLE t_languages (

    tl_id   BIGSERIAL   PRIMARY KEY,
    tl_code TEXT        NOT NULL UNIQUE
                        CHECK (CHAR_LENGTH(tl_code) BETWEEN 2 AND 5),
                        /* TODO: Check is azAZ_- */
    tl_date TIMESTAMPTZ NOT NULL DEFAULT NOW()

);
