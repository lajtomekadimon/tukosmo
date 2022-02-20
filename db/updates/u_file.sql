
CREATE OR REPLACE FUNCTION u_file(
    file_id BIGINT,
    file_title TEXT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    UPDATE t_files
    SET tf_title = file_title
    WHERE tf_id = file_id;

    RETURN file_id;

END;

$$;
