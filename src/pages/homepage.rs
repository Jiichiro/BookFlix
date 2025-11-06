use leptos::{ html::*, prelude::*};
use leptos_meta::Title;

// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <Title text="Home Page"/>
        <div class="relative h-screen text-white">
            <div class="absolute inset-0 bg-gradient-to-r from-gray-900 via-gray-900/70 to-transparent z-10"></div>
            <img src="https://images.unsplash.com/photo-1481627834876-b7833e8f5570?w=1600" alt="Hero Book" class="w-full h-full object-cover"/>
            <div class="absolute inset-0 z-20 flex items-center px-8 md:px-16">
                <div class="max-w-2xl">
                    <h2 class="text-5xl md:text-7xl font-bold mb-4">The Great Gatsby</h2>
                    <p class="text-lg md:text-xl mb-6">Kisah klasik tentang kemewahan, cinta, dan obsesi di era Jazz Age Amerika tahun 1920-an. Ikuti perjalanan Jay Gatsby yang misterius dalam mengejar mimpi dan cinta sejatinya.</p>
                    <div class="flex items-center space-x-4 mb-6">
                        <span class="text-green-500 font-semibold">95% Rating</span>
                        <span>2023</span>
                        <span class="border border-gray-400 px-2 py-1 text-sm">Fiksi Klasik</span>
                    </div>
                    <div class="flex space-x-4">
                        <button class="bg-white text-black px-8 py-3 rounded font-semibold hover:bg-gray-200 transition flex items-center space-x-2">
                            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
                                <path d="M10 12a2 2 0 100-4 2 2 0 000 4z"></path>
                                <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd"></path>
                            </svg>
                            <span>Baca Sekarang</span>
                        </button>
                        <button class="bg-gray-700 bg-opacity-70 px-8 py-3 rounded font-semibold hover:bg-opacity-50 transition flex items-center space-x-2">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                            <span>Info Lengkap</span>
                        </button>
                    </div>
                </div>
            </div>
        </div>

        <div class="relative z-30 -mt-32 px-8 md:px-16 space-y-12 pb-16">
            <div>
                <h3 class="text-2xl font-bold mb-4">Sedang Trending</h3>
                <div class="flex space-x-4 overflow-x-auto scroll-container pb-4">
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1544947950-fa07a98d237f?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">Atomic Habits</h4>
                        <p class="text-xs text-gray-400">James Clear</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1512820790803-83ca734da794?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">The Midnight Library</h4>
                        <p class="text-xs text-gray-400">Matt Haig</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1543002588-bfa74002ed7e?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">Educated</h4>
                        <p class="text-xs text-gray-400">Tara Westover</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1495640388908-05fa85288e61?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">Sapiens</h4>
                        <p class="text-xs text-gray-400">Yuval Noah Harari</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1589998059171-988d887df646?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">The Silent Patient</h4>
                        <p class="text-xs text-gray-400">Alex Michaelides</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1457369804613-52c61a468e7d?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">Becoming</h4>
                        <p class="text-xs text-gray-400">Michelle Obama</p>
                    </div>
                </div>
            </div>

            <div>
                <h3 class="text-2xl font-bold mb-4">Best Seller Bulan Ini</h3>
                <div class="flex space-x-4 overflow-x-auto scroll-container pb-4">
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1519682337058-a94d519337bc?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">Where the Crawdads Sing</h4>
                        <p class="text-xs text-gray-400">Delia Owens</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">The Seven Husbands</h4>
                        <p class="text-xs text-gray-400">Taylor Jenkins Reid</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1532012197267-da84d127e765?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">Project Hail Mary</h4>
                        <p class="text-xs text-gray-400">Andy Weir</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1519331379826-f10be5486c6f?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">The Invisible Life</h4>
                        <p class="text-xs text-gray-400">V.E. Schwab</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1558618666-fcd25c85cd64?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">Circe</h4>
                        <p class="text-xs text-gray-400">Madeline Miller</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1506880018603-83d5b814b5a6?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">The Nightingale</h4>
                        <p class="text-xs text-gray-400">Kristin Hannah</p>
                    </div>
                </div>
            </div>

            <div>
                <h3 class="text-2xl font-bold mb-4">Fiksi Pilihan</h3>
                <div class="flex space-x-4 overflow-x-auto scroll-container pb-4">
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1544716278-ca5e3f4abd8c?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">1984</h4>
                        <p class="text-xs text-gray-400">George Orwell</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1516979187457-637abb4f9353?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">To Kill a Mockingbird</h4>
                        <p class="text-xs text-gray-400">Harper Lee</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1541963463532-d68292c34b19?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">Pride and Prejudice</h4>
                        <p class="text-xs text-gray-400">Jane Austen</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1524995997946-a1c2e315a42f?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">The Alchemist</h4>
                        <p class="text-xs text-gray-400">Paulo Coelho</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1553729459-efe14ef6055d?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">The Kite Runner</h4>
                        <p class="text-xs text-gray-400">Khaled Hosseini</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1550399105-c4db5fb85c18?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">The Book Thief</h4>
                        <p class="text-xs text-gray-400">Markus Zusak</p>
                    </div>
                </div>
            </div>

            <div>
                <h3 class="text-2xl font-bold mb-4">Non-Fiksi Inspiratif</h3>
                <div class="flex space-x-4 overflow-x-auto scroll-container pb-4">
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1522407183863-c0bf2256188f?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">Thinking, Fast and Slow</h4>
                        <p class="text-xs text-gray-400">Daniel Kahneman</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1531346878377-a5be20888e57?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">The Power of Habit</h4>
                        <p class="text-xs text-gray-400">Charles Duhigg</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1491841573634-28140fc7ced7?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">Outliers</h4>
                        <p class="text-xs text-gray-400">Malcolm Gladwell</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1521587760476-6c12a4b040da?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">Man&#39;s Search for Meaning</h4>
                        <p class="text-xs text-gray-400">Viktor E. Frankl</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1485988412941-77a35537dae4?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">The Lean Startup</h4>
                        <p class="text-xs text-gray-400">Eric Ries</p>
                    </div>
                    <div class="book-card flex-shrink-0 w-40 cursor-pointer">
                        <img src="https://images.unsplash.com/photo-1537498425277-c283d32ef9db?w=400" alt="Book" class="w-full h-60 object-cover rounded shadow-lg"/>
                        <h4 class="mt-2 font-semibold text-sm">Grit</h4>
                        <p class="text-xs text-gray-400">Angela Duckworth</p>
                    </div>
                </div>
            </div>
        </div>

    }
}