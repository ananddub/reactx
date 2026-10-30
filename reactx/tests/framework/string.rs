//! Headless string rendering, line flow and text truncation.

use reactx::prelude::*;

fn truncate_app() -> Element {
    jsx! {
        <Box direction="column">
            <Text wrap="truncate-end">{"truncated text here"}</Text>
        </Box>
    }
}

#[test]
fn render_to_string_truncates_and_trims() {
    assert_eq!(render_to_string(truncate_app, 10, 4), "truncated…");
}

fn flow_app() -> Element {
    jsx! {
        <Box direction="column">
            <Text>{"one"}<Newline/>{"two"}</Text>
            <Box direction="row">
                <Text>a</Text>
                <Spacer/>
                <Text>b</Text>
            </Box>
        </Box>
    }
}

#[test]
fn newline_and_spacer_lay_out_lines() {
    assert_eq!(render_to_string(flow_app, 10, 4), "one\ntwo\na        b");
}
