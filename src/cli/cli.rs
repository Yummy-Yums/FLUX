use std::collections::HashMap;

pub fn cli_help_display() {
    // TODO- refactor to use hashmaps
    let list_of_commands = ["tasks", "list"];
    let help_msg = r#"
   Usage [<COMMAND>] [OPTION]

   Pass in <COMMAND> to execute. add the username and passwords options tto authenticate
   With no [OPTION] or <COMMAND>, It displays the interactive menu

   OPTIONS:
     --help     display this help and exit.
     --username pass in username
     --password pass in password
     --view-completed view completed tasks

   COMMANDS:
     tasks  view completed,pending or all tasks
     list   <to be continued>

    "#
    .lines()
    .map(|l| {
        let res = match l {
            l if l.contains("--") => l,
            l if list_of_commands.iter().any(|cmd| l.contains(cmd)) => l,
            _ => l.trim_start(),
        };
        res
    })
    .collect::<Vec<&str>>()
    .join("\n");

    println!("{}", help_msg);
}
