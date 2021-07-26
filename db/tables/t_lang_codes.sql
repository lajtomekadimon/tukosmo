
CREATE TABLE t_lang_codes (

    tlc_id   BIGSERIAL   PRIMARY KEY,
    tlc_code TEXT        NOT NULL UNIQUE CHECK (CHAR_LENGTH(tlc_code) BETWEEN 2 AND 5),
                                       /* TODO: Check is azAZ_- */
    tlc_date TIMESTAMPTZ NOT NULL DEFAULT NOW()

);
