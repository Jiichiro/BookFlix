use leptos::prelude::*;
use leptos_router::{hooks::use_location, components::A};

#[component]
#[allow(non_snake_case)]
pub fn Sidebar(sidebar_collapsed: ReadSignal<bool>) -> impl IntoView {
    view! {
        <nav class="p-4 space-y-2">
            <SidebarItem
                href="/"
                icon="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
                label="Dashboard"
                collapsed=sidebar_collapsed
            />
            <SidebarItem
                href="/content"
                icon="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z"
                label="Content Library"
                collapsed=sidebar_collapsed
            />
            <SidebarItem
                href="/users"
                icon="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"
                label="Users"
                collapsed=sidebar_collapsed
            />
            <SidebarItem
                href="/analytics"
                icon="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                label="Analytics"
                collapsed=sidebar_collapsed
            />
            <SidebarItem
                href="/revenue"
                icon="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                label="Revenue"
                collapsed=sidebar_collapsed
            />
            <SidebarItem
                href="/settings"
                icon="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                label="Settings"
                collapsed=sidebar_collapsed
            />
        </nav>
    }
}

#[component]
#[allow(non_snake_case)]
fn SidebarItem(
    href: &'static str,
    icon: &'static str,
    label: &'static str,
    collapsed: ReadSignal<bool>,
) -> impl IntoView {
    let location = use_location();

    let is_active = move || {
        let pathname = location.pathname.get();
        if href == "/" {
            pathname == "/"
        } else {
            pathname.starts_with(href)
        }
    };

    view! {
        <A
            href=href
            attr:class=move || {
                let base = "flex items-center space-x-3 px-4 py-3 rounded-lg transition-all duration-200 group relative";
                if is_active() {
                    format!("{} bg-red-600 text-white", base)
                } else {
                    format!("{} text-neutral-400 hover:bg-neutral-900 hover:text-white", base)
                }
            }
        >
            <svg class="w-6 h-6 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=icon />
            </svg>
            <Show when=move || !collapsed.get()>
                <span class="font-medium">{label}</span>
            </Show>

            // Tooltip for collapsed state
            <Show when=move || collapsed.get()>
                <div class="absolute left-full ml-2 px-3 py-2 bg-neutral-900 text-white text-sm rounded-lg opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap">
                    {label}
                </div>
            </Show>
        </A>
    }
}
