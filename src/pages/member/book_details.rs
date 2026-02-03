use leptos::{prelude::*};
use leptos_meta::Title;

#[component]
#[allow(non_snake_case)]
pub fn BookDetails() -> impl IntoView {
    let (is_in_list, set_is_in_list) = signal(false);

    let book_title = "The Midnight Library";
    let book_author = "Matt Haig";
    let book_cover = "https://images.unsplash.com/photo-1544947950-fa07a98d237f?w=800&q=80";
    let book_rating = 4.5;
    let book_year = 2020;
    let book_pages = 304;
    let book_genres = vec!["Fiction", "Fantasy", "Philosophy"];
    let book_description = "Nora Seed menemukan dirinya berada di perpustakaan ajaib yang berisi kehidupan alternatif yang bisa dia jalani. Setiap buku di perpustakaan ini mewakili versi berbeda dari hidupnya, dengan pilihan-pilihan berbeda yang pernah dia hadapi. Di antara buku-buku yang tak terbatas ini, dia harus menemukan cara untuk kembali ke kehidupan aslinya dan menemukan makna sesungguhnya dari kebahagiaan.";
    let book_publisher = "Canongate Books";
    let book_language = "English";
    let book_reviews = 12847;
    
    let similar_books = vec![
        ("Anxious People", "https://images.unsplash.com/photo-1512820790803-83ca734da794?w=300&q=80"),
        ("The Invisible Life", "https://images.unsplash.com/photo-1541963463532-d68292c34b19?w=300&q=80"),
        ("Before the Coffee", "https://images.unsplash.com/photo-1495446815901-a7297e633e8d?w=300&q=80"),
        ("The House in the", "https://images.unsplash.com/photo-1524995997946-a1c2e315a42f?w=300&q=80"),
    ];

    view! {
        <Title text=book_title />
        <div class="min-h-screen bg-black text-white">
            // Hero Section
            <div class="relative h-screen">
                // Background Image with Gradient
                <div class="absolute inset-0">
                    <img 
                        src={book_cover}
                        alt={book_title}
                        class="w-full h-full object-cover"
                    />
                    <div class="absolute inset-0 bg-gradient-to-t from-black via-black/60 to-transparent"></div>
                    <div class="absolute inset-0 bg-gradient-to-r from-black via-transparent to-transparent"></div>
                </div>

                // Content
                <div class="relative h-full flex items-end pb-32 px-12">
                    <div class="max-w-2xl">
                        <h1 class="text-6xl font-bold mb-4">{book_title}</h1>
                        
                        <div class="flex items-center gap-4 mb-6">
                            <div class="flex items-center gap-1">
                                <svg class="w-5 h-5 fill-yellow-400 text-yellow-400" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" />
                                </svg>
                                <span class="text-xl font-semibold">{book_rating}</span>
                            </div>
                            <span class="text-gray-400">{book_year}</span>
                            <span class="text-gray-400">{book_pages} " pages"</span>
                            <span class="border border-gray-400 px-2 py-0.5 text-sm">"HD"</span>
                        </div>

                        <div class="flex gap-3 mb-6">
                            {book_genres.iter().cloned().map(|genre| view! {
                                <span class="text-gray-300">{genre}</span>
                            }).collect::<Vec<_>>()}
                        </div>

                        <p class="text-lg mb-8 leading-relaxed">
                            {format!("{}...", &book_description[..180])}
                        </p>

                        // Action Buttons
                        <div class="flex gap-4">
                            <button class="flex items-center gap-2 bg-white text-black px-8 py-3 rounded font-semibold hover:bg-gray-200 transition">
                                <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
                                    <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" />
                                </svg>
                                "Read Now"
                            </button>
                            <button 
                                on:click=move |_| set_is_in_list.update(|v| *v = !*v)
                                class="flex items-center justify-center w-12 h-12 rounded-full border-2 border-gray-400 hover:border-white transition"
                            >
                                <svg 
                                    class=move || format!("w-6 h-6 {} transition-transform", if is_in_list.get() { "rotate-45" } else { "" })
                                    viewBox="0 0 24 24" 
                                    fill="none" 
                                    stroke="currentColor" 
                                    stroke-width="2"
                                >
                                    <line x1="12" y1="5" x2="12" y2="19" />
                                    <line x1="5" y1="12" x2="19" y2="12" />
                                </svg>
                            </button>
                            <button class="flex items-center justify-center w-12 h-12 rounded-full border-2 border-gray-400 hover:border-white transition">
                                <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M14 9V5a3 3 0 0 0-3-3l-4 9v11h11.28a2 2 0 0 0 2-1.7l1.38-9a2 2 0 0 0-2-2.3zM7 22H4a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h3" />
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            // Details Section
            <div class="px-12 py-12">
                <div class="max-w-7xl mx-auto">
                    <div class="grid grid-cols-3 gap-8 mb-16">
                        // Main Info
                        <div class="col-span-2">
                            <h2 class="text-2xl font-bold mb-4">"About this Book"</h2>
                            <p class="text-gray-300 leading-relaxed mb-6">
                                {book_description}
                            </p>
                            
                            <div class="space-y-3">
                                <div class="flex items-center gap-3">
                                    <svg class="w-5 h-5 text-gray-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
                                        <circle cx="12" cy="7" r="4" />
                                    </svg>
                                    <span class="text-gray-400">"Author:"</span>
                                    <span class="text-white">{book_author}</span>
                                </div>
                                <div class="flex items-center gap-3">
                                    <svg class="w-5 h-5 text-gray-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
                                        <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" />
                                    </svg>
                                    <span class="text-gray-400">"Publisher:"</span>
                                    <span class="text-white">{book_publisher}</span>
                                </div>
                                <div class="flex items-center gap-3">
                                    <svg class="w-5 h-5 text-gray-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <rect x="3" y="4" width="18" height="18" rx="2" ry="2" />
                                        <line x1="16" y1="2" x2="16" y2="6" />
                                        <line x1="8" y1="2" x2="8" y2="6" />
                                        <line x1="3" y1="10" x2="21" y2="10" />
                                    </svg>
                                    <span class="text-gray-400">"Publication Year:"</span>
                                    <span class="text-white">{book_year}</span>
                                </div>
                            </div>
                        </div>

                        // Side Info
                        <div>
                            <div class="bg-zinc-900 rounded p-6">
                                <h3 class="text-xl font-semibold mb-4">"Book Stats"</h3>
                                <div class="space-y-4">
                                    <div>
                                        <div class="text-gray-400 text-sm mb-1">"Rating"</div>
                                        <div class="flex items-center gap-2">
                                            <svg class="w-5 h-5 fill-yellow-400 text-yellow-400" viewBox="0 0 24 24" fill="currentColor">
                                                <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" />
                                            </svg>
                                            <span class="text-2xl font-bold">{book_rating}</span>
                                            <span class="text-gray-400">" / 5"</span>
                                        </div>
                                    </div>
                                    <div>
                                        <div class="text-gray-400 text-sm mb-1">"Reviews"</div>
                                        <div class="text-xl font-semibold">{format!("{}", book_reviews)}</div>
                                    </div>
                                    <div>
                                        <div class="text-gray-400 text-sm mb-1">"Language"</div>
                                        <div class="text-xl font-semibold">{book_language}</div>
                                    </div>
                                    <div>
                                        <div class="text-gray-400 text-sm mb-1">"Pages"</div>
                                        <div class="text-xl font-semibold">{book_pages}</div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Similar Books
                    <div>
                        <h2 class="text-2xl font-bold mb-6">"More Like This"</h2>
                        <div class="grid grid-cols-4 gap-4">
                            {similar_books.iter().clone().map(|(title, cover)| view! {
                                <div class="group cursor-pointer">
                                    <div class="relative overflow-hidden rounded aspect-[2/3] mb-2">
                                        <img 
                                            src={*cover}
                                            alt={*title}
                                            class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-300"
                                        />
                                        <div class="absolute inset-0 bg-black/0 group-hover:bg-black/40 transition-colors flex items-center justify-center">
                                            <svg class="w-12 h-12 opacity-0 group-hover:opacity-100 transition-opacity" viewBox="0 0 24 24" fill="currentColor">
                                                <polygon points="5 3 19 12 5 21 5 3" />
                                            </svg>
                                        </div>
                                    </div>
                                    <h3 class="text-sm font-semibold line-clamp-2">{*title}</h3>
                                </div>
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}