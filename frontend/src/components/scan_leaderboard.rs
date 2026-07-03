//! Leaderboard display component for Scan.

use crate::api::ApiService;
use crate::components::scan_logic::Sector;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub sector: Sector,
    pub reload_trigger: usize,
}

#[function_component(ScanLeaderboard)]
pub fn scan_leaderboard(props: &Props) -> Html {
    let sector = props.sector;
    let reload_trigger = props.reload_trigger;
    let entries = use_state(Vec::new);

    {
        let entries = entries.clone();
        use_effect_with((sector, reload_trigger), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let category = sector.name();
                if let Ok(list) = ApiService::get_leaderboard(category).await {
                    entries.set(list);
                } else {
                    entries.set(Vec::new());
                }
            });
        });
    }

    html! {
        <div class="inline-leaderboard">
            { if entries.is_empty() {
                html! { <span class="leaderboard-empty-inline">{"-"}</span> }
            } else {
                html! {
                    { for entries.iter().take(3).enumerate().map(|(idx, entry)| {
                        html! {
                            <span key={idx} class="leaderboard-inline-item">
                                <span class="leader-name">{ format!("{}. {}", idx + 1, entry.name) }</span>
                                { " (" }
                                <span class="leader-score">{ format!("{:.1}s", entry.score as f64 / 10.0) }</span>
                                { ")" }
                            </span>
                        }
                    }) }
                }
            } }
        </div>
    }
}
