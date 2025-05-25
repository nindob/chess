use ratatui::{
    style::{Color, Style, Modifier, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph, Padding},
    Frame, text::{Span, Line}, layout::{Layout, Direction, Constraint, Rect},
};


use crate::{app::App};

// renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {

    // splitting the full tui in 3 vertical boxes and 3 horizontal boxes in the vertical[1]
    let main_area = frame.size();
 
    let main_layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 18),
                Constraint::Ratio(16, 18),
                Constraint::Ratio(1, 18),
            ]
            .as_ref(),
        )
        .split(main_area);
    let main_layout_horizontal = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        [
            Constraint::Ratio(4, 15),
            Constraint::Ratio(7, 15),
            Constraint::Ratio(4, 15),
        ]
        .as_ref(),
    )
    .split(main_layout_vertical[1]);

    // board block representing the full board div
    let board_block =  Block::default()
    .style(Style::default()
    .bg(Color::Rgb((210), (200), (190))));


    
    // we render the board_block in the center layout made above
    frame.render_widget(board_block.clone(), main_layout_horizontal[1]);

    // we make the inside of the board
    board_render(board_block.inner(main_layout_horizontal[1]), frame);
}

fn board_render(area: Rect, frame: &mut Frame) {
    // we have 8 vertical lines
    let columns = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                ]
            .as_ref(),
        )
        .split(area);

    // for each line we set 8 layout
    for i in 0..8{
        let lines = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
    
            ]
            .as_ref(),
        ).split(columns[i]);
        for j in 0..8{
            let color: Color = if (i + j) % 2 == 0 {
                Color::Rgb((210), (200), (190))
            } else {
                Color::Black
            };
            let cell = Block::default().bg(color);
            frame.render_widget(cell,lines[j]);
        }
    }
    
}