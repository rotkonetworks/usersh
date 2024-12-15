use std::fs;
use std::os::unix::fs::MetadataExt;

const HEADER: &str = "USER                     SHELL                ACTIVE";
const SEPARATOR: &str = "------------------------ -------------------- ------";

struct User {
    name: String,
    shell: String,
}

fn read_login_db() -> Vec<User> {
    fs::read_to_string("/etc/passwd")
        .map(|content| content
            .lines()
            .filter_map(|line| {
                let mut parts = line.split(':');
                Some(User {
                    name: parts.next()?.to_string(),
                    shell: parts.nth(5)?.to_string(),
                })
            })
            .collect())
        .unwrap_or_default()
}

fn get_logged_users() -> Vec<String> {
    fs::read_dir("/dev/pts")
        .map(|entries| entries
            .filter_map(|e| e.ok())
            .filter_map(|entry| {
                let uid = entry.metadata().ok()?.uid();
                fs::read_to_string("/etc/passwd")
                    .ok()?
                    .lines()
                    .find(|line| line.split(':').nth(2).and_then(|id| id.parse::<u32>().ok()) == Some(uid))
                    .map(|line| line.split(':').next().unwrap_or("").to_string())
            })
            .collect())
        .unwrap_or_default()
}

fn main() {
    println!("{}", HEADER);
    println!("{}", SEPARATOR);

    let active = get_logged_users();
    let mut users = read_login_db();
    users.sort_by(|a, b| a.shell.cmp(&b.shell).then(a.name.cmp(&b.name)));

    users.iter().for_each(|user| println!("{:<24} {:<20} {}", 
        user.name,
        user.shell,
        if active.contains(&user.name) { "*" } else { "" }
    ));
}
