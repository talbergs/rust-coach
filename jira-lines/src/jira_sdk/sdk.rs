use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketFixVersion {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketStatus {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraComment {
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketCommentField {
    pub comments: Vec<JiraComment>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketAssignee {
    key: String,
    display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketFields {
    pub summary: String,
    pub assignee: JiraTicketAssignee,
    pub fix_versions: Vec<JiraTicketFixVersion>,
    pub status: JiraTicketStatus,
    pub comment: JiraTicketCommentField,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicket {
    pub fields: JiraTicketFields,
    key: String,
}

impl JiraTicket {
    pub fn from_str(s: &String) -> Self {
        serde_json::from_str(s).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraErrors {
    error_messages: Vec<String>,
}
impl JiraErrors {
    pub fn print(&self) {
        for e in self.error_messages.iter() {
            println!("{}", e);
        }
    }
}

