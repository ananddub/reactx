//! Static data tables for the dashboard example.

use reactx::prelude::*;

pub(super) const PROCESSES: &[(&str, &str, &str, &str)] = &[
    ("1", "systemd", "0.1%", "12 MB"),
    ("104", "reactx-app", "2.4%", "48 MB"),
    ("231", "postgres", "8.7%", "512 MB"),
    ("418", "taffy-worker", "15.2%", "96 MB"),
];

pub(super) fn headers() -> Vec<String> {
    vec![
        "PID".to_string(),
        "Process".to_string(),
        "CPU".to_string(),
        "Memory".to_string(),
    ]
}

pub(super) fn rows() -> Vec<Vec<String>> {
    PROCESSES
        .iter()
        .map(|(pid, name, cpu, memory)| {
            vec![
                pid.to_string(),
                name.to_string(),
                cpu.to_string(),
                memory.to_string(),
            ]
        })
        .collect()
}

pub(super) fn datasets() -> (ChartDataset, ChartDataset) {
    let latency = ChartDataset {
        name: "Server A".into(),
        data: vec![
            (0.0, 15.0),
            (1.0, 22.0),
            (2.0, 18.0),
            (3.0, 35.0),
            (4.0, 28.0),
            (5.0, 44.0),
            (6.0, 31.0),
            (7.0, 38.0),
            (8.0, 26.0),
            (9.0, 41.0),
            (10.0, 33.0),
            (11.0, 29.0),
            (12.0, 36.0),
        ],
        color: Some("#a6e3a1".into()),
        marker: "braille".into(),
    };
    let errors = ChartDataset {
        name: "Server B".into(),
        data: vec![
            (0.0, 8.0),
            (2.0, 12.0),
            (4.0, 6.0),
            (6.0, 19.0),
            (8.0, 11.0),
            (10.0, 24.0),
            (12.0, 14.0),
        ],
        color: Some("#f38ba8".into()),
        marker: "braille".into(),
    };
    (latency, errors)
}

pub(super) fn shapes() -> Vec<CanvasShape> {
    vec![
        CanvasShape::Line {
            x1: 6.0,
            y1: 4.0,
            x2: 74.0,
            y2: 16.0,
            color: Some("#585b70".into()),
        },
        CanvasShape::Points {
            coords: vec![
                (6.0, 4.0),
                (20.0, 7.0),
                (38.0, 10.0),
                (56.0, 13.0),
                (74.0, 16.0),
            ],
            color: Some("#89b4fa".into()),
        },
        CanvasShape::Rect {
            x: 30.0,
            y: 5.0,
            width: 18.0,
            height: 10.0,
            color: Some("#fab387".into()),
        },
        CanvasShape::Circle {
            x: 6.0,
            y: 4.0,
            radius: 5.0,
            color: Some("#f38ba8".into()),
        },
        CanvasShape::Circle {
            x: 74.0,
            y: 16.0,
            radius: 5.0,
            color: Some("#a6e3a1".into()),
        },
        CanvasShape::Print {
            x: 2.0,
            y: 17.5,
            text: "eu-1".into(),
            color: Some("#f38ba8".into()),
        },
        CanvasShape::Print {
            x: 60.0,
            y: 2.0,
            text: "ap-1".into(),
            color: Some("#a6e3a1".into()),
        },
    ]
}
