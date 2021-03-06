
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
    admin BOOL,
    suspended BOOL,
    date TEXT
);

CREATE TYPE "SessionDB" AS (
    ip TEXT,
    browser TEXT,
    platform TEXT,
    date TEXT
);

CREATE TYPE "FileDB" AS (
    id BIGINT,
    name TEXT,
    ext TEXT,
    bytes BIGINT,
    title TEXT,
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

CREATE TYPE "TagDB" AS (
    id BIGINT,
    trans_id BIGINT,
    lang "LanguageDB",
    name TEXT,
    permalink TEXT,
    author BIGINT,
    author_name TEXT,
    translator BIGINT,
    translator_name TEXT,
    date TEXT,
    date_trans TEXT
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
    tags "TagDB"[],
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

CREATE TYPE "HTTPDataDB" AS (
    ip TEXT,
    referrer TEXT,
    browser TEXT,
    platform TEXT
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

/*-----------*/

CREATE TYPE "StatsVisitsDB" AS (
    date TEXT,
    visitors TEXT,
    visits TEXT
);

CREATE TYPE "StatsVisitsReferralsDB" AS (
    referrers TEXT,
    visitors TEXT,
    visits TEXT
);

CREATE TYPE "StatsVisitsBrowsersDB" AS (
    browsers TEXT,
    visitors TEXT
);

CREATE TYPE "StatsVisitsPlatformsDB" AS (
    platforms TEXT,
    visitors TEXT
);

CREATE TYPE "StatsServerDB" AS (
    date TEXT,
    uploaded TEXT,
    downloaded TEXT,
    disk_used TEXT,
    disk_free TEXT,
    cpu TEXT,
    memory TEXT
);
