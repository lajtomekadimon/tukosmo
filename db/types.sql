
CREATE TYPE "UserDB" AS (
    id BIGINT,
    email TEXT,
    name TEXT
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
    lang BIGINT,
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

