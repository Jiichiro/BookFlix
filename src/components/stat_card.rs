use leptos::prelude::*;

#[component]
#[allow(non_snake_case)]
pub fn StatsCard(
    title: &'static str,
    value: &'static str,
    change: &'static str,
    trend: &'static str,
    icon: &'static str,
) -> impl IntoView {
    let (badge_bg, badge_text) = if trend == "up" {
        ("bg-green-500 bg-opacity-20", "text-white")
    } else {
        ("bg-red-500 bg-opacity-20", "text-white")
    };

    view! {
        <div class="bg-neutral-900 border border-neutral-800 rounded-xl p-6 hover:border-neutral-700 transition-colors">
            <div class="flex items-start justify-between mb-4">
                <div class="p-3 bg-red-600 bg-opacity-10 rounded-lg">
                    <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=icon />
                    </svg>
                </div>
                <span class=format!("text-xs font-semibold px-2 py-1 rounded-full {}", badge_bg)>
                    <span class=badge_text>{change}</span>
                </span>
            </div>
            <p class="text-neutral-400 text-sm mb-1">{title}</p>
            <p class="text-3xl font-bold">{value}</p>
        </div>
    }
}