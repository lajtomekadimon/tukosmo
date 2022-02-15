use markup;

use crate::files::static_files::{
    staticf_route,
    FAVICON_APPLE_ICON_57X57,
    FAVICON_APPLE_ICON_60X60,
    FAVICON_APPLE_ICON_72X72,
    FAVICON_APPLE_ICON_76X76,
    FAVICON_APPLE_ICON_114X114,
    FAVICON_APPLE_ICON_120X120,
    FAVICON_APPLE_ICON_144X144,
    FAVICON_APPLE_ICON_152X152,
    FAVICON_APPLE_ICON_180X180,
    FAVICON_32X32,
    FAVICON_96X96,
    FAVICON_16X16,
    FAVICON_MANIFEST,
};


markup::define! {
    FaviconMeta<'a>(
        codename: &'a str,
    ) {
        link[
            rel = "apple-touch-icon",
            sizes = "57x57",
            href = staticf_route(FAVICON_APPLE_ICON_57X57, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "60x60",
            href = staticf_route(FAVICON_APPLE_ICON_60X60, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "72x72",
            href = staticf_route(FAVICON_APPLE_ICON_72X72, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "76x76",
            href = staticf_route(FAVICON_APPLE_ICON_76X76, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "114x114",
            href = staticf_route(FAVICON_APPLE_ICON_114X114, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "120x120",
            href = staticf_route(FAVICON_APPLE_ICON_120X120, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "144x144",
            href = staticf_route(FAVICON_APPLE_ICON_144X144, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "152x152",
            href = staticf_route(FAVICON_APPLE_ICON_152X152, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "180x180",
            href = staticf_route(FAVICON_APPLE_ICON_180X180, codename),
        ];

        link[
            rel = "icon",
            type = "image/png",
            sizes = "32x32",
            href = staticf_route(FAVICON_32X32, codename),
        ];
        link[
            rel = "icon",
            type = "image/png",
            sizes = "96x96",
            href = staticf_route(FAVICON_96X96, codename),
        ];
        link[
            rel = "icon",
            type = "image/png",
            sizes = "16x16",
            href = staticf_route(FAVICON_16X16, codename),
        ];

        // Android Icon favicons
        link[
            rel = "manifest",
            href = staticf_route(FAVICON_MANIFEST, codename),
        ];
    }
}

