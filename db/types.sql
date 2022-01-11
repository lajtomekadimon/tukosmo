
CREATE TYPE "AdminRequest" AS (
    session UUID,
    lang_code TEXT
);

CREATE TYPE "WebsiteRequest" AS (
    session UUID,
    lang_code TEXT
);

/*------------*/

CREATE TYPE "UserDB" AS (
    id BIGINT,
    email TEXT,
    name TEXT,
    date TEXT
);

CREATE TYPE "SessionDB" AS (
    user_agent TEXT,
    date TEXT
);

CREATE TYPE "FileDB" AS (
    id BIGINT,
    name TEXT,
    ext TEXT,
    route TEXT,
    author BIGINT,
    author_name TEXT,
    date TEXT
);

CREATE TYPE "LanguageDB" AS (
    id BIGINT,
    code TEXT,
    name TEXT,
    original_name TEXT,
    date TEXT,
    has_all_names BOOL
);

CREATE TYPE "NameDB" AS (
    name TEXT,
    lang "LanguageDB"
);

CREATE TYPE "LanguageWithNamesDB" AS (
    id BIGINT,
    code TEXT,
    name TEXT,
    original_name TEXT,
    date TEXT,
    has_all_names BOOL,
    names "NameDB"[]
);

CREATE TYPE "PostDB" AS (
    id BIGINT,
    featured_image "FileDB",
    trans_id BIGINT,
    lang "LanguageDB",
    title TEXT,
    description TEXT,
    body TEXT,
    permalink TEXT,
    author BIGINT,
    author_name TEXT,
    translator BIGINT,
    translator_name TEXT,
    date TEXT,
    date_trans TEXT,
    draft BOOL,
    deleted BOOL
);

CREATE TYPE "RouteDB" AS (
    lang "LanguageDB",
    route TEXT
);

CREATE TYPE "AdminDataDB" AS (
    userd "UserDB",
    lang "LanguageDB",
    languages "LanguageDB"[],
    website_title TEXT,
    website_subtitle TEXT,
    copyright_owner TEXT
);

CREATE TYPE "WebsiteDataDB" AS (
    userd "UserDB",
    lang "LanguageDB",
    languages "LanguageDB"[],
    website_title TEXT,
    website_subtitle TEXT,
    copyright_owner TEXT
);

