use std::fs;

fn main() {
    fs::read_to_string("/etc/passwd")
        .map(|content| {
            println!("{:<20} {}", "USERNAME", "SHELL");
            println!("{:-<20} {:-<10}", "", "");
            let mut users = content.lines()
                .filter_map(|line| line.split_once(':'))
                .filter(|(_, rest)| rest.contains(':'))
                .map(|(user, rest)| (user, rest.rsplit_once(':').unwrap().1))
                .collect::<Vec<_>>();
            users.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
            users.into_iter()
                .for_each(|(user, shell)| println!("{:<20} {}", user, shell));
        })
        .unwrap_or_else(|_| eprintln!("Error: Could not read /etc/passwd"));
}
