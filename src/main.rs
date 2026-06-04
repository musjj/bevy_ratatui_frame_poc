use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_ratatui::{RatatuiContext, RatatuiPlugins};
use ratatui::prelude::{Rect, *};

fn main() {
    let frame_time = std::time::Duration::from_secs_f32(1. / 60.);

    App::new()
        .init_resource::<BevyFrame>()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_time)),
            RatatuiPlugins::default(),
        ))
        .add_systems(PreUpdate, adjust_frame)
        .add_systems(Update, (draw_left, draw_right))
        .add_systems(PostUpdate, draw_frame)
        .run();
}

fn draw_left(mut frame: ResMut<BevyFrame>) {
    let [left, _] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(frame.area);

    frame.render_widget("hello", left);
}

fn draw_right(mut frame: ResMut<BevyFrame>) {
    let [_, right] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(frame.area);

    frame.render_widget("world", right);
}

#[derive(Resource, Debug, Default, Hash)]
pub struct BevyFrame {
    cursor_position: Option<Position>,
    area: Rect,
    buffer: Buffer,
    count: usize,
}

impl BevyFrame {
    pub fn render_widget<W: Widget>(&mut self, widget: W, area: Rect) {
        widget.render(area, &mut self.buffer);
    }
}

fn adjust_frame(mut frame: ResMut<BevyFrame>, mut context: ResMut<RatatuiContext>) -> Result {
    context.autoresize()?;

    let context_frame = context.get_frame();

    frame.buffer.resize(context_frame.area());
    frame.buffer.reset();

    frame.cursor_position = None;
    frame.area = context_frame.area();
    frame.count = context_frame.count();

    Ok(())
}

fn draw_frame(frame: ResMut<BevyFrame>, mut context: ResMut<RatatuiContext>) -> Result {
    context.current_buffer_mut().merge(&frame.buffer);
    context.apply_buffer_with_cursor(frame.cursor_position)?;
    Ok(())
}
