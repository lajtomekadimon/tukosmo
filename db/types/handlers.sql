
CREATE TYPE "AccountAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "DashboardAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "EditLanguagePostAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "EditPostAH" AS (
    data "AdminDataDB",
    post "PostDB"
);

CREATE TYPE "EditPostPostAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "FilesAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "LanguagesAH" AS (
    data "AdminDataDB",
    languages "LanguageDB"[]
);

CREATE TYPE "LoginAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "LoginPostAH" AS (
    data "AdminDataDB",
    session UUID
);

CREATE TYPE "LogoutAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "NewLanguageAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "NewLanguagePostAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "NewPostAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "NewPostPostAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "PagesAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "PostsAH" AS (
    data "AdminDataDB",
    posts "PostDB"[],
    total_posts BIGINT
);

CREATE TYPE "ServerAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "SessionsAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "StatisticsAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "TukosmoAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "UsersAH" AS (
    data "AdminDataDB"
);

CREATE TYPE "WebsiteAH" AS (
    data "AdminDataDB"
);


/*----------------------*/


CREATE TYPE "BlogWH" AS (
    data "WebsiteDataDB",
    posts "PostDB"[],
    total_posts BIGINT
);

CREATE TYPE "BlogPostWH" AS (
    data "WebsiteDataDB",
    post "PostDB"
    --post_translations "PostDB"[]
);

CREATE TYPE "PageWH" AS (
    data "WebsiteDataDB"
    --page "PageDB",
    --page_translations "PageDB"[]
);

