use crossterm::style::{Color, ResetColor, SetForegroundColor};
use crossterm::{Command, ExecutableCommand};
use std::io::{stdin, stdout, Stdout};
pub enum CLIPrint {
    Warning,
    Error,
    Info,
    Default,
}

impl CLIPrint {
    pub fn out(&self, role: &str, message: &str) {
        let mut stdout: Stdout = stdout();

        let color: Color = match self {
            Self::Warning => Color::DarkYellow,
            Self::Error => Color::DarkRed,
            Self::Info => Color::Cyan,
            Self::Default => Color::Grey,
        };

        let prefix: &str = match self {
            Self::Error => "ERROR: ",
            Self::Warning => "WARNING: ",
            Self::Info => "INFO: ",
            Self::Default => "",
        };
        stdout.execute(SetForegroundColor(Color::Blue));
        print!("|{}|", role);
        stdout.execute(SetForegroundColor(color));
        print!("{}", prefix);
        stdout.execute(ResetColor).unwrap();
        println!("{}", message);
    }
}

// Get user CLI response for question
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    stdout.execute(ResetColor).unwrap();

    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    // Trim whitespace and return
    return user_response.trim().to_string();
}

// Get user response that code is safe to execute
pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        // Print question in specified color
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        print!("WARNING: You are about to run code written entirely by AI.");
        println!("Review your code and confirm you wish to continue.");

        // Reset Color
        stdout.execute(ResetColor).unwrap();

        // Present options with diff. colors
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All good. Let's go!");
        stdout.execute(SetForegroundColor(Color::Red)).unwrap();
        println!("[2] Stop this madness!");

        let mut response: String = String::new();
        stdin()
            .read_line(&mut response)
            .expect("Failed to read human input");

        let response = response.trim().to_lowercase();

        match response.as_str() {
            "1" | "ok" | "y" | "yes" => {
                return true;
            }
            "2" | "n" | "no" => {
                return false;
            }
            _ => {
                println!("Invalid input.");
                return false;
            }
        }
    }
}
