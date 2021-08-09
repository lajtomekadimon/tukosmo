use actix_web::{post, web, HttpRequest, Responder};
use serde::Deserialize;
use serde_json::{Value, json};
use bcrypt::verify;

//use bcrypt::{DEFAULT_COST, hash, verify};
//let hashed = hash("hunter2", DEFAULT_COST)?;
//let valid = verify("hunter2", &hashed)?;

use crate::database::s_user_password_by_email::s_user_password_by_email;
use crate::auth::create_token::create_token;


#[derive(Deserialize)]
struct FormData {
    email: String,
    password: String,
}

#[post("/login")]
async fn login(

    req: HttpRequest,
    form: web::Form<FormData>

) -> impl Responder {

    let json_output: Value;
    let email_value = (form.email).clone();
    let password_value = (form.password).clone();

    if let Ok(selected_password_value) = s_user_password_by_email(
        email_value.clone()
    ) {

        if let Ok(password_is_correct) = verify(
            password_value,
            &selected_password_value
        ) {

            // Login!!
            if password_is_correct {

                if let Some(the_user_agent) = req.headers().get("User-Agent") {

                    if let Ok(user_agent_value) = the_user_agent.to_str() {

                        json_output = create_token(
                            email_value.clone(),
                            user_agent_value.to_string()
                        );
                    
                    } else {

                        json_output = json!({
                            "success": false,
                            "error": "ERROR_UUID",
                            "temp": "(?)"
                        });

                    }

                } else {

                    json_output = json!({
                        "success": false,
                        "error": "ERROR_UUID",
                        "temp": "(?)"
                    });

                }

            // ERROR: Password is not correct
            } else {

                json_output = json!({
                    "success": false,
                    "error": "ERROR_UUID",
                    "temp": "Password is not correct"
                });

            }

        // ERROR: Bcrypt error (password is not correct?)
        } else {

            json_output = json!({
                "success": false,
                "error": "ERROR_UUID",
                "temp": "Bcrypt error"
            });

        }

    // ERROR: Database error
    } else {

        json_output = json!({
            "success": false,
            "error": "ERROR_UUID",
            "temp": "Database error"
        });

    }


    web::Json(json_output)
}

