use super::jira_sdk::sdk::JiraTicket;
use regex;

#[derive(Debug, PartialEq)]
pub struct Subissue<'a> {
    pub number: usize,
    pub resolution: Resolution,
    pub body: &'a str,
}

impl<'a> Subissue<'a> {
    pub fn fmt_oneline(&self) -> String {
        let re = regex::Regex::new(r"^\*\(\d+\)\*\s").unwrap();
        let cln_body = re.replace(self.body, "");
        let re = regex::Regex::new(r"\b\W+\b").unwrap();
        let short_desc = re.replace_all(&cln_body, " ");

        let flag_str = match self.resolution {
            Resolution::Resolved | Resolution::WontFix => "[R] ",
            Resolution::Reopened => "[O] ",
            Resolution::Closed => "[C] ",
            _ => "",
        };

        format!("{} {}{}", self.number, flag_str, short_desc)
    }
}

pub struct FlowTicket {
    ticket: JiraTicket,
}

impl FlowTicket {
    pub fn new(ticket: JiraTicket) -> Self {
        Self { ticket }
    }

    pub fn subissues(&self) -> impl Iterator<Item = Subissue> {
        self.ticket.fields.comment.comments.iter().filter_map(|cmt| {
            let body = cmt.body.as_str();
            if let Some(number) = subissue_number(body) {
                let resolution = last_resolution(body);
                Some(Subissue { body, number, resolution })
            } else {
                None
            }
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Resolution {
    Resolved,
    Reopened,
    Closed,
    Open,
    WontFix,
}

impl Resolution {
    fn r#match(s: &str) -> Self {
        match s {
            "CLOSED" => Resolution::Closed,
            "REOPENED" => Resolution::Reopened,
            "WONT FIX" | "WON'T FIX" => Resolution::WontFix,
            "RESOLVED" | "FIXED" | "IMPLEMENTED" => Resolution::Resolved,
            ____________________________________________ => Resolution::Open,
        }
    }
}

fn subissue_number(s: &str) -> Option<usize> {
    let re = regex::Regex::new(r"(?ms)^\*\((\d+)\)\*\s(.*)$").unwrap();

    if let Some(num_match) = re.captures(s).and_then(|caps| caps.get(1)) {
        return Some(num_match.as_str().parse::<usize>().unwrap());
    }

    None
}

fn last_resolution(s: &str) -> Resolution {
    let re =
        regex::Regex::new(r"\bRESOLVED|WON'T FIX|WONT FIX|CLOSED|REOPENED|IMPLEMENTED|FIXED\b")
            .unwrap();

    match re.captures_iter(s).last() {
        Some(res) => Resolution::r#match(res.get(0).unwrap().as_str()),
        None => Resolution::Open,
    }
}

#[test]
fn various_last_resolutions() {
    assert_eq!(
        Resolution::Reopened,
        last_resolution("text RESOLVED text2 REOPENED text3")
    );
    assert_eq!(Resolution::Open, last_resolution("text"));
    assert_eq!(Resolution::Open, last_resolution("closed"));
    assert_eq!(Resolution::Closed, last_resolution("CLOSED"));
    assert_eq!(
        Resolution::Resolved,
        last_resolution("*(1)* txt [~] RESOLVED")
    );
    assert_eq!(Resolution::Open, last_resolution("a\n\nb\n\na CLOSE text"));
    assert_eq!(
        Resolution::WontFix,
        last_resolution("a\n\nb\n\na WONT FIX\n\ntext")
    );
    assert_eq!(
        Resolution::WontFix,
        last_resolution("a\n\nb\n\na WON'T FIX\n\ntext")
    );
}
