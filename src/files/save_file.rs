use actix_web::{web, Error};
use actix_multipart::Multipart;
use uuid::Uuid;
use std::io::Write;
use futures_util::TryStreamExt as _;  // in save_file(), but not sure if works
use std::path::Path;
use std::ffi::OsStr;


pub async fn save_file(
    mut payload: Multipart,
) -> Result<String, Error> {

    let mut rvalue: Result<String, Error> = Ok("".to_string());

    // Iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field
            .content_disposition();
            //.ok_or_else(|| HttpResponse::BadRequest().finish())?;
            // I think this HttpResponse should be a different value (type)...

        let filename = content_disposition.get_filename().map_or_else(
            || Uuid::new_v4().to_string(),
            |f| sanitize_filename::sanitize(f),
        );
        // TODO: Return the original filename too so UUID is not used.
        
        // If there's a file
        if filename != "" {
            let uuid_text = Uuid::new_v4().to_string();

            let filename_value = match Path::new(&filename)
                .extension()
                .and_then(OsStr::to_str)
            {
                Some(filename_ext) => format!(
                    "{}.{}",
                    uuid_text,
                    filename_ext.to_lowercase(),
                ),
                None => uuid_text,
            };

            let filepath = format!("./files/{}", filename_value);

            // File::create is blocking operation, use threadpool
            let mut f = web::block(|| std::fs::File::create(filepath)).await?;

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.try_next().await? {
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(
                    move || f.as_ref().unwrap().write_all(&chunk).map(|_| f)
                ).await?.unwrap();
            }

            rvalue = Ok(filename_value);
        }
    }

    rvalue

}
