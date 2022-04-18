
pub fn staticf_route(
    route: &str,
    codename: &str,
) -> String {
    route.replace("{codename}", codename)
}


// Website's favicon
pub const FAVICON_APPLE_ICON_57X57: &'static str =
    "/static/{codename}/favicon/apple-icon-57x57.png";
pub const FAVICON_APPLE_ICON_60X60: &'static str =
    "/static/{codename}/favicon/apple-icon-60x60.png";
pub const FAVICON_APPLE_ICON_72X72: &'static str =
    "/static/{codename}/favicon/apple-icon-72x72.png";
pub const FAVICON_APPLE_ICON_76X76: &'static str =
    "/static/{codename}/favicon/apple-icon-76x76.png";
pub const FAVICON_APPLE_ICON_114X114: &'static str =
    "/static/{codename}/favicon/apple-icon-114x114.png";
pub const FAVICON_APPLE_ICON_120X120: &'static str =
    "/static/{codename}/favicon/apple-icon-120x120.png";
pub const FAVICON_APPLE_ICON_144X144: &'static str =
    "/static/{codename}/favicon/apple-icon-144x144.png";
pub const FAVICON_APPLE_ICON_152X152: &'static str =
    "/static/{codename}/favicon/apple-icon-152x152.png";
pub const FAVICON_APPLE_ICON_180X180: &'static str =
    "/static/{codename}/favicon/apple-icon-180x180.png";
pub const FAVICON_32X32: &'static str =
    "/static/{codename}/favicon/favicon-32x32.png";
pub const FAVICON_96X96: &'static str =
    "/static/{codename}/favicon/favicon-96x96.png";
pub const FAVICON_16X16: &'static str =
    "/static/{codename}/favicon/favicon-16x16.png";
pub const FAVICON_MANIFEST: &'static str =
    "/static/{codename}/favicon/manifest.json";


// TAP's favicon
pub const FAVICONADMIN_APPLE_ICON_57X57: &'static str =
    "/static/{codename}/faviconadmin/apple-icon-57x57.png";
pub const FAVICONADMIN_APPLE_ICON_60X60: &'static str =
    "/static/{codename}/faviconadmin/apple-icon-60x60.png";
pub const FAVICONADMIN_APPLE_ICON_72X72: &'static str =
    "/static/{codename}/faviconadmin/apple-icon-72x72.png";
pub const FAVICONADMIN_APPLE_ICON_76X76: &'static str =
    "/static/{codename}/faviconadmin/apple-icon-76x76.png";
pub const FAVICONADMIN_APPLE_ICON_114X114: &'static str =
    "/static/{codename}/faviconadmin/apple-icon-114x114.png";
pub const FAVICONADMIN_APPLE_ICON_120X120: &'static str =
    "/static/{codename}/faviconadmin/apple-icon-120x120.png";
pub const FAVICONADMIN_APPLE_ICON_144X144: &'static str =
    "/static/{codename}/faviconadmin/apple-icon-144x144.png";
pub const FAVICONADMIN_APPLE_ICON_152X152: &'static str =
    "/static/{codename}/faviconadmin/apple-icon-152x152.png";
pub const FAVICONADMIN_APPLE_ICON_180X180: &'static str =
    "/static/{codename}/faviconadmin/apple-icon-180x180.png";
pub const FAVICONADMIN_32X32: &'static str =
    "/static/{codename}/faviconadmin/favicon-32x32.png";
pub const FAVICONADMIN_96X96: &'static str =
    "/static/{codename}/faviconadmin/favicon-96x96.png";
pub const FAVICONADMIN_16X16: &'static str =
    "/static/{codename}/faviconadmin/favicon-16x16.png";
pub const FAVICONADMIN_MANIFEST: &'static str =
    "/static/{codename}/faviconadmin/manifest.json";


// CSS files
pub const CSS_WEBSITE: &'static str =
    "/static/{codename}/bundles/bundle.css";
pub const CSS_ADMIN: &'static str =
    "/static/{codename}/bundles/bundle.admin.css";


// JavaScript files
pub const JS_WEBSITE: &'static str =
    "/static/{codename}/bundles/bundle.js";
pub const JS_ADMIN: &'static str =
    "/static/{codename}/bundles/bundle.admin.js";
pub const JS_TINYMCE: &'static str =
    "/static/{codename}/js/external/tinymce/tinymce.min.js";
pub const JS_CHARTJS: &'static str =
    "/static/{codename}/js/external/chart.min.js";


// Images
pub const TUKOSMO_LOGO_TAP_40: &'static str =
    "/static/{codename}/img/tukosmo-logo-tap-40.png";
pub const FIREFOX_TABS_LIGHT: &'static str =
    "/static/{codename}/img/firefox-tabs-light.png";
pub const FIREFOX_TABS_DARK: &'static str =
    "/static/{codename}/img/firefox-tabs-dark.png";
pub const CHROME_TABS_LIGHT: &'static str =
    "/static/{codename}/img/chrome-tabs-light.png";
pub const CHROME_TABS_DARK: &'static str =
    "/static/{codename}/img/chrome-tabs-dark.png";

