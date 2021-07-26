use actix_web::{post, web, HttpRequest, Responder};
use serde_json::{Value, json};

use crate::auth::current_session::current_session;
use crate::auth::current_session_json::current_session_json;
use crate::database::aj_update::aj_update;


#[post("/update")]
async fn handler_api_json_update(

    req: HttpRequest,

    json_input: web::Json<Value>

) -> impl Responder {

    let json_output: Value;

    if let Some(session_id) = current_session(req.clone()) {

        if let Ok(json_result) = aj_update(json_input.0, session_id) {

            json_output = json_result;

        // ERROR: Database error
        } else {

            json_output = json!({
                "success": false,
                "error": "ERROR_UUID",
                "temp": "Database error"
            });

        }

    // ERROR: Auth error
    } else {

        json_output = current_session_json(req.clone());

    }


    web::Json(json_output)
}

