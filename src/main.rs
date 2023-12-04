use std::process::Command;
use std::process::exit;

fn main() {
    let dir_path = String::from("/home/heitor/Documents/repos/quero-boot/quero_bolsa");
    
    get_commits(&dir_path);
    get_changed_files(&dir_path);
}

fn get_changed_files(repo_path: &String){
    let changed_files = Command::new("git")
        .current_dir(repo_path)
        .arg("log")
        .arg("--shortstat")
        .arg("--since=\"Jan 1 2023\"")
        .arg("--until=\"Dez 2 2023\"")
        .output()
        .expect("failed to execute process");

     // Check if the command was successful
     if changed_files.status.success() {
        let stdout = String::from_utf8_lossy(&changed_files.stdout);
        println!("Git changed files:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&changed_files.stderr);
        eprintln!("Git command failed:\n{}", stderr);
        exit(1);
    }
}

fn get_commits(repo_path: &String){
    let commits = Command::new("git")
        .current_dir(repo_path)
        .arg("rev-list")
        .arg("--count")
        .arg("--since=\"Jan 1 2023\"")
        .arg("--before=\"Dez 2 2023\"")
        .arg("--all")
        .arg("--no-merges")
        .output()
        .expect("failed to execute process");
    // Check if the command was successful
    if commits.status.success() {
        let stdout = String::from_utf8_lossy(&commits.stdout);
        println!("Git commits:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&commits.stderr);
        eprintln!("Git command failed:\n{}", stderr);
        exit(1);
    }
}
