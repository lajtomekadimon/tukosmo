
CREATE OR REPLACE FUNCTION s_user_by_session_lang(

    session_id UUID,

    lang_id BIGINT  -- This is for multilang names

)

RETURNS TABLE(
    id BIGINT,
    email TEXT,
    name TEXT
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT
    tu_id AS id,
    tu_email AS email,
    tu_name AS name
FROM t_users
INNER JOIN t_sessions
ON ts_user = tu_id
    AND ts_id = session_id
LIMIT 1

$$;
