use actix_web::{post, web, Responder};
use serde_json::{Value, json};

use crate::database::aj_query::aj_query;


#[post("/query")]
async fn query(

    json_input: web::Json<Value>

) -> impl Responder {

    let json_output: Value;

    if let Ok(json_result) = aj_query(json_input.0) {

        json_output = json_result;

    } else {

        json_output = json!({
            "success": false,
            "error": "ERROR_UUID",
            "temp": "Database error"
        });

    }


    web::Json(json_output)
}

