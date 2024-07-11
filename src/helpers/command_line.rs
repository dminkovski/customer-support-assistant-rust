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