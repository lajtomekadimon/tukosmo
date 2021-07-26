
CREATE TABLE t_post_ids (

    tpi_id   BIGSERIAL   PRIMARY KEY,
    tpi_date TIMESTAMPTZ NOT NULL DEFAULT NOW()

);
