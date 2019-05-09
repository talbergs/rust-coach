use jira_lines::*;

fn main() -> Result<(), Box<std::error::Error>> {
    let conf = config::Config::form_args()?;
    let client = jira_sdk::client::Client::new(conf.url, conf.un, conf.pw);

    let ticket = client.fetch_ticket(&conf.ticket);
    let flow_ticket = flow::FlowTicket::new(ticket);

    for subissue in flow_ticket.subissues() {
        println!("{}", subissue.fmt_oneline());
    }

    Ok(())
}
