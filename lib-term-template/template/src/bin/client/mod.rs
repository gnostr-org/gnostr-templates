/// Much of this example borrows from the `tui-rs` examples, and was modified for our purposes.
/// See: https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs
extern crate chrono;
use chrono::prelude::*;
const TIME_FORMAT: &'static str = "%H:%M:%S";

extern crate lib_term;
use lib_term::*;

use crate::messages;
use messages::*;

extern crate tui;

use tui::backend::MouseBackend;
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, Widget};
use tui::Terminal;

pub struct App {
    user_name: String,
    size: Rect,
    input: String,
    input_mode: Mode,
    messages: Vec<(DateTime<Local>, String, String)>,
    commands: Vec<(DateTime<Local>, String, String, String)>,
    terminal: Terminal<MouseBackend>,
}

impl App {
    pub fn new(user_name: String) -> App {
        App {
            user_name,
            size: Rect::default(),
            input: String::new(),
            input_mode: Mode::CHAT,
            messages: Vec::new(),
            commands: Vec::new(),
            terminal: Terminal::new(MouseBackend::new().unwrap()).unwrap(),
        }
    }
}

impl lib_term::client::ShellClient<Message, Response> for App {
    fn server_url(&self) -> String {
        "127.0.0.1:8080".to_owned()
    }

    fn on_key(&mut self, key: lib_term::client::Key) -> lib_term::client::KeyAction<Message> {
        match key {
            lib_term::client::Key::Ctrl('c') | lib_term::client::Key::Esc => {
                return lib_term::client::KeyAction::Exit;
            }
            lib_term::client::Key::Char('\n') => {
                let message = self.input.drain(..).collect::<String>();
                match message.as_ref() {
                    "CHAT" => {
                        self.input_mode = Mode::CHAT;
                    }
                    "CMD" => {
                        self.input_mode = Mode::CMD;
                    }
                    "CLEAR" => {
                        match self.input_mode {
                            Mode::CMD => self.commands.clear(),
                            Mode::CHAT => self.messages.clear(),
                        };
                    }
                    _ => {
                        return lib_term::client::KeyAction::SendMessage(Message {
                            content: message,
                            mode: self.input_mode.clone(),
                            user_name: self.user_name.clone(),
                        });
                    }
                }
            }
            lib_term::client::Key::Char(c) => {
                self.input.push(c);
            }
            lib_term::client::Key::Backspace => {
                self.input.pop();
            }
            _ => {}
        }

        lib_term::client::KeyAction::DoNothing
    }

    fn receive_response(&mut self, response: Response) {
        match response.og_msg.mode {
            Mode::CHAT => {
                self.messages
                    .push((Local::now(), response.og_msg.user_name, response.response));
            }
            Mode::CMD => {
                self.commands.push((
                    Local::now(),
                    response.og_msg.user_name,
                    response.og_msg.content,
                    response.response,
                ));
            }
        };
    }

    fn first_draw(&mut self) {
        self.terminal.clear().unwrap();
        self.terminal.hide_cursor().unwrap();
        self.size = self.terminal.size().unwrap();

        self.draw();
    }

    fn last_draw(&mut self) {
        self.terminal.show_cursor().unwrap();
    }

    fn draw(&mut self) {
        let mut size = self.terminal.size().unwrap();
        self.terminal.resize(size).unwrap();

        size = self.terminal.size().unwrap();
        let mode = &self.input_mode;
        let input = &self.input;
        let messages = &self.messages;
        let commands = &self.commands;

        Group::default()
            .direction(Direction::Vertical)
            //above
            .margin(1) //add tui drop down menu
            //       height           width  
            .sizes(&[Size::Fixed(3), Size::Min(1)])
            .render(&mut self.terminal, &size, |t, chunks| {
                Paragraph::default()
                    .style(Style::default().fg(Color::Yellow))
                    //.block(Block::default().borders(Borders::NONE).title(match *mode {
                    //    Mode::CHAT => "CHAT",
                    //    Mode::CMD => "CMD",
                    //}))
                    .block(Block::default().borders(Borders::NONE).title(""))
                    .text(&format!("{:?}> {}",&mode, input))
                    .render(t, &chunks[0]);

                Group::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .sizes(&[Size::Percent(50), Size::Percent(50)])
                    .render(t, &chunks[1], |t, chunks| {
                        // Use Paragraphs so we can get text wrapping
                        let messages: String =
                            messages
                                .iter()
                                .rev()
                                .fold("".to_owned(), |mut acc, (t, u, m)| {
                                    acc.push_str(&format!(
                                        "{}: {}: {}\n",
                                        t.format(TIME_FORMAT).to_string(),
                                        u,
                                        m
                                    ));
                                    acc
                                });
                        Paragraph::default()
                            .block(Block::default().borders(Borders::ALL).title("Messages"))
                            .wrap(true)
                            .text(&messages)
                            .render(t, &chunks[0]);

                        let commands: String =
                            commands
                                .iter()
                                .rev()
                                .fold("".to_owned(), |mut acc, (t, u, c, m)| {
                                    acc.push_str(&format!(
                                        "{}: {} >> {}\n{}{}\n",
                                        t.format(TIME_FORMAT).to_string(),
                                        u,
                                        c,
                                        m,
                                        if m.ends_with("\n") { "" } else { "\n" }
                                    ));
                                    acc
                                });
                        Paragraph::default()
                            .block(Block::default().borders(Borders::ALL).title("Commands"))
                            .wrap(true)
                            .text(&commands)
                            .render(t, &chunks[1]);
                    });
            });

        self.terminal.draw().unwrap();
    }
}
