
CREATE OR REPLACE FUNCTION s_file_by_id(
    file_id BIGINT
)

RETURNS "FileDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT (
    -- id
    tf_id,

    -- name
    tf_name,

    -- ext
    tf_ext,

    -- author
    tf_author,

    -- date
    tf_date::TEXT
)::"FileDB"

FROM t_files

WHERE tf_id = file_id

LIMIT 1

$$;
