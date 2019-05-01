use crate::util::*;
use crate::console::Console;
use crate::command_handler::*;

pub struct App {
    pub mode: Mode,
    pub processes_sort_by: SortBy,
    pub processes_sort_direction: SortDirection,
    pub size: tui::layout::Rect,
    pub console: Console
}

impl App {
    pub fn new() -> App {
        App {
            mode: Mode::Main,
            processes_sort_by: SortBy::CPU,
            processes_sort_direction: SortDirection::DESC,
            size: tui::layout::Rect::new(0, 0, 0, 0),
            console: Console::new()
        }
    }

    // Toggles the soring of processes between ascending and descending
    pub fn toggle_sort_direction(&mut self) {
        match self.processes_sort_direction {
            SortDirection::ASC => {
                self.processes_sort_direction = SortDirection::DESC;
            }
            SortDirection::DESC => {
                self.processes_sort_direction = SortDirection::ASC;
            }
        }
    }

    // Processes the current console buffer as a command
    pub fn process_command(&mut self) {
        let input = self.console.clear_input();

        match handle_cmd(nom::types::CompleteStr(&input)) {
            Ok((_, cmd)) => self.console.write(format!("{:?}", cmd)),
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                if let nom::Context::Code(_, nom::ErrorKind::Custom(cmd_err)) = e {
                    self.console.write(cmd_err.display());
                } else {
                    self.console.write(CmdError::ParseErr.display());
                }
            }
            _ => {}
        }
    }
}
