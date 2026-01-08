// A simple CLI in Rust
// Import the env module to get the command line arguments
use std::io::{self, Write};
use std::process;
use std::process::Command;

fn main() {
    println!("Hello Strawhat! Please tell me what do you want to do?
    1. This is my first use.
    2. I need to download the working files.
    3. I want to upload my work.
    4. I entered by accident. Exit.");
    print!("> ");

    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            println!("Welcome! Please let me introduce the program to you first.");
            intro();
        }
        "2" => {
            fetch();
            pull();
        }
        "3" => {
            push();
        }
        "4" => {
            println!("Exiting. See you later!");
            process::exit(0);
        }
        _ => {
            println!("Invalid choice. Please choose a valid option.");
        }
    }
}

fn intro() {
    println!("Hello Strawhat!
    Let me introduce what would you need to do 
    make sure that no conflit will happen when 
    you work on editing with other. 
    
    1. You will have to clone the repository first. Don't worry, I'll do it now for you.");
    clone();
    println!("You should find now under the current folder a new folder named \"VideoEditing_2026\"
    WARNING: If you are working, please let others know before you have done any work!
    Because editing files are not text files, git cannot merge the changes automatically.
    
    2. After cloning, import the project to DaVinci Resolve. You can start working on your part now.
    Remember to EXPORT the project (NOT JUST SAVE) after you have done your work.
    
    3. You can close this program while working. When you are done, please run this program again but to UPLOAD your work.
    4. EVERY TIME before working on the file in the future, please run this program first to DOWNLOAD the latest version
    so you won't have any conflict with others' work.
    
    Please note that this program is JUST a tool to make you sync your work with others without needing to learn git commands.
    
    If you still have any questions or need help, please contact Cloumy074 on Discord. Thank you!");
}

fn clone() {
    let clone_cmd = Command::new("git")
        .arg("clone")
        .arg("https://github.com/Cloumy074/Test_Davinci.git")
        .output()
        .expect("Failed to execute git clone command");
    
    println!("{}", String::from_utf8_lossy(&clone_cmd.stdout));
}

fn fetch() {

}

fn pull() {

}

fn push() {

}
