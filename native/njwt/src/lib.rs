use rustler::{Encoder, Env, Error, Term};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, dangerous_insecure_decode};


mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        atom error;
        atom __true__ = "true";
        atom __false__ = "false";
    }
}

rustler::rustler_export_nifs! {
    "Elixir.Njwt",
    [
        ("peek_payload", 1, peek_payload),
        ("validate_token", 2, validate_token)
    ],
    None
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: i64
}

fn peek_payload<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    // TODO: Find some way of allowing the user to pass custom claims - this is very stupid
    let token: String = args[0].decode()?;
    let payload = dangerous_insecure_decode::<Claims>(&token);

    let result = match payload {
        Ok(data) => Ok((atoms::ok(), data.claims.id).encode(env)),
        Err(error) => Ok((atoms::error(), format!("{}", error)).encode(env))
    };
    result
}

fn validate_token<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let token: String = args[0].decode()?;
    let key: String = args[1].decode()?;


    let validation = Validation{validate_exp: false, algorithms: vec![Algorithm::HS512], ..Default::default()};
    let payload = decode::<Claims>(&token, &DecodingKey::from_secret(key.as_ref()), &validation);
    let result = match payload {
        Ok(_data) => Ok((atoms::ok(), atoms::__true__()).encode(env)),
        Err(error) => Ok((atoms::error(), format!("{}", error)).encode(env))
    };
    result
}
