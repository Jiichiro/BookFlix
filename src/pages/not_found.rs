use leptos::{html::*, prelude::*, *};
use leptos_meta::Title;

#[component]
#[allow(non_snake_case)]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text="Halaman Tidak Ditemukan"/>
        <div
        class="min-h-screen bg-black text-white flex items-center justify-center p-6"
        >
        <div
            class="max-w-4xl w-full grid grid-cols-1 md:grid-cols-2 gap-8 items-center"
        >
            <section class="flex flex-col items-start gap-6 md:gap-8">
            <div class="flex items-center gap-4">
                <div
                class="w-14 h-14 md:w-20 md:h-20 bg-red-600 rounded-sm flex items-center justify-center shadow-lg"
                >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    class="w-7 h-7 md:w-9 md:h-9"
                >
                    <path fill="white" d="M8 5v14l11-7z" />
                </svg>
                </div>
                <div>
                <h1
                    class="text-4xl md:text-5xl font-extrabold tracking-tight"
                >404</h1>
                <p class="mt-1 text-sm md:text-base opacity-80">{ "Halaman tidak ditemukan" }</p>
                </div>
            </div>
            <h2 class="text-2xl md:text-3xl font-bold leading-tight">Coba periksa link anda</h2>
            <p class="text-sm md:text-base max-w-prose opacity-80">{"Kami tidak menemukan buku yang sesuai dengan link anda. Link mungkin sudah kadaluarsa atau tidak ada disistem kami"}</p>
            <div class="flex gap-3 mt-2">
                <a
                href="/dash"
                class="inline-flex items-center gap-2 px-4 py-2 bg-red-600 hover:bg-red-700 transition rounded-md font-medium shadow"
                >Kembali ke Beranda</a>
                <button
                on:click=move |_| { web_sys::window().unwrap().location().reload().unwrap(); }
                class="inline-flex items-center gap-2 px-4 py-2 border border-neutral-700 rounded-md text-sm hover:bg-white/5 transition"
                >
                Coba Muat Ulang
                </button>
            </div>

            <small class="text-xs opacity-60">Butuh bantuan? Cek menu bantuan atau
                hubungi support.</small>
            </section>

            {/* Right: stylized poster / glass card */}
            <aside class="relative flex items-center justify-center">
            <div
                class="w-full h-64 md:h-80 rounded-lg overflow-hidden shadow-2xl border border-neutral-800 bg-gradient-to-b from-neutral-900 via-neutral-900/60 to-black"
            >
                <div class="absolute inset-0 mix-blend-overlay opacity-20" />

                {/* poster content */}
                <div class="h-full flex flex-col justify-end p-6">
                <div class="backdrop-blur-sm bg-black/30 p-4 rounded">
                    <h3 class="text-lg font-bold">Judul Misterius</h3>
                    <p class="text-xs opacity-80 mt-1">{ "Tidak tersedia lagi — halaman
                    ini seperti koleksi yang dihapus." }</p>

                    <div class="mt-4 flex items-center gap-3">
                    <button
                        class="px-3 py-1 rounded-md bg-red-600 font-semibold"
                    >Putar Trailer</button>
                    <button
                        class="px-3 py-1 rounded-md border border-neutral-700 text-sm"
                    >Tambahkan ke Daftar</button>
                    </div>
                </div>
                </div>
            </div>

            {/* subtle decorative big watermark like "NETFLIX" but generic to avoid
            trademark issues */}
            <div
                class="pointer-events-none absolute -bottom-6 -right-6 text-[8rem] font-black tracking-tight opacity-5 select-none"
            >NFX</div>
            </aside>
        </div>

        {/* small cinematic gradient at bottom */}
        <div
            class="absolute inset-x-0 bottom-0 h-40 bg-gradient-to-t from-black/90 to-transparent pointer-events-none"
        />
        </div>
    }
}
