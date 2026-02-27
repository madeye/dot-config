use leptos::prelude::*;

use crate::models::schema::{ConfigSchema, OptionDef, OptionKind};
use crate::components::dropdown::ConfigDropdown;
use crate::components::toggle::ConfigToggle;
use crate::components::number_input::ConfigNumber;
use crate::components::text_input::ConfigText;
use crate::components::section::ConfigSection;
use crate::components::preview::FilePreview;
use crate::components::toolbar::Toolbar;

#[component]
pub fn ConfigPage(
    schema: ConfigSchema,
    values: RwSignal<std::collections::HashMap<String, String>>,
    #[allow(unused)] extra_lines: RwSignal<Vec<String>>,
    preview_text: Memo<String>,
    file_exists: RwSignal<bool>,
    #[prop(into)] on_save: Callback<()>,
    #[prop(into)] on_reset: Callback<()>,
    #[prop(into)] on_import: Callback<()>,
) -> impl IntoView {
    let sections = schema.sections();
    let default_path = schema.default_path.clone();

    view! {
        <div class="config-page">
            <div class="config-header">
                <h1>{schema.name.clone()}" Configuration"</h1>
                <Show when=move || !file_exists.get()>
                    <div class="banner banner-info">
                        "Creating new file — no existing "{default_path.clone()}" found"
                    </div>
                </Show>
            </div>
            <Toolbar
                on_save=on_save
                on_reset=on_reset
                on_import=on_import
            />
            <div class="config-body">
                <div class="config-options">
                    {sections.into_iter().map(|section| {
                        let opts: Vec<OptionDef> = schema.options_for_section(&section)
                            .into_iter()
                            .cloned()
                            .collect();
                        view! {
                            <ConfigSection title=section.clone()>
                                <div class="option-grid">
                                    {opts.into_iter().map(|opt| {
                                        let key = opt.key.clone();
                                        let value = {
                                            let key = key.clone();
                                            Signal::derive(move || {
                                                values.get()
                                                    .get(&key)
                                                    .cloned()
                                                    .unwrap_or_default()
                                            })
                                        };
                                        let on_change = {
                                            let key = key.clone();
                                            Callback::new(move |new_val: String| {
                                                values.update(|v| {
                                                    v.insert(key.clone(), new_val);
                                                });
                                            })
                                        };
                                        view! {
                                            <div class="option-item">
                                                {render_option(opt, value, on_change)}
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </ConfigSection>
                        }
                    }).collect_view()}
                </div>
                <FilePreview content=preview_text/>
            </div>
        </div>
    }
}

fn render_option(
    opt: OptionDef,
    value: Signal<String>,
    on_change: Callback<String>,
) -> impl IntoView {
    let label = opt.label.clone();
    let help = opt.help.clone();

    match opt.kind {
        OptionKind::Dropdown(choices) => view! {
            <ConfigDropdown label=label help=help choices=choices value=value on_change=on_change/>
        }.into_any(),
        OptionKind::Toggle => view! {
            <ConfigToggle label=label help=help value=value on_change=on_change/>
        }.into_any(),
        OptionKind::Number { min, max, step } => view! {
            <ConfigNumber label=label help=help min=min max=max step=step value=value on_change=on_change/>
        }.into_any(),
        OptionKind::Text | OptionKind::TextArea => view! {
            <ConfigText label=label help=help value=value on_change=on_change/>
        }.into_any(),
    }
}
