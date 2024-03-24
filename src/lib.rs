use anyhow::anyhow;
use aws_sdk_sts::Client;
use serde::Serialize;
use url::Url;

use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct UrlCredentials {
    #[serde(rename="sessionId")]
    pub session_id: String,
    #[serde(rename="sessionKey")]
    pub session_key: String,
    #[serde(rename="sessionToken")]
    pub session_token: String,
}

impl UrlCredentials {
    pub fn new(id: String, key: String, token: String) -> Self {
        Self {
            session_id: id,
            session_key: key,
            session_token: token
        }
    }
}

pub async fn get_caller_identity(client: &Client) -> Result<String, anyhow::Error> {
    let response = client.get_caller_identity().send().await?;
    let user_arn = response.arn().ok_or_else(||anyhow!("Unable to parse the user ARN"))?;
    Ok(user_arn.to_string())
}

pub async fn assume_role(role: String, account: String, session: String, c: Client) -> Result<(), anyhow::Error> {
    let result = c.assume_role()
        .role_arn(format!("arn:aws:iam::{}:role/{}", account, role))
        .role_session_name(session)
        .send()
        .await?;

    let id = result.credentials()
        .ok_or_else(||anyhow!("Unable to get the Access Key ID"))?
        .access_key_id();
    let key = result.credentials()
        .ok_or_else(||anyhow!("Unable to get the Secret Access Key"))?
        .secret_access_key();
    let token = result.credentials()
        .ok_or_else(||anyhow!("Unable to get the Session Token"))?
        .session_token();
    // TODO: This can be cleaned up
    let creds = UrlCredentials::new(id.to_string(), key.to_string(), token.to_string());
    let json_creds = serde_json::to_string(&creds)?;
    let url_creds = urlencoding::encode(json_creds.as_str());

    // TODO: Make the duration configurable
    let req_url = format!("https://signin.aws.amazon.com/federation?Action=getSigninToken&SessionDuration=43200&Session={}",url_creds);
    let req_url = Url::parse(req_url.as_str())?;

    let resp = reqwest::get(req_url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    
    let signin_token = resp.get("SigninToken").ok_or_else(||anyhow!("Unable to get the SigninToken from the inital request"))?;

    let url = format!("https://signin.aws.amazon.com/federation?Action=login&Issuer=Example.org&Destination={}&SigninToken={}", urlencoding::encode("https://console.aws.amazon.com/"), signin_token);
    println!("{}", url);

    Ok(())
}
