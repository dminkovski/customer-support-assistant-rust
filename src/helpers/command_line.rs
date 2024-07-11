use crossterm::style::{Color, SetForegroundColor};
use crossterm::{Command, ExecutableCommand};
use std::io::{stdout, Stdout};
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
            Self::Default => Color::White,
        };

        let prefix: &str = match self {
            Self::Error => "ERROR: ",
            Self::Warning => "WARNING: ",
            Self::Info => "INFO: ",
            Self::Default => "",
        };
        stdout.execute(SetForegroundColor(Color::Blue));
        print!("{}: ", role);
        stdout.execute(SetForegroundColor(color));
        print!("{}", prefix);
        stdout.execute(SetForegroundColor(Color::White));
        println!("{}", message);
    }
}
