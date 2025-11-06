use leptos::{*, html::*, prelude::*};

#[allow(non_snake_case)]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-black bg-opacity-50 py-12 px-8 md:px-16 mt-16 text-white">
            <div class="grid grid-cols-2 md:grid-cols-4 gap-8 mb-8">
                <div>
                    <h4 class="font-semibold mb-4">Navigasi</h4>
                    <ul class="space-y-2 text-sm text-gray-400">
                        <li><a href="#" class="hover:text-white">Beranda</a></li>
                        <li><a href="#" class="hover:text-white">Jelajahi</a></li>
                        <li><a href="#" class="hover:text-white">Koleksi Saya</a></li>
                    </ul>
                </div>
                <div>
                    <h4 class="font-semibold mb-4">Genre</h4>
                    <ul class="space-y-2 text-sm text-gray-400">
                        <li><a href="#" class="hover:text-white">Fiksi</a></li>
                        <li><a href="#" class="hover:text-white">Non-Fiksi</a></li>
                        <li><a href="#" class="hover:text-white">Biografi</a></li>
                        <li><a href="#" class="hover:text-white">Fantasi</a></li>
                    </ul>
                </div>
                <div>
                    <h4 class="font-semibold mb-4">Bantuan</h4>
                    <ul class="space-y-2 text-sm text-gray-400">
                        <li><a href="#" class="hover:text-white">Pusat Bantuan</a></li>
                        <li><a href="#" class="hover:text-white">FAQ</a></li>
                        <li><a href="#" class="hover:text-white">Kontak</a></li>
                    </ul>
                </div>
                <div>
                    <h4 class="font-semibold mb-4">Legal</h4>
                    <ul class="space-y-2 text-sm text-gray-400">
                        <li><a href="#" class="hover:text-white">Privasi</a></li>
                        <li><a href="#" class="hover:text-white">Syarat & Ketentuan</a></li>
                        <li><a href="#" class="hover:text-white">Cookie</a></li>
                    </ul>
                </div>
            </div>
            <div class="text-center text-gray-500 text-sm">
                <p>&copy; 2025 BookFlix. Semua hak cipta dilindungi.</p>
            </div>
        </footer>
    }
}