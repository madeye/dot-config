use leptos::prelude::*;
use leptos_router::components::A;

use crate::models::schema::DotfileType;
use crate::server::detect::detect_dotfiles;

#[component]
pub fn HomePage() -> impl IntoView {
    let dotfiles = Resource::new(|| (), |_| detect_dotfiles());

    view! {
        <div class="home-page">
            <h1>"dot-config"</h1>
            <p class="home-subtitle">"Configure your dotfiles with a visual interface"</p>

            <Suspense fallback=move || view! { <p>"Detecting dotfiles..."</p> }>
                {move || {
                    dotfiles.get().map(|result| {
                        match result {
                            Ok(files) => {
                                view! {
                                    <div class="dotfile-grid">
                                        {files.into_iter().map(|df| {
                                            let href = match df.dotfile_type {
                                                DotfileType::Vimrc => "/vimrc",
                                                DotfileType::Shellrc => "/shellrc",
                                                DotfileType::Gitconfig => "/gitconfig",
                                                DotfileType::Tmux => "/tmux",
                                                DotfileType::Ssh => "/ssh",
                                                DotfileType::Editorconfig => "/editorconfig",
                                                DotfileType::Inputrc => "/inputrc",
                                                DotfileType::Npmrc => "/npmrc",
                                                DotfileType::Wgetrc => "/wgetrc",
                                            };
                                            let status = if df.exists { "Found" } else { "Not found" };
                                            let status_class = if df.exists { "status-found" } else { "status-missing" };
                                            view! {
                                                <A href=href attr:class="dotfile-card">
                                                    <h3>{df.name}</h3>
                                                    <p class="dotfile-path">{df.path}</p>
                                                    <span class={format!("dotfile-status {status_class}")}>{status}</span>
                                                </A>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                            Err(e) => {
                                let msg = e.to_string();
                                view! {
                                    <p class="error">"Error: " {msg}</p>
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
