
CREATE TYPE "BlogPostWRequest" AS (
    req "WebsiteRequest",
    permalink TEXT
);

CREATE TYPE "BlogPostWResponse" AS (
    data "WebsiteDataDB",
    post "PostDB"
);


CREATE OR REPLACE FUNCTION aww_blog_post(
    r "BlogPostWRequest"
)

RETURNS "BlogPostWResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "WebsiteDataDB";

    language_of_user BIGINT;

    post "PostDB";

BEGIN

    d := s_website_handler_data(r.req);

    IF d IS NULL THEN

        RAISE EXCEPTION 'TODO: Wrong request.';

    END IF;

    language_of_user := (d.lang).id;

    post := s_post_by_lang_permalink(
        language_of_user,
        r.permalink
    );

    IF post IS NULL THEN

        RAISE EXCEPTION 'TODO: Post permalink is not correct.';

    END IF;

    -- User is logged in
    RETURN (
        -- data
        d,

        -- post
        post
    )::"BlogPostWResponse";

END;

$$;
