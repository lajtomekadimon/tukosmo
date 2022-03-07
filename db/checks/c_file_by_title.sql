
CREATE OR REPLACE FUNCTION c_file_by_title(
    file_title TEXT
)

RETURNS BOOL

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT EXISTS(
    SELECT 1
    FROM t_files
    WHERE tf_title = file_title
    LIMIT 1
)

$$;