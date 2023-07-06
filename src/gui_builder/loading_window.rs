/*
contains a function to open a window that lets the user see the current state of the conversion
*/

use druid::{WindowId, EventCtx, Env, WindowConfig, UnitPoint, WidgetExt, Widget, Event, Target, TimerToken};
use druid::widget::{ Label, LineBreaking, Controller, Flex, Either, ProgressBar};
use super::AppState::AppState;
use crate::{ERROR, NEW_LOADING_WINDOW};

//controlls when to close the sub window
struct LoadingController {}

impl LoadingController {
    fn new() -> LoadingController {
        LoadingController {}
    }
}

impl<W: Widget<AppState>> Controller<AppState,W> for LoadingController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &druid::Event, data: &mut AppState, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(NEW_LOADING_WINDOW) => { //close current window if new window is opened
                ctx.window().close()
            },
            _ => (),
        }
        child.event(ctx, event, data, env)
    }

    fn lifecycle(
            &mut self,
            child: &mut W,
            ctx: &mut druid::LifeCycleCtx,
            event: &druid::LifeCycle,
            data: &AppState,
            env: &Env,
        ) {
        child.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, child: &mut W, ctx: &mut druid::UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        println!("sub window received update");
        child.update(ctx, old_data, data, env);
    }
}

pub fn open_loading(ctx: &mut EventCtx, data: &mut AppState, env: &Env) -> WindowId {
    let error_label = Label::new(|data: &AppState, _env: &Env| { //shows error messages
        format!("{}", data.error_msg)
    })
        .with_line_break_mode(LineBreaking::WordWrap)
        .align_vertical(UnitPoint::CENTER)
        .align_horizontal(UnitPoint::CENTER);

    let calc_label = Label::new(|data: &AppState, _env: &Env| {
        format!("{}", data.calculating_msg)
    });
    let calc_container = Flex::column()
        .with_flex_child(calc_label, 1.0)
        .with_flex_child(ProgressBar::new().lens(AppState::calculating), 1.0);

    let root_widget = Either::new(|data: &AppState, _env: &Env| {
        /*
        if !data.error_msg.is_empty() { //show error msg if an error occurs
            return true
        }
        */
        false //else show current conversion status
    }, error_label, calc_container).controller(LoadingController::new());
    
    let size = (300., 200.);
    
    ctx.submit_command(NEW_LOADING_WINDOW.to(Target::Global));
    ctx.new_sub_window( //open sub window
        WindowConfig::default()
            .resizable(false)
            .window_size(size),
        root_widget,
        data.clone(),
        env.clone(),
    )
}