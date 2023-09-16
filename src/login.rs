use reqwest::{blocking::Client, Error};
use serde::{self, Serialize, Deserialize};
use reqwest;
use serde_json;

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    #[serde(alias = "bak:ApiVersion")]
    api_version: String,
    #[serde(alias = "bak:AppVersion")]
    app_vesion: String,
    #[serde(alias = "bak:UserId")]
    user_id: String,
    access_token: String,
    refresh_token: String,
    id_token: Option<String>,
    token_type: String,
    expires_in: i32,
    scope: String
}

pub fn login(username: &String, password: &String, url: &reqwest::Url, is_debug_run: bool) -> Result<LoginResponse, Error> {
    let client = Client::new();
    let url = format!("{url}/api/login");
    let request_body = format!("client_id=ANDR&grant_type=password&username={username}&password={password}");
    let request = client
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(request_body)
        .send();
    
    
    //let text = request.unwrap().text().unwrap();
    //println!("{text:?}");
    
    
    match request {
        Ok(response) => {
            match response.json::<LoginResponse>() {
                Ok(parsed_json) => Ok(parsed_json),
                Err(why) => Err(why)
            }
        }
        Err(why) => Err(why)
    }
    
}

pub fn get_marks(api: &LoginResponse, url: reqwest::Url, is_debug_run: bool) {
    let client = Client::new();
    let url = format!("{url}/api/3/marks");
    let request = client.post(url)
        .bearer_auth(&api.access_token)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send();

    match request {
        Ok(_) => todo!(),
        Err(_) => todo!()
    }
}