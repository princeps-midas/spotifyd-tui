use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    prelude::*,
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Gauge, Paragraph, Widget},
};
// use ratatui_image::{Image, StatefulImage};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("spotifyd-tui")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .fg(Color::Magenta);
        // .bg(Color::Black);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(80),
                Constraint::Min(5),
                Constraint::Length(1),
            ])
            .split(block.inner(area));

        let text = format!(
            "\n\
                title: {}\n\
                artist: {}\n\
                {}: {}",
            self.title, self.artist, self.speaker, self.volume
        );

        let paragraph = Paragraph::new(text)
            // .block(block)
            .fg(Color::Magenta)
            // .bg(Color::Black)
            .centered();

        let progress = Gauge::default()
            .gauge_style(Style::new().magenta())
            .use_unicode(true)
            .label("")
            .ratio(self.progress);

        block.render(area, buf);
        // match &self.cover {
        //     None => (),
        //     Some(cover) => StatefulImage::new().render(layout[0], buf, &mut cover),
        // }
        paragraph.render(layout[1], buf);
        progress.render(layout[2], buf);
    }
}
