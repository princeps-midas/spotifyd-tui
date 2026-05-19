use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};
// use ratatui_image::{StatefulImage, picker::Picker, protocol::StatefulProtocol};

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
            .border_type(BorderType::Rounded);

        // let image = StatefulImage::default();

        let text = format!(
            "\n\
                title: {}\n\
                artist: {}\n\
                {}: {}",
            self.title, self.artist, self.speaker, self.volume
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Magenta)
            .bg(Color::Black)
            .centered();

        paragraph.render(area, buf);
    }
}
