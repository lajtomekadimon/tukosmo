
CREATE TABLE tsh_server (

    tshs_date TIMESTAMPTZ PRIMARY KEY,  -- Example: 2022-03-22 19:00
    
    tshs_uploaded BIGINT NOT NULL,
    tshs_downloaded BIGINT NOT NULL,

    tshs_disk_used BIGINT NOT NULL,
    tshs_disk_free BIGINT NOT NULL,

    tshs_cpu DOUBLE PRECISION NOT NULL,

    tshs_memory DOUBLE PRECISION NOT NULL

);
