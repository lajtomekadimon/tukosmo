use markup;

use crate::files::static_files;


markup::define! {
    FaviconAdminMeta() {
        link[
            rel = "apple-touch-icon",
            sizes = "57x57",
            href = static_files::FAVICONADMIN_APPLE_ICON_57X57,
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "60x60",
            href = static_files::FAVICONADMIN_APPLE_ICON_60X60,
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "72x72",
            href = static_files::FAVICONADMIN_APPLE_ICON_72X72,
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "76x76",
            href = static_files::FAVICONADMIN_APPLE_ICON_76X76,
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "114x114",
            href = static_files::FAVICONADMIN_APPLE_ICON_114X114,
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "120x120",
            href = static_files::FAVICONADMIN_APPLE_ICON_120X120,
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "144x144",
            href = static_files::FAVICONADMIN_APPLE_ICON_144X144,
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "152x152",
            href = static_files::FAVICONADMIN_APPLE_ICON_152X152,
        ];
        link[
            rel = "apple-touch-icon",
            sizes = "180x180",
            href = static_files::FAVICONADMIN_APPLE_ICON_180X180,
        ];

        link[
            rel = "icon",
            type = "image/png",
            sizes = "32x32",
            href = static_files::FAVICONADMIN_32X32,
        ];
        link[
            rel = "icon",
            type = "image/png",
            sizes = "96x96",
            href = static_files::FAVICONADMIN_96X96,
        ];
        link[
            rel = "icon",
            type = "image/png",
            sizes = "16x16",
            href = static_files::FAVICONADMIN_16X16,
        ];

        // Android Icon favicons
        link[
            rel = "manifest",
            href = static_files::FAVICONADMIN_MANIFEST,
        ];
    }
}

