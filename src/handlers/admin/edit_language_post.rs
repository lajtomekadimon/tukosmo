use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
use std::fmt;

use crate::handlers::admin::admin_handler::admin_handler;
use crate::database::aw_edit_language::aw_edit_language;


impl<'de> Deserialize<'de> for FormData {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<FormData, D::Error>
    where D: Deserializer<'de>
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = FormData;

            fn expecting(
                &self,
                formatter: &mut fmt::Formatter,
            ) -> fmt::Result {
                formatter.write_str("MESSAGE")  // TODO: expecting "a ___"
            }

            fn visit_map<V>(
                self,
                mut map: V
            ) -> Result<FormData, V::Error>
            where V: MapAccess<'de>
            {
                let mut language_id: i64 = 0;
                let mut lang_code: String = "".to_string();
                let mut lang_ids: Vec<i64> = Vec::default();
                let mut lang_names: Vec<String> = Vec::default();

                while let Some(key) = map.next_key()? {
                    match key {
                        "language_id" => {
                            language_id = map.next_value::<i64>()?;
                        }
                        "lang_code" => {
                            lang_code = map.next_value::<String>()?;
                        }
                        "lang_id" => {
                            lang_ids.push(map.next_value::<i64>()?);
                        }
                        "lang_name" => {
                            lang_names.push(map.next_value::<String>()?);
                        }
                        _ => unreachable!()
                    }
                }

                // TODO: unreachable!() if empty Vec or different lengths

                Ok(FormData {
                    language_id: language_id,
                    lang_code: lang_code,
                    lang_ids: lang_ids,
                    lang_names: lang_names,
                })
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

pub struct FormData {
    language_id: i64,
    lang_code: String,
    lang_ids: Vec<i64>,
    lang_names: Vec<String>,
}

pub async fn edit_language_post(
    req: HttpRequest,
    form: web::Form<FormData>,
    id: Identity,
) -> impl Responder {

    match admin_handler(req, id) {

        Ok((lang_code, _user_id)) => {

            let language_id = (form.language_id).clone();
            let lang_code_value = (form.lang_code).clone();
            let lang_ids = (form.lang_ids).clone();
            let lang_names = (form.lang_names).clone();

            if let Ok(_language_id2) = aw_edit_language(
                language_id,
                lang_code_value,
                lang_ids,
                lang_names,
            ) {
                let redirect_route = "/{lang}/admin/languages".replace("{lang}", &lang_code);

                HttpResponse::Found()
                    .header("Location", redirect_route)
                    .finish()
            } else {
                let redirect_route = "/{lang}/admin/edit_language?id={id}"
                    .replace("{lang}", &lang_code)
                    .replace("{id}", &language_id.to_string());
                // TODO: Show what failed!

                HttpResponse::Found()
                    .header("Location", redirect_route)
                    .finish()
            }
        }

        Err(r) => {r}

    }

}

