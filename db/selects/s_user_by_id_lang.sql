
CREATE OR REPLACE FUNCTION s_user_by_id_lang(
    user_id BIGINT,
    lang_id BIGINT  -- This is for multilang names
)

RETURNS "UserDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT (
    -- id
    tu_id,
    -- email
    tu_email,
    -- name
    tu_name,
    -- date
    tu_date
)::"UserDB"

FROM t_users

WHERE tu_id = user_id

LIMIT 1

$$;
