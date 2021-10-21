
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

CREATE TYPE "AdminDataDB" AS (
    userd "UserDB",
    lang "LanguageDB",
    languages "LanguageDB"[]
);

CREATE TYPE "WebsiteDataDB" AS (
    userd "UserDB",
    lang "LanguageDB",
    languages "LanguageDB"[]
);

