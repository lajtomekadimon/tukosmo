
CREATE OR REPLACE FUNCTION s_user_by_id(
    user_id BIGINT
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
    -- suspended
    tu_suspended,
    -- date
    tu_date
)::"UserDB"

FROM t_users

WHERE tu_id = user_id

LIMIT 1

$$;
