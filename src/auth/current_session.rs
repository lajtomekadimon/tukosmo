use actix_web::HttpRequest;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use uuid::Uuid;

use crate::auth::token_secret::token_secret;


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,  // user's session ID
    exp: usize,  // expiration time (as UTC timestamp)
}


pub fn current_session(req: HttpRequest) -> Option<Uuid> {

    let current_session_id: Option<Uuid>;

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

                            if let Ok(session_id) = Uuid::parse_str(
                                &token_data.claims.id
                            ) {

                                current_session_id = Some(session_id);

                            } else {

                                current_session_id = None;

                            }

                        } else {

                            current_session_id = None;

                        }

                    } else {

                        current_session_id = None;

                    }

                } else {

                    current_session_id = None;

                }

            } else {

                current_session_id = None;

            }

        } else {
            
            current_session_id = None;

        }
    } else {

        current_session_id = None;
        
    }

    current_session_id
}
