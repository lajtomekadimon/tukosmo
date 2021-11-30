
CREATE OR REPLACE FUNCTION s_file_by_id(
    file_id BIGINT,
    language_of_user BIGINT
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

    -- author_name
    COALESCE(
        s_user_name_by_user_lang(
            tu_id,
            language_of_user
        ),
        tu_name
    ),

    -- date
    tf_date::TEXT
)::"FileDB"

FROM t_files

INNER JOIN t_users
ON tf_author = tu_id

WHERE tf_id = file_id

LIMIT 1

$$;
