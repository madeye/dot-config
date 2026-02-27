use leptos::prelude::*;
use std::collections::HashMap;

use crate::components::layout::ConfigPage;
use crate::generators::ssh::generate_ssh;
use crate::models::ssh::ssh_host_schema;
use crate::parsers::ssh::parse_ssh;
use crate::server::file_ops::{read_dotfile, write_dotfile};

#[component]
pub fn SshPage() -> impl IntoView {
    let schema = ssh_host_schema();
    let defaults = schema.defaults_map();

    let hosts = RwSignal::new(Vec::<HashMap<String, String>>::new());
    let current_host_idx = RwSignal::new(0usize);
    let extra_lines = RwSignal::new(Vec::<String>::new());
    let file_exists = RwSignal::new(false);
    let values = RwSignal::new(defaults.clone());

    let defaults_clone = defaults.clone();
    let load_resource = Resource::new(|| (), move |_| {
        let defaults = defaults_clone.clone();
        async move {
            match read_dotfile("~/.ssh/config".to_string()).await {
                Ok(result) if result.exists => {
                    let (parsed_hosts, extras) = parse_ssh(&result.content);
                    if parsed_hosts.is_empty() {
                        (vec![defaults], extras, false)
                    } else {
                        (parsed_hosts, extras, true)
                    }
                }
                _ => (vec![defaults], Vec::new(), false),
            }
        }
    });

    Effect::new(move || {
        if let Some((h, extras, exists)) = load_resource.get() {
            hosts.set(h.clone());
            extra_lines.set(extras);
            file_exists.set(exists);
            if let Some(first) = h.first() {
                values.set(first.clone());
            }
            current_host_idx.set(0);
        }
    });

    // Sync current values back to hosts list
    Effect::new(move || {
        let idx = current_host_idx.get();
        let vals = values.get();
        hosts.update(|h| {
            if idx < h.len() {
                h[idx] = vals;
            }
        });
    });

    let preview_text = Memo::new(move |_| {
        generate_ssh(&hosts.get(), &extra_lines.get())
    });

    let schema_clone = schema.clone();

    let on_save = Callback::new(move |_: ()| {
        let content = generate_ssh(&hosts.get_untracked(), &extra_lines.get_untracked());
        leptos::task::spawn_local(async move {
            let _ = write_dotfile("~/.ssh/config".to_string(), content).await;
        });
    });

    let on_reset = {
        let defaults = schema.defaults_map();
        Callback::new(move |_: ()| {
            values.set(defaults.clone());
            hosts.set(vec![defaults.clone()]);
            extra_lines.set(Vec::new());
            current_host_idx.set(0);
        })
    };

    let on_import = Callback::new(move |_: ()| {
        let defaults = schema_clone.defaults_map();
        leptos::task::spawn_local(async move {
            if let Ok(result) = read_dotfile("~/.ssh/config".to_string()).await {
                if result.exists {
                    let (parsed_hosts, extras) = parse_ssh(&result.content);
                    if !parsed_hosts.is_empty() {
                        values.set(parsed_hosts[0].clone());
                        hosts.set(parsed_hosts);
                    } else {
                        hosts.set(vec![defaults]);
                    }
                    extra_lines.set(extras);
                    file_exists.set(true);
                    current_host_idx.set(0);
                }
            }
        });
    });

    let host_schema = ssh_host_schema();

    view! {
        <div class="ssh-page">
            <div class="ssh-host-list">
                <h3>"Host Entries"</h3>
                <For
                    each=move || {
                        let h = hosts.get();
                        (0..h.len()).collect::<Vec<_>>()
                    }
                    key=|idx| *idx
                    children=move |idx| {
                        let is_active = Signal::derive(move || current_host_idx.get() == idx);
                        let host_name = Signal::derive(move || {
                            hosts.get()
                                .get(idx)
                                .and_then(|h| h.get("Host").cloned())
                                .unwrap_or_else(|| format!("Host {}", idx + 1))
                        });
                        view! {
                            <button
                                class="host-entry"
                                class:active=is_active
                                on:click=move |_| {
                                    let cur_vals = values.get_untracked();
                                    let cur_idx = current_host_idx.get_untracked();
                                    hosts.update(|h| {
                                        if cur_idx < h.len() {
                                            h[cur_idx] = cur_vals;
                                        }
                                    });
                                    current_host_idx.set(idx);
                                    if let Some(h) = hosts.get_untracked().get(idx) {
                                        values.set(h.clone());
                                    }
                                }
                            >
                                {host_name}
                            </button>
                        }
                    }
                />
                <button class="btn btn-secondary btn-sm" on:click=move |_| {
                    let new_host = ssh_host_schema().defaults_map();
                    hosts.update(|h| h.push(new_host.clone()));
                    let new_idx = hosts.get_untracked().len() - 1;
                    current_host_idx.set(new_idx);
                    values.set(hosts.get_untracked()[new_idx].clone());
                }>"+ Add Host"</button>
            </div>
            <ConfigPage
                schema=host_schema
                values=values
                extra_lines=extra_lines
                preview_text=preview_text
                file_exists=file_exists
                on_save=on_save
                on_reset=on_reset
                on_import=on_import
            />
        </div>
    }
}
