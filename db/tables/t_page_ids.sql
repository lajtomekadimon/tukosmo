
CREATE TABLE t_page_ids (

    tpi_id   BIGSERIAL   PRIMARY KEY,
    tpi_date TIMESTAMPTZ NOT NULL DEFAULT NOW()

);
