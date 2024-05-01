use anyhow::anyhow;
use aws_sdk_sts::Client;
use serde::Serialize;
use url::Url;

use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct UrlCredentials {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "sessionKey")]
    pub session_key: String,
    #[serde(rename = "sessionToken")]
    pub session_token: String,
}

impl UrlCredentials {
    pub fn new(id: String, key: String, token: String) -> Self {
        Self {
            session_id: id,
            session_key: key,
            session_token: token,
        }
    }
}

impl UrlCredentials {
    fn to_json(&self) -> Result<String, anyhow::Error> {
        Ok(serde_json::to_string(&self)?)
    }
}

pub async fn get_caller_identity(client: &Client) -> Result<String, anyhow::Error> {
    let response = client.get_caller_identity().send().await?;
    let user_arn = response
        .arn()
        .ok_or_else(|| anyhow!("Unable to parse the user ARN"))?;
    Ok(user_arn.to_string())
}

pub async fn assume_role(
    role: String,
    account: String,
    session: String,
    timeout: i32,
    c: Client,
) -> Result<String, anyhow::Error> {
    let result = c
        .assume_role()
        .role_arn(format!("arn:aws:iam::{}:role/{}", account, role))
        .role_session_name(session)
        .send()
        .await?;

    let id = result
        .credentials()
        .ok_or_else(|| anyhow!("Unable to get the Access Key ID from the assume_role call"))?
        .access_key_id();
    let key = result
        .credentials()
        .ok_or_else(|| anyhow!("Unable to get the Secret Access Key from the assume_role call"))?
        .secret_access_key();
    let token = result
        .credentials()
        .ok_or_else(|| anyhow!("Unable to get the Session Token from the assume_role call"))?
        .session_token();
    // converts the new credentials to a json string
    let creds = UrlCredentials::new(id.to_string(), key.to_string(), token.to_string())
        .to_json()?;
    // encodes the credentials
    let url_creds = urlencoding::encode(creds.as_str());

    let req_url = format!("https://signin.aws.amazon.com/federation?Action=getSigninToken&SessionDuration={}&Session={}",timeout,url_creds);
    let req_url = Url::parse(req_url.as_str())?;

    let resp = reqwest::get(req_url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    let signin_token = resp
        .get("SigninToken")
        .ok_or_else(|| anyhow!("Unable to get the SigninToken from the inital request"))?;

    // return the url
    Ok(format!("https://signin.aws.amazon.com/federation?Action=login&Issuer=Example.org&Destination={}&SigninToken={}", urlencoding::encode("https://console.aws.amazon.com/"), signin_token))

}
