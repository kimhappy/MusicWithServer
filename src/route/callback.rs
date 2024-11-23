use rocket::response::Redirect;
use crate::env;

// TODO: Verify state
#[rocket::get("/callback?<code>&<state>")]
pub async fn get_callback(
    code : &str,
    state: &str
) -> Result< Redirect, String > {
    let params = [
        ("code" , code ),
        ("state", state)
    ];
    let qs           = serde_urlencoded::to_string(params).map_err(|_| "Failed to serialize token data")?;
    let redirect_uri = format!("{}/?{}", *env::REDIRECT_APP_URI, qs);
    Ok(Redirect::to(redirect_uri))
}
