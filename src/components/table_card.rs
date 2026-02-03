use leptos::prelude::*;

#[component]
#[allow(non_snake_case)]
pub fn TableCard(title: &'static str) -> impl IntoView {
    view! {
        <div class="bg-neutral-900 border border-neutral-800 rounded-xl p-6">
            <h3 class="text-lg font-semibold mb-6">{title}</h3>
            <div class="space-y-4">
                {(0..5).map(|i| {
                    view! {
                        <div class="flex items-center justify-between p-3 hover:bg-neutral-800 rounded-lg transition-colors cursor-pointer">
                            <div class="flex items-center space-x-3">
                                <div class="w-10 h-10 bg-gradient-to-br from-red-600 to-red-800 rounded-lg flex items-center justify-center font-bold">
                                    {i + 1}
                                </div>
                                <div>
                                    <p class="font-medium">"Content Title "{i + 1}</p>
                                    <p class="text-sm text-neutral-500">"Category • 2024"</p>
                                </div>
                            </div>
                            <span class="text-neutral-400 text-sm">"1.2M views"</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}