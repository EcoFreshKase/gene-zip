/*
Enthält eine Funktion um ein Sub-Window zu öffnen, welches eine Fehlermeldung enthält
*/

use druid::{EventCtx, Env, WidgetExt, UnitPoint, WindowConfig, Data};
use druid::widget::{Label, LabelText, LineBreaking};

pub fn open_error<T: Data>(ctx: &mut EventCtx, data: &T, env: &Env, msg: impl Into<LabelText<T>>) {
    /*
    Erstellt ein Fenster, welches eine gegebene Fehlermeldung anzeigt
    */
    let error_window = Label::new(msg)
        .with_line_break_mode(LineBreaking::WordWrap)
        .align_vertical(UnitPoint::CENTER)
        .align_horizontal(UnitPoint::CENTER);

    let size = (250.0, 180.0);

    ctx.new_sub_window( //Fenster öffnen
        WindowConfig::default()
            .resizable(false)
            .window_size(size),
        error_window,
        data.clone(),
        env.clone(),
    );
}