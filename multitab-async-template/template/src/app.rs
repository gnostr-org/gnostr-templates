use std::error;
use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;
use ratatui::{
    style::{Style, Stylize},
    symbols,
    widgets::{Block, Tabs},
};
use std::process::{Command, Stdio};
use tokio::process::Command as TokioCommand;
use tokio::sync::mpsc;
use tokio::task;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Clone, Copy, Debug, PartialEq)]
enum MainTab {
    System,
    Network,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SubTab {
    Ls,
    Top,
    Ping,
    Netstat,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    main_tabs: Vec<MainTab>,
    main_active_tab: usize,
    sub_tabs: Vec<SubTab>,
    sub_active_tab: usize,
    system_ls_output: Option<String>,
    system_top_output: Option<String>,
    network_ping_output: Option<String>,
    network_netstat_output: Option<String>,
    rx_system_ls: Option<mpsc::Receiver<String>>,
    rx_system_top: Option<mpsc::Receiver<String>>,
    rx_network_ping: Option<mpsc::Receiver<String>>,
    rx_network_netstat: Option<mpsc::Receiver<String>>,
}
impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            main_tabs: Vec::<MainTab>,
            main_active_tab: 0,
            sub_tabs: Vec::<SubTab>,
            sub_active_tab: 0,
            system_ls_output: std::option::Option::None,
            system_top_output: std::option::Option::None,
            network_ping_output: std::option::Option::None,
            network_netstat_output: std::option::Option::None,
            rx_system_ls: Option::<mpsc::Receiver::<String>>,
            rx_system_top: Option::<mpsc::Receiver::<String>>,
            rx_network_ping: Option::<mpsc::Receiver::<String>>,
            rx_network_netstat: Option::<mpsc::Receiver::<String>>,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn default() -> Self {
        Self::default()
    }
    pub fn new() -> Self {
        Self {
            running: bool,
            counter: i32,
            main_tabs: vec![MainTab::System, MainTab::Network],
            main_active_tab: 0,
            sub_tabs: vec![SubTab::Ls, SubTab::Top],
            sub_active_tab: 0,
            system_ls_output: None,
            system_top_output: None,
            network_ping_output: None,
            network_netstat_output: None,
            rx_system_ls: None,
            rx_system_top: None,
            rx_network_ping: None,
            rx_network_netstat: None,
        }
    }

    pub fn next_main_tab(&mut self) {
        self.main_active_tab = (self.main_active_tab + 1) % self.main_tabs.len();
    }

    pub fn prev_main_tab(&mut self) {
        if self.main_active_tab == 0 {
            self.main_active_tab = self.main_tabs.len() - 1;
        } else {
            self.main_active_tab -= 1;
        }
    }

    pub fn next_sub_tab(&mut self) {
        self.sub_active_tab = (self.sub_active_tab + 1) % self.sub_tabs.len();
    }

    pub fn prev_sub_tab(&mut self) {
        if self.sub_active_tab == 0 {
            self.sub_active_tab = self.sub_tabs.len() - 1;
        } else {
            self.sub_active_tab -= 1;
        }
    }
    ///// Handles the tick event of the terminal.
    pub fn tick(&mut self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }




}

impl StatefulWidget for App {
    type State = ();

//note: `render` from trait: `fn(Self, ratatui::layout::Rect, &mut ratatui::buffer::Buffer, &mut <Self as ratatui::prelude::StatefulWidget>::State)`


    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &State) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20), Constraint::Min(0)])
            .split(area);

        // Render main tabs
        let main_tab_titles = self
            .main_tabs
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let style = if i == self.main_active_tab {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                };
                Span::styled(format!("{:?}", t), style)
            })
            .collect::<Vec<_>>();

        let main_tab_bar = TabBar::new(main_tab_titles)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::DarkGray))
            .select(self.main_active_tab);

        buf.render_widget(main_tab_bar, chunks[0]);

        // Render sub tabs
        let sub_tab_titles = self
            .sub_tabs
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let style = if i == self.sub_active_tab {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                };
                Span::styled(format!("{:?}", t), style)
            })
            .collect::<Vec<_>>();

        let sub_tab_bar = TabBar::new(sub_tab_titles)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::DarkGray))
            .select(self.sub_active_tab);

        buf.render_widget(sub_tab_bar, chunks[1]);

        // Render output based on active tabs
        let output_block = match self.main_tabs[self.main_active_tab] {
            MainTab::System => match self.sub_tabs[self.sub_active_tab] {
                SubTab::Ls => {
                    // ... (Handle system_ls_output and receive updates)
                }
                SubTab::Top => {
                    // ... (Handle system_top_output and receive updates)
                }
            },
            MainTab::Network => match self.sub_tabs[self.sub_active_tab] {
                SubTab::Ping => {
                    // ... (Handle network_ping_output and receive updates)
                }
                SubTab::Netstat => {
                    // ... (Handle network_netstat_output and receive updates)
                }
            },
        };

        buf.render_widget(output_block, chunks[1]);
    }
}
