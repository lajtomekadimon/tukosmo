
CREATE OR REPLACE FUNCTION i_file(
    filename TEXT,
    author_id BIGINT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    file_ext_value TEXT;

    file_id BIGINT;

BEGIN

    file_ext_value := SUBSTRING(filename FROM '\.([^\.]*)$');

    IF file_ext_value = '' THEN
        file_ext_value = NULL;
    END IF;

    INSERT INTO t_files (
        --tf_id,
        tf_name,
        tf_ext,
        tf_title,
        tf_author
        --tf_date
    )
    VALUES (
        -- BIGSERIAL (autoincrement)
        filename,
        file_ext_value,
        filename,
        author_id
        --NOW()
    )
    RETURNING tf_id INTO file_id;

    RETURN file_id;

END;

$$;
