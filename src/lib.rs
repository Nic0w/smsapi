use std::borrow::Cow;

use reqwest::{blocking, StatusCode};

const API_URL: &str = "https://smsapi.free-mobile.fr/sendmsg";

pub struct SmsApi<'a> {

    client: blocking::Client,

    user: &'a str,
    password: &'a str
}
#[derive(Debug)]
pub enum Result {

    /// 200 : Le SMS a été envoyé sur votre mobile.
    Success,

    /// 400 : Un des paramètres obligatoires est manquant.
    MissingParameter,

    /// 402 : Trop de SMS ont été envoyés en trop peu de temps.
    RateLimited,

    /// 403 : Le service n'est pas activé sur l'espace abonné, ou login / clé incorrect.
    AuthenticationError,

    /// 500 : Erreur côté serveur. Veuillez réessayer ultérieurement.
    ServerError,

    Other(Cow<'static, str>)
}

impl Result {
    pub fn other<T: Into<Cow<'static, str>>>(error: T) -> Self {
        Result::Other(error.into())
    }
}

impl From<StatusCode> for Result {
    fn from(status: StatusCode) -> Self {
        
        match status.as_u16() {

            200 => Result::Success,
            400 => Result::MissingParameter,
            402 => Result::RateLimited,
            403 => Result::AuthenticationError,
            500 => Result::ServerError,

            status => Result::other(format!("unknown status code: {}", status))
        }
    }
}

impl SmsApi<'_> {
    pub fn send(&self, message: &str) -> Result {

        let request = self.client.get(API_URL)
            .query(&[
                ("user", self.user),
                ("pass", self.password),
                ("msg", message)
            ])
            .build();

        if let Ok(request) = request {

            match self.client.execute(request) {

                Ok(response) => response.status().into(),

                Err(e) => 
                    Result::other(format!("i/o error - {}", e)),
            }
        }
        else {
            Result::other("could not encode message")
        }
    }
}

pub fn authenticate<'a>(user: &'a str, password: &'a str) -> SmsApi<'a> {
    SmsApi { 
        client: blocking::Client::new(), 
        user, password
    }
}