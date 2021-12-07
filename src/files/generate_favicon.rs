use actix_web::{web, HttpResponse, Error};
use actix_multipart::Multipart;
use uuid::Uuid;
use std::io::Write;
use futures_util::TryStreamExt as _;  // in save_file(), but not sure if works
use std::path::Path;
use std::ffi::OsStr;
use std::fs;

use crate::files::resize_image::resize_image;


pub async fn save_file(
    mut payload: Multipart,
) -> Result<String, Error> {

    let mut rvalue: Result<String, Error> = Ok("".to_string());

    // Iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| HttpResponse::BadRequest().finish())?;
            // I think this HttpResponse should be a different value (type)...

        let filename = content_disposition.get_filename().map_or_else(
            || Uuid::new_v4().to_string(),
            |f| sanitize_filename::sanitize(f),
        );

        // If there's a file
        if filename != "" {
            match Path::new(&filename)
                .extension()
                .and_then(OsStr::to_str)
            {
                Some(filename_ext) => {
                    // Check PNG extension
                    if (filename_ext == "png") || (filename_ext == "PNG") {
                        // File::create is blocking operation, use threadpool
                        let mut f = web::block(
                            || std::fs::File::create(
                                "./temp/favicon/original-image.png"
                            )
                        ).await?;

                        // Field in turn is stream of *Bytes* object
                        while let Some(chunk) = field.try_next().await? {
                            // filesystem operations are blocking,
                            // we have to use threadpool
                            f = web::block(
                                move || f.write_all(&chunk).map(|_| f)
                            ).await?;
                        }

                        rvalue = Ok(filename);
                    }
                },
                None => {},
            }
        }
    }

    rvalue

}


pub async fn generate_favicon(
    payload: Multipart,
) -> bool {
    // Delete temp/favicon/*
    let temp_favicon_path = "./temp/favicon";
    fs::remove_dir_all(temp_favicon_path).unwrap();
    fs::create_dir(temp_favicon_path).unwrap();

    match save_file(payload).await {
        Ok(filename) => if filename != "" {
            let filepath = "./temp/favicon/original-image.png";

            // Create all favicons
            resize_image(
                filepath, 36, 36,
                "./temp/favicon/android-icon-36x36.png",
            );
            resize_image(
                filepath, 48, 48,
                "./temp/favicon/android-icon-48x48.png",
            );
            resize_image(
                filepath, 72, 72,
                "./temp/favicon/android-icon-72x72.png",
            );
            resize_image(
                filepath, 96, 96,
                "./temp/favicon/android-icon-96x96.png",
            );
            resize_image(
                filepath, 144, 144,
                "./temp/favicon/android-icon-144x144.png",
            );
            resize_image(
                filepath, 192, 192,
                "./temp/favicon/android-icon-192x192.png",
            );
            resize_image(
                filepath, 192, 192,
                "./temp/favicon/apple-icon.png",
            );
            resize_image(
                filepath, 57, 57,
                "./temp/favicon/apple-icon-57x57.png",
            );
            resize_image(
                filepath, 60, 60,
                "./temp/favicon/apple-icon-60x60.png",
            );
            resize_image(
                filepath, 72, 72,
                "./temp/favicon/apple-icon-72x72.png",
            );
            resize_image(
                filepath, 76, 76,
                "./temp/favicon/apple-icon-76x76.png",
            );
            resize_image(
                filepath, 114, 114,
                "./temp/favicon/apple-icon-114x114.png",
            );
            resize_image(
                filepath, 120, 120,
                "./temp/favicon/apple-icon-120x120.png",
            );
            resize_image(
                filepath, 144, 144,
                "./temp/favicon/apple-icon-144x144.png",
            );
            resize_image(
                filepath, 152, 152,
                "./temp/favicon/apple-icon-152x152.png",
            );
            resize_image(
                filepath, 180, 180,
                "./temp/favicon/apple-icon-180x180.png",
            );
            resize_image(
                filepath, 192, 192,
                "./temp/favicon/apple-icon-precomposed.png",
            );
            resize_image(
                filepath, 16, 16,
                "./temp/favicon/favicon-16x16.png",
            );
            resize_image(
                filepath, 32, 32,
                "./temp/favicon/favicon-32x32.png",
            );
            resize_image(
                filepath, 96, 96,
                "./temp/favicon/favicon-96x96.png",
            );

            // Delete favicon/
            let favicon_path = "./static/favicon";
            fs::remove_dir_all(favicon_path).unwrap();

            // Move generated favicons to favicon/
            fs::rename(
                temp_favicon_path,
                favicon_path,
            ).unwrap();

            // Copy manifest.json from faviconadmin/
            fs::copy(
                "./static/faviconadmin/manifest.json",
                "./static/favicon/manifest.json",
            ).unwrap();

            // Create temp/favicon/ again
            fs::create_dir(temp_favicon_path).unwrap();

            true
        } else {
            // TODO
            println!("Empty filename. 123");
            false
        },
        Err(e) => {
            // TODO
            println!("{}", e);
            false
        },
    }
}
