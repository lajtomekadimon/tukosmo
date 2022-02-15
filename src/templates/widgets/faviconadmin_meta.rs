use markup;

use crate::files::static_files::{
    staticf_route,
    FAVICONADMIN_APPLE_ICON_57X57,
    FAVICONADMIN_APPLE_ICON_60X60,
    FAVICONADMIN_APPLE_ICON_72X72,
    FAVICONADMIN_APPLE_ICON_76X76,
    FAVICONADMIN_APPLE_ICON_114X114,
    FAVICONADMIN_APPLE_ICON_120X120,
    FAVICONADMIN_APPLE_ICON_144X144,
    FAVICONADMIN_APPLE_ICON_152X152,
    FAVICONADMIN_APPLE_ICON_180X180,
    FAVICONADMIN_32X32,
    FAVICONADMIN_96X96,
    FAVICONADMIN_16X16,
    FAVICONADMIN_MANIFEST,
};


markup::define! {
    FaviconAdminMeta<'a>(
        codename: &'a str,
    ) {
        link[
            rel = "apple-touch-icon",
            sizes = "57x57",
            href = staticf_route(FAVICONADMIN_APPLE_ICON_57X57, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "60x60",
            href = staticf_route(FAVICONADMIN_APPLE_ICON_60X60, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "72x72",
            href = staticf_route(FAVICONADMIN_APPLE_ICON_72X72, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "76x76",
            href = staticf_route(FAVICONADMIN_APPLE_ICON_76X76, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "114x114",
            href = staticf_route(FAVICONADMIN_APPLE_ICON_114X114, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "120x120",
            href = staticf_route(FAVICONADMIN_APPLE_ICON_120X120, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "144x144",
            href = staticf_route(FAVICONADMIN_APPLE_ICON_144X144, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "152x152",
            href = staticf_route(FAVICONADMIN_APPLE_ICON_152X152, codename),
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "180x180",
            href = staticf_route(FAVICONADMIN_APPLE_ICON_180X180, codename),
        ];

        link[
            rel = "icon",
            type = "image/png",
            sizes = "32x32",
            href = staticf_route(FAVICONADMIN_32X32, codename),
        ];
        link[
            rel = "icon",
            type = "image/png",
            sizes = "96x96",
            href = staticf_route(FAVICONADMIN_96X96, codename),
        ];
        link[
            rel = "icon",
            type = "image/png",
            sizes = "16x16",
            href = staticf_route(FAVICONADMIN_16X16, codename),
        ];

        // Android Icon favicons
        link[
            rel = "manifest",
            href = staticf_route(FAVICONADMIN_MANIFEST, codename),
        ];
    }
}

