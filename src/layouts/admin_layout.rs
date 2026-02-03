use leptos::prelude::*;
use leptos_router::{LazyRoute, components::Outlet, lazy_route};

use crate::components::Sidebar;

pub struct AdminLayout;

#[lazy_route]
impl LazyRoute for AdminLayout {
    fn data() -> Self {
        AdminLayout
    }

    fn view(this: Self) -> AnyView {
        let _ = this;
        let (sidebar_collapsed, set_sidebar_collapsed) = signal(false);

        view! {
            <div class="min-h-screen bg-neutral-950 text-white flex w-full">
                // Sidebar
                <aside class=move || format!(
                    "sticky left-0 top-0 h-screen bg-black border-r border-neutral-800 transition-all duration-300 z-40 {}",
                    if sidebar_collapsed.get() { "w-20" } else { "w-72" }
                )>
                    // Logo & Toggle
                    <div class="h-20 flex items-center justify-between px-6 border-b border-neutral-800">
                        <Show
                            when=move || !sidebar_collapsed.get()
                            fallback=|| view! { <span class="text-2xl font-black text-red-600">"B"</span> }
                        >
                            <h1 class="text-2xl font-black text-red-600 tracking-tight">"BookFlix"</h1>
                        </Show>
                        <button
                            on:click=move |_| set_sidebar_collapsed.update(|v| *v = !*v)
                            class="p-2 hover:bg-neutral-900 rounded-lg transition"
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                            </svg>
                        </button>
                    </div>

                    // Navigation
                    <Sidebar sidebar_collapsed=sidebar_collapsed />

                    // User Profile at Bottom
                    <div class="absolute bottom-0 left-0 right-0 p-4 border-t border-neutral-800">
                        <div class="flex items-center space-x-3">
                            <div class="w-10 h-10 bg-gradient-to-br from-red-600 to-red-800 rounded-lg flex items-center justify-center font-bold flex-shrink-0">
                                "AD"
                            </div>
                            <Show when=move || !sidebar_collapsed.get()>
                                <div class="flex-1 min-w-0">
                                    <p class="font-semibold truncate">"Admin User"</p>
                                    <p class="text-xs text-neutral-400 truncate">"admin@netflix.com"</p>
                                </div>
                            </Show>
                        </div>
                    </div>
                </aside>

                // Main Content
                <main class="flex-1 transition-all duration-300 ">
                    // Top Bar
                    <header class="h-20 bg-black bg-opacity-60 backdrop-blur-xl border-b border-neutral-800 sticky top-0 z-30">
                        <div class="h-full px-8 flex items-center justify-between">
                            <div class="flex items-center space-x-4">
                                <h2 class="text-2xl font-bold">"Dashboard"</h2>
                            </div>

                            <div class="flex items-center space-x-4">
                                // Search
                                <div class="relative">
                                    <input
                                        type="text"
                                        placeholder="Search..."
                                        class="w-80 bg-neutral-900 border border-neutral-800 rounded-lg px-4 py-2 pl-10 focus:outline-none focus:border-red-600 transition"
                                    />
                                    <svg class="w-5 h-5 absolute left-3 top-2.5 text-neutral-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                                    </svg>
                                </div>

                                // Notifications
                                <button class="relative p-2 hover:bg-neutral-900 rounded-lg transition">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                                    </svg>
                                    <span class="absolute top-1 right-1 w-2 h-2 bg-red-600 rounded-full animate-pulse"></span>
                                </button>
                            </div>
                        </div>
                    </header>

                    // Page Content
                    <div class="p-8">
                        <Outlet />
                    </div>
                </main>
            </div>
        }.into_any()
    }
}
