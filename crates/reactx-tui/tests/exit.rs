//! Exit animations and composed node effects.

use std::thread::sleep;
use std::time::{Duration, Instant};

use dioxus::prelude::*;

mod common;

use common::{click, fg, screen, setup};

fn exit_app() -> Element {
    let mut shown = use_signal(|| true);
    rsx! {
        div { style: "direction:column",
            if shown() {
                div {
                    style: "width:14;height:1;animate-exit:fade-out;animate-duration:80;animate-color:black",
                    span { "leaving" }
                }
            }
            button { style: "width:10", onclick: move |_| shown.set(false), "hide" }
        }
    }
}

#[test]
fn exit_animation_replays_the_node_then_clears_it() {
    let (mut dom, mut renderer, mut terminal) = setup(exit_app);
    assert!(screen(&terminal).contains("leaving"));
    assert!(
        !renderer.effects_active(),
        "exit animation ran while the node was mounted"
    );

    click(&mut dom, &mut renderer, &mut terminal);
    assert!(renderer.effects_active(), "exit animation did not start");
    assert!(
        screen(&terminal).contains("leaving"),
        "ghost did not replay the removed node:\n{}",
        screen(&terminal)
    );
    assert!(
        !screen(&terminal).contains("hide"),
        "ghost did not cover the freed cell:\n{}",
        screen(&terminal)
    );

    sleep(Duration::from_millis(150));
    renderer.paint(&mut terminal).unwrap();
    renderer.paint(&mut terminal).unwrap();
    assert!(!renderer.effects_active(), "exit animation never finished");
    assert!(
        !screen(&terminal).contains("leaving"),
        "ghost cells were left on screen:\n{}",
        screen(&terminal)
    );
    assert!(
        screen(&terminal).contains("hide"),
        "layout was not repainted"
    );
}

fn composed_app() -> Element {
    rsx! {
        div {
            style: "width:10;height:1;\
                animate:fade, dissolve;animate-duration:60;animate-color:red;\
                animate-ping-pong:1;animate-repeat:2",
            span { "x" }
        }
    }
}

#[test]
fn sequence_ping_pong_and_repeat_finish() {
    let (_dom, mut renderer, mut terminal) = setup(composed_app);
    assert!(renderer.effects_active(), "composed effect did not start");

    let deadline = Instant::now() + Duration::from_secs(3);
    while renderer.effects_active() && Instant::now() < deadline {
        sleep(Duration::from_millis(40));
        renderer.paint(&mut terminal).unwrap();
    }
    renderer.paint(&mut terminal).unwrap();
    assert!(!renderer.effects_active(), "composed effect never finished");
}

fn stagger_app() -> Element {
    rsx! {
        div {
            style: "width:12;height:1;animate:fade;animate-duration:60;animate-color:red",
            span { "a" }
        }
        div {
            style: "width:12;height:1;animate:fade;animate-duration:60;animate-color:red;\
                animate-index:4;animate-stagger:100",
            span { "b" }
        }
    }
}

#[test]
fn stagger_delays_later_siblings() {
    let (_dom, mut renderer, mut terminal) = setup(stagger_app);
    let outside = fg(&terminal, 25, 5);
    assert!(renderer.effects_active());

    sleep(Duration::from_millis(150));
    renderer.paint(&mut terminal).unwrap();
    renderer.paint(&mut terminal).unwrap();
    assert_eq!(
        fg(&terminal, 0, 0),
        outside,
        "first sibling kept animating past its duration"
    );
    assert!(
        renderer.effects_active(),
        "staggered sibling finished before its delay"
    );

    let deadline = Instant::now() + Duration::from_secs(3);
    while renderer.effects_active() && Instant::now() < deadline {
        sleep(Duration::from_millis(40));
        renderer.paint(&mut terminal).unwrap();
    }
    renderer.paint(&mut terminal).unwrap();
    assert!(
        !renderer.effects_active(),
        "staggered sibling never finished"
    );
    assert_eq!(fg(&terminal, 0, 0), outside);
}
