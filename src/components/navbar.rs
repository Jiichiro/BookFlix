use leptos::{*, html::*, prelude::*};
use leptos_router::components::A;
use crate::server::auth::logout;

#[component]
#[allow(non_snake_case)]
pub fn NavBar() -> impl IntoView {
    let logout_btn = Action::new(|_: &()| async move {
        let _ = logout().await;
        web_sys::window().unwrap().location().reload().unwrap();
    });

    let (is_mobile_menu_open, set_is_mobile_menu_open) = signal(false);
    let (is_profile_menu_open, set_is_profile_menu_open) = signal(false);

    let toggle_mobile_menu = move |_| {
        set_is_mobile_menu_open.update(|open| *open = !*open);
    };

    let close_mobile_menu = move |_| {
        set_is_mobile_menu_open.set(false);
    };

    let toggle_profile_menu = move |_| {
        set_is_profile_menu_open.update(|open| *open = !*open);
    };

    let close_profile_menu = move |_| {
        set_is_profile_menu_open.set(false);
    };

    view! {
        <nav class="fixed top-0 w-full bg-gradient-to-b from-gray-900 to-transparent z-50 px-4 md:px-8 py-4 text-white">
            <div class="flex items-center justify-between">
                <div class="flex items-center space-x-4 md:space-x-8">
                    <h1 class="text-2xl md:text-3xl font-bold text-red-600">BookFlix</h1>
                    
                    // Desktop menu
                    <div class="hidden md:flex space-x-6">
                        <A href="/dash" attr:class="hover:text-gray-300 transition">Beranda</A>
                        <A href="/genre" attr:class="hover:text-gray-300 transition">Genre</A>
                        <A href="/best-seller" attr:class="hover:text-gray-300 transition">Best Seller</A>
                        <A href="/koleksi" attr:class="hover:text-gray-300 transition">Koleksi Saya</A>
                    </div>
                </div>

                <div class="flex items-center space-x-3 md:space-x-6">
                    // Desktop icons
                    <button class="hover:text-gray-300 hidden md:block">
                        <svg class="w-6 h-6" fill="none" stroke="#ffffff" viewBox="0 0 24 24" aria-label="search">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                        </svg>
                    </button>
                    <button class="hover:text-gray-300 hidden md:block">
                        <svg class="w-6 h-6" fill="none" stroke="#ffffff" viewBox="0 0 24 24" aria-label="notifications">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"></path>
                        </svg>
                    </button>

                    // Desktop profile dropdown
                    <div class="relative hidden md:block">
                        <button 
                            on:click=toggle_profile_menu
                            class="w-8 h-8 bg-red-600 rounded flex items-center justify-center font-semibold cursor-pointer hover:bg-red-700 transition"
                        >
                            A
                        </button>

                        // Desktop profile dropdown menu
                        <Show when=move || is_profile_menu_open.get()>
                            <div class="absolute right-0 mt-2 w-56 bg-gray-800 rounded-lg shadow-lg py-2 border border-gray-700">
                                <div class="px-4 py-3 border-b border-gray-700">
                                    <p class="text-sm font-semibold">Admin User</p>
                                    <p class="text-xs text-gray-400">admin@bookflix.com</p>
                                </div>
                                <A 
                                    href="/profile" 
                                    attr:class="flex items-center space-x-3 px-4 py-2 hover:bg-gray-700 transition"
                                    on:click=close_profile_menu
                                >
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                                    </svg>
                                    <span class="text-sm">Profil Saya</span>
                                </A>
                                <A 
                                    href="/settings" 
                                    attr:class="flex items-center space-x-3 px-4 py-2 hover:bg-gray-700 transition"
                                    on:click=close_profile_menu
                                >
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                    </svg>
                                    <span class="text-sm">Pengaturan</span>
                                </A>
                                <A 
                                    href="/help" 
                                    attr:class="flex items-center space-x-3 px-4 py-2 hover:bg-gray-700 transition"
                                    on:click=close_profile_menu
                                >
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                    </svg>
                                    <span class="text-sm">Bantuan</span>
                                </A>
                                <div class="border-t border-gray-700 mt-2 pt-2">
                                    <button 
                                        on:click=move |m| { 
                                            logout_btn.dispatch(()); 
                                            close_profile_menu(m);
                                        }
                                        class="flex items-center space-x-3 px-4 py-2 hover:bg-gray-700 transition w-full text-left text-red-400"
                                    >
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path>
                                        </svg>
                                        <span class="text-sm">Logout</span>
                                    </button>
                                </div>
                            </div>
                        </Show>
                    </div>

                    // Hamburger button (mobile only)
                    <button 
                        on:click=toggle_mobile_menu
                        class="md:hidden hover:text-gray-300 focus:outline-none"
                        aria-label="Toggle menu"
                    >
                        <Show
                            when=move || !is_mobile_menu_open.get()
                            fallback=move || view! {
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                </svg>
                            }
                        >
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                            </svg>
                        </Show>
                    </button>
                </div>
            </div>

            // Mobile menu
            <Show when=move || is_mobile_menu_open.get()>
                <div class="md:hidden mt-4 pb-4 space-y-3 border-t border-gray-700 pt-4">
                    <A 
                        href="/dash" 
                        attr:class="block hover:text-gray-300 transition py-2"
                        on:click=close_mobile_menu
                    >
                        Beranda
                    </A>
                    <A 
                        href="/genre" 
                        attr:class="block hover:text-gray-300 transition py-2"
                        on:click=close_mobile_menu
                    >
                        Genre
                    </A>
                    <A 
                        href="/best-seller" 
                        attr:class="block hover:text-gray-300 transition py-2"
                        on:click=close_mobile_menu
                    >
                        Best Seller
                    </A>
                    <A 
                        href="/koleksi" 
                        attr:class="block hover:text-gray-300 transition py-2"
                        on:click=close_mobile_menu
                    >
                        Koleksi Saya
                    </A>

                    // Divider
                    <div class="border-t border-gray-700 pt-3 mt-3"></div>

                    // Search button
                    <button 
                        class="flex items-center space-x-3 hover:text-gray-300 transition py-2 w-full"
                        on:click=close_mobile_menu
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                        </svg>
                        <span>Cari</span>
                    </button>

                    // Notifications button
                    <button 
                        class="flex items-center space-x-3 hover:text-gray-300 transition py-2 w-full"
                        on:click=close_mobile_menu
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"></path>
                        </svg>
                        <span>Notifikasi</span>
                    </button>

                    // Divider
                    <div class="border-t border-gray-700 pt-3 mt-3"></div>

                    // Mobile profile section
                    <div class="flex items-center justify-between py-2">
                        <div class="flex items-center space-x-3">
                            <div class="w-8 h-8 bg-red-600 rounded flex items-center justify-center font-semibold text-sm">
                                A
                            </div>
                            <div>
                                <p class="text-sm font-semibold">Admin User</p>
                                <p class="text-xs text-gray-400">admin@bookflix.com</p>
                            </div>
                        </div>
                        <button 
                            on:click=move |m| { 
                                logout_btn.dispatch(()); 
                                close_mobile_menu(m);
                            }
                            class="text-red-500 hover:text-red-400 transition"
                            aria-label="Logout"
                        >
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path>
                            </svg>
                        </button>
                    </div>

                    // Mobile profile links
                    <A 
                        href="/profile" 
                        attr:class="flex items-center space-x-3 hover:text-gray-300 transition py-2"
                        on:click=close_mobile_menu
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                        </svg>
                        <span>Profil Saya</span>
                    </A>
                    <A 
                        href="/settings" 
                        attr:class="flex items-center space-x-3 hover:text-gray-300 transition py-2"
                        on:click=close_mobile_menu
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                        </svg>
                        <span>Pengaturan</span>
                    </A>
                    <A 
                        href="/help" 
                        attr:class="flex items-center space-x-3 hover:text-gray-300 transition py-2"
                        on:click=close_mobile_menu
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                        <span>Bantuan</span>
                    </A>
                </div>
            </Show>
        </nav>
    }
}