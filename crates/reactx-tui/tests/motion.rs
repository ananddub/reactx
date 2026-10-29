//! Spring physics, colour tweening and custom node shaders.

use std::thread::sleep;
use std::time::{Duration, Instant};

use dioxus::prelude::*;
use reactx_tui::effects::{fx, register_shader, Color, Interpolation};
use reactx_tui::{use_color_tween, use_spring, Spring};

mod common;

use common::{click, fg, screen, setup};

fn spring_app() -> Element {
    let mut target = use_signal(|| 0.0f64);
    let value = use_spring(target(), Spring::bouncy());
    rsx! {
        div { style: "width:16;direction:column",
            span { "{value:.0}" }
            button { onclick: move |_| target.set(50.0), "go" }
        }
    }
}

#[tokio::test]
async fn spring_hook_settles_on_the_target() {
    let (mut dom, mut renderer, mut terminal) = setup(spring_app);
    assert!(screen(&terminal).contains('0'), "{}", screen(&terminal));

    click(&mut dom, &mut renderer, &mut terminal);
    let deadline = Instant::now() + Duration::from_secs(3);
    while Instant::now() < deadline {
        tokio::select! {
            _ = dom.wait_for_work() => {
                dom.render_immediate(&mut renderer);
                renderer.paint(&mut terminal).unwrap();
            }
            _ = tokio::time::sleep(Duration::from_millis(20)) => {}
        }
        if screen(&terminal).contains("50") {
            return;
        }
    }
    panic!("spring never settled; screen:\n{}", screen(&terminal));
}

fn css(color: Color) -> String {
    match color {
        Color::Red => "red".into(),
        Color::Blue => "blue".into(),
        Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
        _ => "reset".into(),
    }
}

fn color_app() -> Element {
    let mut second = use_signal(|| false);
    let tint = if second() { Color::Blue } else { Color::Red };
    let tint = use_color_tween(tint, 100u32);
    rsx! {
        div { style: "width:10",
            span { style: format!("color:{}", css(tint)), "tint" }
        }
        button { onclick: move |_| second.set(true), "swap" }
    }
}

#[tokio::test]
async fn color_hook_blends_to_the_target_color() {
    let (mut dom, mut renderer, mut terminal) = setup(color_app);
    assert_eq!(fg(&terminal, 0, 0), Color::Red);

    click(&mut dom, &mut renderer, &mut terminal);
    let deadline = Instant::now() + Duration::from_secs(3);
    while Instant::now() < deadline {
        tokio::select! {
            _ = dom.wait_for_work() => {
                dom.render_immediate(&mut renderer);
                renderer.paint(&mut terminal).unwrap();
            }
            _ = tokio::time::sleep(Duration::from_millis(20)) => {}
        }
        if fg(&terminal, 0, 0) == Color::Blue {
            return;
        }
    }
    panic!("colour never reached the target: {:?}", fg(&terminal, 0, 0));
}

fn custom_app() -> Element {
    rsx! {
        div {
            style: "width:12;height:1;animate:test-sparkle;animate-duration:80",
            span { "spark" }
        }
    }
}

#[test]
fn custom_shader_runs_from_animate_attribute() {
    register_shader("test-sparkle", |_| {
        fx::fade_from_fg(Color::Magenta, (80u32, Interpolation::Linear))
    });
    let (_dom, mut renderer, mut terminal) = setup(custom_app);
    assert!(renderer.effects_active(), "custom shader did not start");
    let outside = fg(&terminal, 25, 4);

    sleep(Duration::from_millis(30));
    renderer.paint(&mut terminal).unwrap();
    assert_ne!(
        fg(&terminal, 0, 0),
        outside,
        "custom shader did not tint the node"
    );

    let deadline = Instant::now() + Duration::from_secs(3);
    while renderer.effects_active() && Instant::now() < deadline {
        sleep(Duration::from_millis(40));
        renderer.paint(&mut terminal).unwrap();
    }
    renderer.paint(&mut terminal).unwrap();
    assert!(!renderer.effects_active(), "custom shader never finished");
    assert_eq!(fg(&terminal, 0, 0), outside, "cell was not restored");
}
