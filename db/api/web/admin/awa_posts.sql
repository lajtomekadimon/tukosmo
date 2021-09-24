
CREATE OR REPLACE FUNCTION awa_posts(
    language_of_user BIGINT,
    results_number BIGINT,
    page_number BIGINT
)

RETURNS "PostDB"[]

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT s_posts(
    language_of_user,
    results_number,
    page_number
)

$$;
