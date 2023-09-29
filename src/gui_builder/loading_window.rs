/*
contains a function to open a window that lets the user see the current state of the conversion
*/

use druid::{WindowId, EventCtx, Env, WindowConfig, UnitPoint, WidgetExt, Widget, Event, Target, Size};
use druid::commands::CLOSE_WINDOW;
use druid::widget::{Label, LineBreaking, Controller, Flex, Either, ProgressBar, Button};
use super::AppState::AppState;
use crate::NEW_LOADING_WINDOW;

// An empty Widget. Just nothing.
// Used in an Either Widget to only show another Widget if a condition is true.
struct EmptyWidget;

impl<T> Widget<T> for EmptyWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(&mut self, _ctx: &mut druid::LayoutCtx, _bc: &druid::BoxConstraints, _data: &T, _env: &Env) -> druid::Size {
        Size::new(0.0, 0.0)
    }

    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &T, _env: &Env) {}
}

//controls when to close the sub window
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
        data.error_msg.to_string()
    })
        .with_line_break_mode(LineBreaking::WordWrap)
        .center();

    let calc_label = Label::new(|data: &AppState, _env: &Env| {
        data.calculating_msg.to_string()
    })
        .with_line_break_mode(LineBreaking::WordWrap)
        .center();

    let close_button = Button::new("close")
        .on_click(|ctx: &mut EventCtx, _data: &mut AppState, _env: &Env| {
            ctx.submit_command(CLOSE_WINDOW)
        });
    let end_button = Either::new(
        |data: &AppState, _env: &Env| {
            if data.calculating >= 1.0 {
                return true
            }
            false
        }, 
        close_button,
        EmptyWidget,
    ).align_horizontal(UnitPoint::CENTER)
    .padding((0.0, 25.0, 0.0, 0.0));

    let calc_container = Flex::column()
        .with_flex_child(calc_label, 1.0)
        .with_flex_child(
            ProgressBar::new().lens(AppState::calculating)
                .align_horizontal(UnitPoint::CENTER)
                .expand_width(),
            1.0
        ).with_flex_child(end_button, 1.0);

    let root_widget = Either::new(|data: &AppState, _env: &Env| {
        if !data.error_msg.is_empty() { //show error msg if an error occurs
            return true
        }
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