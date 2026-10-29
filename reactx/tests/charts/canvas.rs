use crate::harness::{any_cell, Harness};
use reactx::prelude::*;

fn canvas_app() -> Element {
    jsx! {
        <div style="direction:column;width:50">
            <Canvas
                x_bounds={(-180.0, 180.0)}
                y_bounds={(-90.0, 90.0)}
                shapes={vec![
                    CanvasShape::WorldMap { color: Some("#585b70".to_string()) },
                    CanvasShape::Circle { x: 77.2, y: 28.6, radius: 5.0, color: Some("#f38ba8".to_string()) },
                    CanvasShape::Print { x: 60.0, y: 20.0, text: "Delhi".to_string(), color: Some("#f38ba8".to_string()) },
                ]}
                title="Nodes"
            />
        </div>
    }
}

#[test]
fn canvas_draws_world_map_shapes_and_text() {
    let harness = Harness::new(canvas_app);
    let screen = harness.screen();
    assert!(screen.contains("Delhi"), "{screen}");
    assert!(
        any_cell(&harness, |cell| cell.symbol() == "⠂"
            || ('\u{2800}'..='\u{28FF}')
                .contains(&cell.symbol().chars().next().unwrap_or(' '))),
        "expected canvas dots; screen:\n{screen}"
    );
}

fn canvas_shapes_app() -> Element {
    let shapes = vec![
        CanvasShape::Line {
            x1: -9.0,
            y1: -9.0,
            x2: 9.0,
            y2: 9.0,
            color: Some("#89b4fa".to_string()),
        },
        CanvasShape::Rect {
            x: -9.0,
            y: -9.0,
            width: 6.0,
            height: 6.0,
            color: Some("#a6e3a1".to_string()),
        },
        CanvasShape::Circle {
            x: 5.0,
            y: 5.0,
            radius: 4.0,
            color: Some("#f38ba8".to_string()),
        },
        CanvasShape::Points {
            coords: vec![(-9.0, 9.0), (0.0, 0.0), (9.0, -9.0)],
            color: Some("#f9e2af".to_string()),
        },
        CanvasShape::Print {
            x: -9.0,
            y: -9.0,
            text: "hi".to_string(),
            color: None,
        },
    ];
    jsx! {
        <Canvas
            x_bounds={(-10.0, 10.0)}
            y_bounds={(-10.0, 10.0)}
            shapes={shapes}
            title="shapes"
            border="single"
        />
    }
}

#[test]
fn canvas_paints_every_shape_kind() {
    let harness = Harness::new(canvas_shapes_app);
    let screen = harness.screen();
    assert!(screen.contains("shapes"), "{screen}");
    assert!(screen.contains("hi"), "canvas text shape:\n{screen}");
    let canvas = harness.node_where(|node| node.attrs.variant.as_deref() == Some("canvas"));
    let rect = harness.renderer.layout[&canvas];
    let mut painted = 0;
    for y in rect.y + 1..rect.y + rect.height.saturating_sub(1) {
        for x in rect.x + 1..rect.x + rect.width.saturating_sub(1) {
            let symbol = harness.terminal.backend().buffer()[(x, y)].symbol();
            if symbol != " "
                && !"─│┌┐└┘╭╮╰╯├┤┬┴┼".contains(symbol)
                && !symbol.chars().all(|ch| ch.is_ascii_alphabetic())
            {
                painted += 1;
            }
        }
    }
    assert!(painted > 20, "canvas painted {painted} cells:\n{screen}");
}

fn canvas_marker_app(marker: &str) -> Element {
    let marker = marker.to_string();
    jsx! {
        <Canvas
            x_bounds={(-10.0, 10.0)}
            y_bounds={(-10.0, 10.0)}
            shapes={vec![
                CanvasShape::Line {
                    x1: -9.0,
                    y1: 0.0,
                    x2: 9.0,
                    y2: 0.0,
                    color: None,
                },
                CanvasShape::Circle {
                    x: 0.0,
                    y: 0.0,
                    radius: 5.0,
                    color: None,
                },
            ]}
            marker={marker}
            border="single"
        />
    }
}

#[test]
fn canvas_marker_prop_switches_the_glyph_grid() {
    let harness = Harness::new(|| canvas_marker_app("block"));
    let screen = harness.screen();
    assert!(
        any_cell(&harness, |cell| cell.symbol() == "\u{2588}"),
        "block marker should paint full cells:\n{screen}"
    );
    assert!(
        !any_cell(&harness, |cell| ('\u{2800}'..='\u{28FF}')
            .contains(&cell.symbol().chars().next().unwrap_or(' '))),
        "block marker must not paint braille:\n{screen}"
    );
}
