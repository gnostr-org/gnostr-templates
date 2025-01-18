use std::error;
use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;
use std::process::{Command, Stdio};
use tokio::process::Command as TokioCommand;
use tokio::sync::mpsc;
use tokio::task;


/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MainTab {
    System,
    Network,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SubTab {
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
    pub main_tabs: Vec<MainTab>,
    pub main_active_tab: usize,
    pub sub_tabs: Vec<SubTab>,
    pub sub_active_tab: usize,
    pub system_ls_output: Option<String>,
    pub system_top_output: Option<String>,
    pub network_ping_output: Option<String>,
    pub network_netstat_output: Option<String>,
    pub rx_system_ls: Option<mpsc::Receiver<String>>,
    pub rx_system_top: Option<mpsc::Receiver<String>>,
    pub rx_network_ping: Option<mpsc::Receiver<String>>,
    pub rx_network_netstat: Option<mpsc::Receiver<String>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            //main_tabs: Vec::<MainTab>,
			main_tabs: vec![MainTab::System, MainTab::Network],
            main_active_tab: 0,
            sub_tabs: vec![SubTab::Ls, SubTab::Top],
            sub_active_tab: 0,
            system_ls_output: Option::None,
            system_top_output: Option::None,
            network_ping_output: Option::None,
            network_netstat_output: Option::None,
            rx_system_ls: Option::None,
            rx_system_top: Option::None,
            rx_network_ping: Option::None,
            rx_network_netstat: Option::None,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

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


    fn next_main_tab(&mut self) {
        self.main_active_tab = (self.main_active_tab + 1) % self.main_tabs.len();
    }

    fn prev_main_tab(&mut self) {
        if self.main_active_tab == 0 {
            self.main_active_tab = self.main_tabs.len() - 1;
        } else {
            self.main_active_tab -= 1;
        }
    }

    fn next_sub_tab(&mut self) {
        self.sub_active_tab = (self.sub_active_tab + 1) % self.sub_tabs.len();
    }

    fn prev_sub_tab(&mut self) {
        if self.sub_active_tab == 0 {
            self.sub_active_tab = self.sub_tabs.len() - 1;
        } else {
            self.sub_active_tab -= 1;
        }
    }

}
