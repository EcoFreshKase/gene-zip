/*
contains a function to open a window that lets the user see the current state of the conversion
*/

use druid::{WindowId, EventCtx, Env, WindowConfig, UnitPoint, WidgetExt, Widget, Event};
use druid::widget::{ Label, LineBreaking, Controller};
use super::AppState::AppState;
use crate::ERROR;

//controlls when to close the sub window
struct LoadingController;

impl<W: Widget<AppState>> Controller<AppState,W> for LoadingController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &druid::Event, data: &mut AppState, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(ERROR) => ctx.window().close(),
            _ => child.event(ctx, event, data, env)
        }
    }

    fn update(&mut self, child: &mut W, ctx: &mut druid::UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        child.update(ctx, old_data, data, env);
    }
}

pub fn open_loading(ctx: &mut EventCtx, data: &AppState, env: &Env) -> WindowId {
    let root_widget = Label::new(|data: &AppState, _env: &Env| {
        if data.calculating {
            return "Converting ..."
        }
        "Converting completed!"
    })
        .with_line_break_mode(LineBreaking::WordWrap)
        .align_vertical(UnitPoint::CENTER)
        .align_horizontal(UnitPoint::CENTER)
        .controller(LoadingController);
    
    let size = (300.0, 200.);
    
    ctx.new_sub_window( //open sub window
        WindowConfig::default()
            .resizable(false)
            .window_size(size),
        root_widget,
        data.clone(),
        env.clone(),
    )
}