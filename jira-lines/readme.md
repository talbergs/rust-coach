Some jira interaction via cli
godzilla
only fetches a single issue and outputs subissues in list
usage:
```
jira_lines "http://jira_installation.com" -i "http://jira.localhost:8080" -t "JRA-123"
jira_lines "http://jira_installation.com" -i "http://jira.localhost:8080" -t "JRA-123" -u "U/n" -p "P/w"
```

todo:
- [ ] cli --help flag not working
- [ ] too much unwaps
- [ ] in sdk::client::fetch() there is codeduplication
- [ ] add PGP support for credential values
