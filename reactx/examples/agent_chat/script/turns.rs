//! Canned prompts, reasoning, tool calls and answers for the demo turns.

pub(super) struct Turn {
    pub(super) prompt: &'static str,
    pub(super) reasoning: &'static [&'static str],
    pub(super) tool: (&'static str, &'static str),
    pub(super) output: &'static [&'static str],
    pub(super) answer: &'static [&'static str],
}

pub(super) const TURNS: [Turn; 3] = [
    Turn {
        prompt: "Make the agent chat example keep updating",
        reasoning: &[
            "Scanning the transcript layout and scroll offsets",
            "Planning a scripted stream so every tick adds a line",
            "Running the widget suite before the demo",
        ],
        tool: ("Bash", "cargo test -p reactx --tests"),
        output: &[
            "running 23 tests",
            "test framework::headless ... ok",
            "test widgets::transform ... ok",
            "test result: ok. 23 passed; 0 failed",
        ],
        answer: &[
            "The demo loop is running — the transcript grows on every tick.",
            "Spinner, context usage and tool output all come from live signals.",
            "Press Enter to turn your message into a new turn.",
        ],
    },
    Turn {
        prompt: "Patch the runtime so paste events stream",
        reasoning: &[
            "Reading crates/reactx-tui/src/runtime/input.rs",
            "Paste handlers run on the same thread as the event loop",
            "Wiring dispatch_paste before focused widgets see the event",
        ],
        tool: ("Read", "crates/reactx-tui/src/runtime/input.rs"),
        output: &[
            "  1  use std::cell::RefCell;",
            "  2  use dioxus_core::VirtualDom;",
            "  …",
            " 51  pub fn use_input(handler: impl FnMut(String, TuiKeyEvent) + 'static)",
        ],
        answer: &[
            "Paste dispatch is already wired — hooks get events as soon as they are active.",
            "Press Ctrl+O to expand the collapsed tool output.",
        ],
    },
    Turn {
        prompt: "Show the 0.1.1 release going out",
        reasoning: &[
            "Bumping workspace crates to 0.1.1 (reactx-ui 0.1.2)",
            "Publishing seven crates in dependency order",
            "Pushing main to github.com/ananddub/reactx",
        ],
        tool: ("Bash", "cargo publish -p reactx --allow-dirty"),
        output: &[
            "Packaged 96 files (52.7KiB compressed)",
            "Uploaded reactx v0.1.1 to registry `crates-io`",
            "Published reactx v0.1.1 at registry `crates-io`",
        ],
        answer: &[
            "The release is live — this binary also runs from the crates.io build.",
            "This stream is canned for the demo, but the hooks and renderer are real.",
        ],
    },
];

pub(super) const USER_TURN: Turn = Turn {
    prompt: "",
    reasoning: &[
        "Your message is queued, the scripted turn is starting",
        "Runtime hooks and the renderer are real; the text is canned for the demo",
        "You can start a new turn from the composer",
    ],
    tool: ("Bash", "cargo run -p reactx --example agent_chat"),
    output: &[
        "Compiling reactx v0.1.1",
        "Finished dev profile in 1.2s",
        "Running `target/debug/examples/agent_chat`",
    ],
    answer: &[
        "Message received — a scripted response is rendering like a demo agent.",
        "Ctrl+O expands the collapsed tool output in place.",
        "This example exercises use_ticker, use_input and use_terminal_size.",
    ],
};
