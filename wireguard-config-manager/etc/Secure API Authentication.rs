use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// Generate token for authorized users
fn generate_token(user: &str) -> String {
    let claims = Claims { sub: user.to_string(), exp: 10000000000 };
    encode(&Header::default(), &claims, &EncodingKey::from_secret("your_secret".as_ref())).unwrap()
}

// Validate incoming token
fn validate_token(token: &str) -> bool {
    decode::<Claims>(token, &DecodingKey::from_secret("your_secret".as_ref()), &Validation::default()).is_ok()
}