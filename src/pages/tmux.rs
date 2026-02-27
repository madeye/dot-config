use leptos::prelude::*;

use crate::components::layout::ConfigPage;
use crate::generators::tmux::generate_tmux;
use crate::models::tmux::tmux_schema;
use crate::parsers::tmux::parse_tmux;
use crate::server::file_ops::{read_dotfile, write_dotfile};

#[component]
pub fn TmuxPage() -> impl IntoView {
    let schema = tmux_schema();
    let defaults = schema.defaults_map();

    let values = RwSignal::new(defaults.clone());
    let extra_lines = RwSignal::new(Vec::<String>::new());
    let file_exists = RwSignal::new(false);

    let defaults_clone = defaults.clone();
    let load_resource = Resource::new(|| (), move |_| {
        let defaults = defaults_clone.clone();
        async move {
            match read_dotfile("~/.tmux.conf".to_string()).await {
                Ok(result) if result.exists => {
                    let state = parse_tmux(&result.content);
                    let mut merged = defaults;
                    for (k, v) in state.values {
                        merged.insert(k, v);
                    }
                    (merged, state.extra_lines, true)
                }
                _ => (defaults, Vec::new(), false),
            }
        }
    });

    Effect::new(move || {
        if let Some((vals, extras, exists)) = load_resource.get() {
            values.set(vals);
            extra_lines.set(extras);
            file_exists.set(exists);
        }
    });

    let preview_text = Memo::new(move |_| {
        generate_tmux(&values.get(), &extra_lines.get())
    });

    let schema_clone = schema.clone();
    let save_defaults = schema.defaults_map();

    let on_save = Callback::new(move |_: ()| {
        let content = generate_tmux(&values.get_untracked(), &extra_lines.get_untracked());
        leptos::task::spawn_local(async move {
            let _ = write_dotfile("~/.tmux.conf".to_string(), content).await;
        });
    });

    let on_reset = {
        let defaults = save_defaults.clone();
        Callback::new(move |_: ()| {
            values.set(defaults.clone());
            extra_lines.set(Vec::new());
        })
    };

    let on_import = Callback::new(move |_: ()| {
        let defaults = schema_clone.defaults_map();
        leptos::task::spawn_local(async move {
            if let Ok(result) = read_dotfile("~/.tmux.conf".to_string()).await {
                if result.exists {
                    let state = parse_tmux(&result.content);
                    let mut merged = defaults;
                    for (k, v) in state.values {
                        merged.insert(k, v);
                    }
                    values.set(merged);
                    extra_lines.set(state.extra_lines);
                    file_exists.set(true);
                }
            }
        });
    });

    view! {
        <ConfigPage
            schema=schema
            values=values
            extra_lines=extra_lines
            preview_text=preview_text
            file_exists=file_exists
            on_save=on_save
            on_reset=on_reset
            on_import=on_import
        />
    }
}
