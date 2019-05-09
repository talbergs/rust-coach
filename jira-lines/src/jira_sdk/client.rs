use reqwest;
use super::sdk::*;

pub struct Client {
    client: reqwest::Client,
    url: String,
    un: Option<String>,
    pw: Option<String>,
}

impl Client {

    pub fn new(url: String, un: Option<String>, pw: Option<String>) -> Self {
        let client = reqwest::Client::new();

        Client { client, url, un, pw }
    }

    fn url(&self, resource: String) -> reqwest::Url {
        reqwest::Url::parse(&format!("{}/rest/api/latest/{}", self.url, resource)).unwrap()
    }

    fn fetch(&self, resource: String) -> reqwest::Response {
        if self.un.is_some() {
            self.client
                .get(self.url(resource))
                .basic_auth(self.un.clone().unwrap(), self.pw.clone())
                .send()
                .unwrap()
        } else {
            self.client
                .get(self.url(resource))
                .send()
                .unwrap()
        }
    }

    pub fn fetch_ticket(&self, ticket: &String) -> JiraTicket {
        let mut resp = self.fetch(format!("issue/{}", ticket));

        JiraTicket::from_str(&resp.text().unwrap())
    }
}
