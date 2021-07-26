use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use uuid::Uuid;
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};

use crate::database::new_user_session::new_user_session;

use crate::auth::token_secret::token_secret;


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,  // user's session ID
    exp: usize,  // expiration time (as UTC timestamp)
}


pub fn create_token(
    email_value: String,
    user_agent_value: String
) -> Value {

    let json_output: Value;

    // Login!
    if let Ok(session_id) = new_user_session(email_value.clone(), user_agent_value.clone()) {
        let encode_buffer_value = &mut Uuid::encode_buffer();
        let session_id_up = session_id
            .to_simple()
            .encode_upper(encode_buffer_value);

        let my_claims = Claims {
            id: session_id_up.to_owned(),  // user's session ID
            exp: 10000000000  // expiration time (as UTC timestamp)
        };

        // TODO: Use ES256
        let header = Header::new(Algorithm::HS256);

        // Token created!
        if let Ok(token_value) = encode(
            &header,
            &my_claims,
            &EncodingKey::from_secret(token_secret().as_ref())
        ) {

            json_output = json!({
                "success": true,
                "token": token_value
            });

        // ERROR: JWT encoding didn't work
        } else {

            json_output = json!({
                "success": false,
                "error": "ERROR_UUID",
                "temp": "JWT encoding didn't work"
            });

        }

    // ERROR: Database error
    } else {

        json_output = json!({
            "success": false,
            "error": "ERROR_UUID",
            "temp": "Email was not found"
        });

    }

    json_output

}
