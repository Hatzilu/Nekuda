use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::process::{exit, Command};
use std::io::{stdin, stdout, Error, Write};


static TEMP_CLONE_PATH: &'static str = "/tmp/dotfiles";
const IGNORED_FILES: HashSet<&str> = [".git", "README.md", ".gitignore"].into_iter().collect();
/**
 * Exit early if dependencies like git are not installed on this machine. 
 */
pub fn check_necessary_dependencies() {
    let git_command = Command::new("git")
    .arg("--version")
    .output()
    .expect("Failed to execute git command");

    let is_git_installed = git_command.status.success();

    if is_git_installed == false {
        eprintln!("git is not installed, please review the necessary dependencies to run this program.");
        exit(1);
    }
}

pub fn install() {
    let mut s=String::new();
    print!("Please provide a url to your dotfiles repository: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");

    let repo_url = s.trim();

    create_temporary_dir(&TEMP_CLONE_PATH).expect("Failed to create temporary path for git repository");

    Command::new("git")
    .arg("clone")
    .arg(&repo_url)
    .arg(&TEMP_CLONE_PATH)
    .output()
    .expect("Failed to clone dotfiles repository");

    upgrade();

}
pub fn upgrade() {
    let p = Path::new(&TEMP_CLONE_PATH);
    if Path::exists(&p) == false {

    }
    let install_script_path_str = format!("{}/{}",&TEMP_CLONE_PATH,"install.sh");
    let install_script_path = Path::new(&install_script_path_str);

    if Path::exists(install_script_path) {
        Command::new("sh")
        .arg(&install_script_path_str)
        .output()
        .expect("Failed to run install script");
    }

    let cloned_directory = fs::read_dir(&TEMP_CLONE_PATH).expect("Failed to read cloned diretory");

    for f in cloned_directory.into_iter() {
        let file = f.expect("Failed to read file");

        let file_name = file.file_name();
        let s = file_name.to_str().expect("Failed to get file name");

        if IGNORED_FILES.contains(s) {
            continue;
        }

    }


    println!("TODO upgrade command");
}

pub fn help(){
    println!("TODO help message");
}

fn create_temporary_dir(path_str: &str) -> Result<&Path,Error> {
    fs::create_dir(path_str)?;

    let path = Path::new(path_str);
    Ok(&path)
}