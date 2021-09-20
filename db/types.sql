
CREATE TYPE "UserDB" AS (
    id BIGINT,
    email TEXT,
    name TEXT
);

CREATE TYPE "LanguageDB" AS (
    id BIGINT,
    code TEXT,
    name TEXT,
    date TEXT,  -- TIMESTAMPTZ?
    has_all_names BOOL
);

CREATE TYPE "NameDB" AS (
    name TEXT,
    lang "LanguageDB"
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

CREATE TYPE "DataDB" AS (
    userd "UserDB",
    lang "LanguageDB",
    languages "LanguageDB"[]
);

