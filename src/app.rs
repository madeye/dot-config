use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment,
};

use crate::pages::home::HomePage;
use crate::pages::vimrc::VimrcPage;
use crate::pages::shellrc::ShellrcPage;
use crate::pages::gitconfig::GitconfigPage;
use crate::pages::tmux::TmuxPage;
use crate::pages::ssh::SshPage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/dot-config.css"/>
        <Title text="dot-config"/>
        <Router>
            <div class="app-layout">
                <Sidebar/>
                <main class="main-content">
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=StaticSegment("vimrc") view=VimrcPage/>
                        <Route path=StaticSegment("shellrc") view=ShellrcPage/>
                        <Route path=StaticSegment("gitconfig") view=GitconfigPage/>
                        <Route path=StaticSegment("tmux") view=TmuxPage/>
                        <Route path=StaticSegment("ssh") view=SshPage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    view! {
        <nav class="sidebar">
            <div class="sidebar-header">
                <A href="/" attr:class="sidebar-logo">"dot-config"</A>
            </div>
            <ul class="sidebar-nav">
                <li><A href="/vimrc" attr:class="sidebar-link">"Vimrc"</A></li>
                <li><A href="/shellrc" attr:class="sidebar-link">"Shell (Bash/Zsh)"</A></li>
                <li><A href="/gitconfig" attr:class="sidebar-link">"Git Config"</A></li>
                <li><A href="/tmux" attr:class="sidebar-link">"Tmux"</A></li>
                <li><A href="/ssh" attr:class="sidebar-link">"SSH Config"</A></li>
            </ul>
        </nav>
    }
}
