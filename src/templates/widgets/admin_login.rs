use markup;

use crate::files::static_files::FAVICON_96X96;
use crate::handlers::{
    admin::{
        login_get::ra_login,
        forgotten_password_get::ra_forgotten_password,
    },
    website::home_get::rw_home,
};
use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::WebsiteDataDB;


markup::define! {
    AdminLogin<'a, BodyContent: markup::Render>(
        content: BodyContent,
        data: &'a WebsiteDataDB,
        t: &'a TranslateI18N,
        forgotten_password: &'a bool,
    ) {

        section[class = "hero is-success is-fullheight"] {
            div[class = "hero-body"] {
                div[class = "container has-text-centered"] {
                    div[class = "column is-4 is-offset-4"] {
                        div[class = "box"] {
                            figure[class = "avatar"] {
                                div[class = "avatar-container"] {
                                    img[src = FAVICON_96X96];
                                }
                            }

                            @content
                        }

                        @if **forgotten_password {
                            p[class = "has-text-grey has-text-left ml-3"] {
                                a[
                                    href = &ra_login(&data.lang.code),
                                ] {
                                    @t.login_k_noun
                                }
                            }
                        } else {
                            p[class = "has-text-grey has-text-left ml-3"] {
                                a[
                                    href = &ra_forgotten_password(
                                        &data.lang.code,
                                    ),
                                ] {
                                    @t.forgotten_password
                                }
                            }
                        }

                        p[class = "has-text-grey has-text-left mt-3 ml-3"] {
                            a[href = rw_home(&data.lang.code)] {
                                {t.go_back_to_w_website
                                    .replace(
                                        "{website}",
                                        &data.website_title,
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }

    }
}

