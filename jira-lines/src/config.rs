use getopts::Options;
use getopts::Fail;
use std::env;

pub struct Config {
    /// Basic auth will be used if this is not empty.
    pub un: Option<String>,
    /// Empty password is possible.
    pub pw: Option<String>,
    /// Ticket id: JRA-123
    pub ticket: String,
    /// Jiras installation Unified Resource Location
    pub url: String,
}

impl Config {
    pub fn form_args() -> Result<Config, Fail> {
        let args: Vec<String> = env::args().collect();
        let mut opts = Options::new();

        opts.optopt("u", "username", "Credential", "USER");
        opts.optopt("p", "password", "Credential", "PASS");
        opts.reqopt("i", "installation", "Jira installation address", "URL");
        opts.reqopt("t", "ticket", "Jira ticket, i.e. JRA-123", "TICKET");
        opts.optflag("h", "help", "print this help menu");

        if args[1] == "-h" || args[1] == "--help" {
            print!("{}", opts.usage(""));
        }

        opts.parse(&args[1..]).and_then(|m| {
            let ticket = m.opt_str("ticket").unwrap();
            let url = m.opt_str("installation").unwrap();
            let pw = m.opt_str("password");
            let un = m.opt_str("password");

            Ok(Config { url, ticket, un, pw })
        })
    }
}


