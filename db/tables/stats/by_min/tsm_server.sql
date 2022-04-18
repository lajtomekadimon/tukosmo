
CREATE TABLE tsm_server (

    tsms_date TIMESTAMPTZ PRIMARY KEY,  -- Example: 2022-03-22 19:08
    
    tsms_uploaded BIGINT NOT NULL,
    tsms_downloaded BIGINT NOT NULL,

    tsms_disk_used BIGINT NOT NULL,
    tsms_disk_free BIGINT NOT NULL,

    tsms_cpu DOUBLE PRECISION NOT NULL,

    tsms_memory DOUBLE PRECISION NOT NULL

);
