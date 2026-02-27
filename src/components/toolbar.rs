use leptos::prelude::*;

#[component]
pub fn Toolbar(
    #[prop(into)] on_save: Callback<()>,
    #[prop(into)] on_reset: Callback<()>,
    #[prop(into)] on_import: Callback<()>,
) -> impl IntoView {
    let save_status = RwSignal::new(String::new());

    view! {
        <div class="toolbar">
            <button class="btn btn-secondary" on:click=move |_| on_import.run(())>
                "Import"
            </button>
            <button class="btn btn-secondary" on:click=move |_| on_reset.run(())>
                "Reset to Defaults"
            </button>
            <button class="btn btn-primary" on:click=move |_| {
                on_save.run(());
                save_status.set("Saved!".into());
            }>
                "Save"
            </button>
            <Show when=move || !save_status.get().is_empty()>
                <span class="save-status">{save_status}</span>
            </Show>
        </div>
    }
}
