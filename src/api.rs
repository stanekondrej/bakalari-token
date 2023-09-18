use reqwest::{self, Error};
use serde::{self, Deserialize};


#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    #[serde(alias = "bak:ApiVersion")]
    pub api_version: String,
    #[serde(alias = "bak:AppVersion")]
    pub app_vesion: String,
    #[serde(alias = "bak:UserId")]
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: Option<String>,
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String
}

#[derive(Debug)]
pub struct Api {
    pub access_token: String,
    pub refresh_token: String,
    pub school_url: reqwest::Url,
    reqwest_client: reqwest::blocking::Client
}

impl Api {
    pub fn login(username: &String, password: &String, url: reqwest::Url, is_debug_run: bool) -> Result<Api, Error> {
        let client = reqwest::blocking::Client::new();
        let formatted_url = format!("{url}/api/login");
        let request_body = format!("client_id=ANDR&grant_type=password&username={username}&password={password}");
        let request = client
            .post(&formatted_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(request_body)
            .send();
        
        
        //let text = request.unwrap().text().unwrap();
        //println!("{text:?}");
        
        
        let parsed_json = match request {
            Ok(response) => {
                match response.json::<LoginResponse>() {
                    Ok(parsed_json) => Ok(parsed_json),
                    Err(why) => Err(why)
                }
            }
            Err(why) => Err(why)
        };

        match parsed_json {
            Ok(details) => {
                Ok(Api { access_token: details.access_token, refresh_token: details.refresh_token, school_url: url, reqwest_client: client })
            }
            Err(why) => Err(why)
        }
    }
}