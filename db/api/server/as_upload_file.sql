
CREATE TYPE "SpiUploadFile" AS (
    author_id BIGINT,
    filename TEXT
);

CREATE OR REPLACE FUNCTION as_upload_file(
    r "SpiUploadFile"
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    file_id BIGINT;

BEGIN

    -- Check file name
    IF NOT e_is_filename(r.filename) THEN
        PERFORM err_wrong_filename();
    END IF;

    -- Check file name is unique
    IF c_file_by_name(r.filename) THEN
        PERFORM err_filename_already_exists();
    END IF;

    file_id := i_file(
        r.filename,
        r.author_id
    );

    RETURN file_id;

END;

$$;
