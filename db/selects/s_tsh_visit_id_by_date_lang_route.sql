
CREATE OR REPLACE FUNCTION s_tsh_visit_id_by_date_lang_route(
    date_value TIMESTAMPTZ,
    lang_value TEXT,
    route_value TEXT
)

RETURNS BIGINT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tshv_id
FROM tsh_visits
WHERE tshv_date = date_value
    AND tshv_lang = lang_value
    AND tshv_route = route_value
LIMIT 1

$$;
