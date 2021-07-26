use actix_web::HttpRequest;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::auth::token_secret::token_secret;


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,  // user's session ID
    exp: usize,  // expiration time (as UTC timestamp)
}


pub fn current_session_json(req: HttpRequest) -> Value {

    let json_output: Value;

    if let Some(the_token) = req.headers().get("Authorization") {
        if let Ok(the_token_str) = the_token.to_str() {
            let mut parts = the_token_str.splitn(2, ' ');

            if let Some(the_name) = parts.next() {

                if the_name == "Bearer" {

                    if let Some(token) = parts.next() {

                        if let Ok(token_data) = decode::<Claims>(
                            &token,
                            &DecodingKey::from_secret(token_secret().as_ref()),
                            &Validation::new(Algorithm::HS256),
                        ) {

                            json_output = json!({
                                "success": true,
                                "id": token_data.claims.id  // user ID
                            });

                        } else {

                            json_output = json!({
                                "success": false,
                                "error": "ERROR_UUID",
                                "temp": "Token couldn't be decoded properly"
                                // TODO: Control different token errors.
                            });

                        }

                    } else {

                        json_output = json!({
                            "success": false,
                            "error": "ERROR_UUID",
                            "temp": "Authorization credentials can't be found"
                        });

                    }

                } else {

                    json_output = json!({
                        "success": false,
                        "error": "ERROR_UUID",
                        "temp": "Authorization type must be Bearer"
                    });

                }

            } else {

                json_output = json!({
                    "success": false,
                    "error": "ERROR_UUID",
                    "temp": "Authorization type can't be found"
                });

            }

        } else {
            
            json_output = json!({
                "success": false,
                "error": "ERROR_UUID",
                "temp": "Token couldn't be converted to string."
            });

        }
    } else {

        json_output = json!({
            "success": false,
            "error": "ERROR_UUID",
            "temp": "Authorization header not found."
        });
        
    }

    json_output
}
