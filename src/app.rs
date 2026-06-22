use crate::{
    event::{AppEvent, Event, EventHandler},
    main,
};
use std::cmp;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use image::ImageReader;
use ratatui::{DefaultTerminal, layout::Rect};
// use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use rsbash::rash;

/// Application.
// #[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    // song title
    pub title: String,
    // song artist
    pub artist: String,
    // current volume
    pub volume: u8,
    // whether audio is currently muted
    pub mute: bool,
    // ui speaker symbol
    pub speaker: String,
    // progress of the current song
    pub progress: f64,
    // cover art of the song
    // pub cover: StatefulProtocol,
    //area the cover may take up
    pub cover_area: Rect,
    // whether the song changed since last tick
    pub new_song: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            title: String::new(),
            artist: String::new(),
            volume: 0,
            mute: false,
            speaker: String::new(),
            progress: 0.,
            // cover: None,
            cover_area: Rect::new(0, 0, 0, 0),
            new_song: true,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event)
                    if key_event.kind == crossterm::event::KeyEventKind::Press =>
                {
                    self.handle_key_event(key_event)?
                }
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
                AppEvent::Reload => self.reload(),
                AppEvent::PlayPause => self.playpause(),
                AppEvent::Previous => self.previous(),
                AppEvent::Next => self.next(),
                AppEvent::VolUp => self.vol_up(),
                AppEvent::VolDown => self.vol_down(),
                AppEvent::ToggleMute => self.toggle_mute(),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Char('r' | 'R') => self.events.send(AppEvent::Reload),
            KeyCode::Char(' ') => self.events.send(AppEvent::PlayPause),
            KeyCode::Left => self.events.send(AppEvent::Previous),
            KeyCode::Right => self.events.send(AppEvent::Next),
            KeyCode::Up => self.events.send(AppEvent::VolUp),
            KeyCode::Down => self.events.send(AppEvent::VolDown),
            KeyCode::Char('m' | 'M') => self.events.send(AppEvent::ToggleMute),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) {
        self.get_title();
        self.get_artist();
        self.get_volume();
        self.get_speaker();
        self.get_progress();
        // if self.new_song {
        // self.get_cover();
        // }
    }

    // Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    // there might be a better way to do this
    pub fn reload(&mut self) {
        let _ = rash!("killall spotifyd && hyprctl dispatch spotifyd");
    }

    pub fn get_title(&mut self) {
        self.new_song = false;
        let (retval, mut stdout, _) =
            rash!("playerctl --player=spotifyd metadata --format '{{title}}'").unwrap();
        if retval == 0 {
            stdout.pop();
            if self.title != stdout {
                self.title = stdout;
                self.new_song = true;
            }
        } else {
            self.title = "none".to_string();
        }
    }

    pub fn get_artist(&mut self) {
        let (retval, mut stdout, _) =
            rash!("playerctl --player=spotifyd metadata --format '{{artist}}'").unwrap();
        if retval == 0 {
            stdout.pop();
            self.artist = stdout;
        } else {
            self.artist = "none".to_string();
        }
    }

    pub fn get_volume(&mut self) {
        if self.mute == true {
            return;
        }
        let (retval, mut stdout, _) =
            rash!("playerctl --player=spotifyd volume --format {{volume}}").unwrap();
        if retval == 0 {
            stdout.pop();
            self.volume = (stdout.to_string().parse::<f32>().unwrap() * 100 as f32).round() as u8;
        } else {
            self.volume = 0;
        }
    }

    pub fn get_speaker(&mut self) {
        if self.mute {
            self.speaker = "󰖁".to_string();
        } else if self.volume >= 70 {
            self.speaker = "".to_string();
        } else if self.volume >= 30 {
            self.speaker = "".to_string();
        } else {
            self.speaker = "".to_string();
        }
    }

    pub fn get_progress(&mut self) {
        let (_, mut length, _) =
            rash!("playerctl --player=spotifyd metadata mpris:length").unwrap();
        let (retval, mut position, _) = rash!("playerctl --player=spotifyd position").unwrap();
        if retval == 0 {
            length.pop();
            position.pop();
            self.progress = (position.parse::<f64>().unwrap() * 1000000 as f64)
                / (length.parse::<f64>().unwrap());
            self.progress = cmp::min(self.progress, 1);
            // println!("{}", self.progress)
        } else {
            self.progress = 0.;
        }
    }

    pub fn get_cover(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // let (retval, mut stdout, _) =
        //     rash!("playerctl --player=spotifyd metadata mpris:url").unwrap();

        // let picker = Picker::from_query_stdio()?;
        // let font_size = picker.font_size();

        // let img = ImageReader::open("./assets/spotifyd.png")?.decode()?;

        // let rect = Rect::new(
        //     0,
        //     0,
        //     img.width().div_ceil(font_size.0 as u32) as u16,
        //     img.height().div_ceil(font_size.1 as u32) as u16,
        // );

        // Create the Protocol once, or in other words, transform the image data to Sixels, Kitty
        // data, iTerm2 base64 PNG data, or some kind of ASCII-art.
        // let image = picker.new_resize_protocol(img);

        // self.cover = image;

        Ok(())
    }

    pub fn playpause(&mut self) {
        let _ = rash!("playerctl --player=spotifyd play-pause");
    }

    pub fn previous(&mut self) {
        let _ = rash!("playerctl --player=spotifyd previous");
    }

    pub fn next(&mut self) {
        let _ = rash!("playerctl --player=spotifyd next");
    }

    pub fn vol_up(&mut self) {
        let _ = rash!("playerctl --player=spotifyd volume 0.05+");
    }

    pub fn vol_down(&mut self) {
        let _ = rash!("playerctl --player=spotifyd volume 0.05-");
    }

    pub fn toggle_mute(&mut self) {
        if self.mute == false {
            self.mute = true;
            let _ = rash!("playerctl --player=spotifyd volume 0");
        } else {
            self.mute = false;
            let command = format!("playerctl --player=spotifyd volume 0.{}", self.volume);
            let _ = rash!(command);
        }
    }
}
