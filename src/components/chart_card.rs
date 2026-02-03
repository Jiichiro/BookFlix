use leptos::prelude::*;

#[component]
#[allow(non_snake_case)]
pub fn ChartCard(title: &'static str) -> impl IntoView {
    view! {
        <div class="bg-neutral-900 border border-neutral-800 rounded-xl p-6">
            <h3 class="text-lg font-semibold mb-6">{title}</h3>
            <div class="h-64 flex items-end justify-between space-x-2">
                {(0..12).map(|i| {
                    let height = (i + 1) * 8;
                    view! {
                        <div class="flex-1 bg-gradient-to-t from-red-600 to-red-400 rounded-t-lg hover:from-red-500 hover:to-red-300 transition-all duration-300 cursor-pointer"
                            style=format!("height: {}%", height)>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
            <div class="flex justify-between mt-4 text-xs text-neutral-500">
                <span>"Jan"</span>
                <span>"Dec"</span>
            </div>
        </div>
    }
}