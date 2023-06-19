/*
contains a function to open a sub-window to show error messages
*/

use druid::{EventCtx, Env, WidgetExt, UnitPoint, WindowConfig, Data};
use druid::widget::{Label, LabelText, LineBreaking};

/// opens a sub window displaying a given error message
pub fn open_error<T: Data>(ctx: &mut EventCtx, data: &T, env: &Env, msg: impl Into<LabelText<T>>) {
    let error_window = Label::new(msg)
        .with_line_break_mode(LineBreaking::WordWrap)
        .align_vertical(UnitPoint::CENTER)
        .align_horizontal(UnitPoint::CENTER);

    let size = (250.0, 180.0);

    ctx.new_sub_window( //open window
        WindowConfig::default()
            .resizable(false)
            .window_size(size),
        error_window,
        data.clone(),
        env.clone(),
    );
}