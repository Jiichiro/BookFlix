use leptos::{*, html::*, prelude::*};
use leptos_router::components::A;

#[component]
#[allow(non_snake_case)]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="fixed top-0 w-full bg-gradient-to-b from-gray-900 to-transparent z-50 px-8 py-4 text-white">
            <div class="flex items-center justify-between">
                <div class="flex items-center space-x-8">
                    <h1 class="text-3xl font-bold text-red-600">BookFlix</h1>
                    <div class="hidden md:flex space-x-6">
                        <A href="#" attr:class="hover:text-gray-300 transition">Beranda</A>
                        <A href="#" attr:class="hover:text-gray-300 transition">Genre</A>
                        <A href="#" attr:class="hover:text-gray-300 transition">Best Seller</A>
                        <A href="#" attr:class="hover:text-gray-300 transition">Koleksi Saya</A>
                    </div>
                </div>
                <div class="flex items-center space-x-6">
                    <button class="hover:text-gray-300">
                        <svg class="w-6 h-6" fill="none" stroke="#ffffff" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                        </svg>
                    </button>
                    <button class="hover:text-gray-300">
                        <svg class="w-6 h-6" fill="none" stroke="#ffffff" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"></path>
                        </svg>
                    </button>
                    <div class="w-8 h-8 bg-red-600 rounded flex items-center justify-center font-semibold">
                        A
                    </div>
                </div>
            </div>
        </nav>
    }
}
