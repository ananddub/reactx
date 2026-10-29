//! A todo list with a controlled input, checkboxes and a filter, written with
//! HTML-like `jsx!` syntax.
//!
//! Run with: `cargo run -p reactx --example todo`

#![allow(non_snake_case)]

use reactx::prelude::*;

#[derive(Clone, PartialEq)]
struct Todo {
    id: u64,
    text: String,
    done: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Filter {
    All,
    Active,
    Done,
}

impl Filter {
    fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Active => "Active",
            Self::Done => "Done",
        }
    }

    fn matches(self, todo: &Todo) -> bool {
        match self {
            Self::All => true,
            Self::Active => !todo.done,
            Self::Done => todo.done,
        }
    }
}

fn App() -> Element {
    let mut todos = use_signal(Vec::<Todo>::new);
    let mut draft = use_signal(String::new);
    let mut next_id = use_signal(|| 1u64);
    let mut filter = use_signal(|| Filter::All);

    let mut add = move || {
        let text = draft().trim().to_string();
        if text.is_empty() {
            return;
        }
        let id = next_id();
        next_id += 1;
        todos.write().push(Todo {
            id,
            text,
            done: false,
        });
        draft.set(String::new());
    };

    let visible: Vec<Todo> = todos()
        .into_iter()
        .filter(|todo| filter().matches(todo))
        .collect();
    let remaining = todos().iter().filter(|todo| !todo.done).count();
    let title = format!("{remaining} remaining");

    jsx! {
        <ThemeProvider>
            <Container class="flex flex-col border border-blue-300 rounded p-1 gap-1">
                <span class="bold">Todos</span>
                <div class="flex flex-row gap-1">
                    <Input
                        class="grow"
                        value={draft()}
                        placeholder="What needs doing?"
                        on_input=move |value| draft.set(value)
                        on_submit=move |_| add()
                    />
                    <Button on_click=move |_| add()>Add</Button>
                </div>
                <div class="flex flex-row gap-1">
                    {[Filter::All, Filter::Active, Filter::Done].into_iter().map(|option| jsx! {
                        <Button
                            key="{option.label()}"
                            variant={if filter() == option { "primary" } else { "ghost" }}
                            on_click=move |_| filter.set(option)
                        >{option.label()}</Button>
                    })}
                </div>
                <List title={Some(title)}>
                    {visible.is_empty().then(|| jsx! {
                        <ListItem><span class="dim">Nothing here yet.</span></ListItem>
                    })}
                    {visible.into_iter().map(|todo| jsx! {
                        <ListItem key="{todo.id}">
                            <div class="flex flex-row gap-1">
                                <Checkbox
                                    checked={todo.done}
                                    on_change=move |done| {
                                        if let Some(entry) = todos.write().iter_mut().find(|entry| entry.id == todo.id) {
                                            entry.done = done;
                                        }
                                    }
                                />
                                {if todo.done {
                                    jsx! { <span class="dim line-through">{todo.text}</span> }
                                } else {
                                    jsx! { <span>{todo.text}</span> }
                                }}
                                <Button
                                    variant="danger"
                                    on_click=move |_| todos.write().retain(|entry| entry.id != todo.id)
                                >x</Button>
                            </div>
                        </ListItem>
                    })}
                </List>
                <span class="dim">Enter submits. Tab moves focus. Ctrl+C quits.</span>
            </Container>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
