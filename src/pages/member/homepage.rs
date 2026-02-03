use crate::components::BookSections;
use crate::models::Books;
use leptos::{html::*, prelude::*};
use leptos_meta::Title;

#[derive(Clone)]
struct BooksArgs {
    title: String,
    books: Vec<Books>,
}

// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let token_store = crate::utils::auth_token_manager::use_token_store();

    Effect::new(move |_| {
        if token_store.has_token() {
            println!("✓ Token exists, fetching books...");
            // ✅ TRIGGER SERVER FUNCTION
            // Token otomatis dikirim di header!
            // fetch_books.dispatch(());
            println!("✓ Fetching books from server...");
        } else {
            println!("❌ No token, redirecting to login...");
            let navigate = leptos_router::hooks::use_navigate();
            navigate("/login", Default::default());
        }
    });

    let book_list = vec![
        BooksArgs {
            title: "Sedang Trending".to_string(),
            books: vec![
                Books {
                    title: "Atomic Habits".to_string(),
                    creator: "James Clear".to_string(),
                    image: "https://images.unsplash.com/photo-1544947950-fa07a98d237f?w=400"
                        .to_string(),
                },
                Books {
                    creator: "Matt Haig".to_string(),
                    image: "https://images.unsplash.com/photo-1512820790803-83ca734da794?w=400"
                        .to_string(),
                    title: "The Midnight Library".to_string(),
                },
                Books {
                    creator: "Tara Westover".to_string(),
                    image: "https://images.unsplash.com/photo-1543002588-bfa74002ed7e?w=400"
                        .to_string(),
                    title: "Educated".to_string(),
                },
                Books {
                    creator: "Yuval Noah Harari".to_string(),
                    image: "https://images.unsplash.com/photo-1495640388908-05fa85288e61?w=400"
                        .to_string(),
                    title: "Sapiens".to_string(),
                },
                Books {
                    creator: "Alex Michaelides".to_string(),
                    image: "https://images.unsplash.com/photo-1589998059171-988d887df646?w=400"
                        .to_string(),
                    title: "The Silent Patient".to_string(),
                },
                Books {
                    creator: "Michelle Obama".to_string(),
                    image: "https://images.unsplash.com/photo-1457369804613-52c61a468e7d?w=400"
                        .to_string(),
                    title: "Becoming".to_string(),
                },
            ],
        },
        BooksArgs {
            title: "Best Seller Bulan Ini".to_string(),
            books: vec![
                Books {
                    title: "Where the Crawdads Sing".to_string(),
                    creator: "Delia Owels".to_string(),
                    image: "https://images.unsplash.com/photo-1519682337058-a94d519337bc?w=400"
                        .to_string(),
                },
                Books {
                    title: "The Seven Husbands".to_string(),
                    creator: "Taylor Jenkins Reid".to_string(),
                    image: "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400"
                        .to_string(),
                },
                Books {
                    title: "Project Hail Mary".to_string(),
                    creator: "Andy Weir".to_string(),
                    image: "https://images.unsplash.com/photo-1532012197267-da84d127e765?w=400"
                        .to_string(),
                },
                Books {
                    title: "The Invisible Life".to_string(),
                    creator: "V.E. Schwab".to_string(),
                    image: "https://images.unsplash.com/photo-1519331379826-f10be5486c6f?w=400"
                        .to_string(),
                },
                Books {
                    title: "Circe".to_string(),
                    creator: "Madeline Miller".to_string(),
                    image: "https://images.unsplash.com/photo-1558618666-fcd25c85cd64?w=400"
                        .to_string(),
                },
                Books {
                    title: "The Nightingale".to_string(),
                    creator: "Kristin Hannah".to_string(),
                    image: "https://images.unsplash.com/photo-1506880018603-83d5b814b5a6?w=400"
                        .to_string(),
                },
            ],
        },
        BooksArgs {
            title: "Fiksi Pilihan".to_string(),
            books: vec![
                Books {
                    title: "1984".to_string(),
                    creator: "George Orwell".to_string(),
                    image: "https://images.unsplash.com/photo-1544716278-ca5e3f4abd8c?w=400"
                        .to_string(),
                },
                Books {
                    title: "To Kill a Mockingbird".to_string(),
                    creator: "Harper Lee".to_string(),
                    image: "https://images.unsplash.com/photo-1516979187457-637abb4f9353?w=400"
                        .to_string(),
                },
                Books {
                    title: "Pride and Prejudice".to_string(),
                    creator: "Jane Austen".to_string(),
                    image: "https://images.unsplash.com/photo-1541963463532-d68292c34b19?w=400"
                        .to_string(),
                },
                Books {
                    title: "The Alchemist".to_string(),
                    creator: "Paulo Coelho".to_string(),
                    image: "https://images.unsplash.com/photo-1524995997946-a1c2e315a42f?w=400"
                        .to_string(),
                },
                Books {
                    title: "The Kite Runner".to_string(),
                    creator: "Khaled Hosseini".to_string(),
                    image: "https://images.unsplash.com/photo-1553729459-efe14ef6055d?w=400"
                        .to_string(),
                },
                Books {
                    title: "The Book Thief".to_string(),
                    creator: "Markus Zusak".to_string(),
                    image: "https://images.unsplash.com/photo-1550399105-c4db5fb85c18?w=400"
                        .to_string(),
                },
            ],
        },
        BooksArgs {
            title: "Non-Fiksi Inspiratif".to_string(),
            books: vec![
                Books {
                    title: "Thinking, Fast and Slow".to_string(),
                    creator: "Daniel Kahneman".to_string(),
                    image: "https://images.unsplash.com/photo-1565128555334-7fa709893f16?w=400"
                        .to_string(),
                },
                Books {
                    title: "The Power of Habit".to_string(),
                    creator: "Charles Duhigg".to_string(),
                    image: "https://images.unsplash.com/photo-1531346878377-a5be20888e57?w=400"
                        .to_string(),
                },
                Books {
                    title: "Outliers".to_string(),
                    creator: "Malcolm Gladwell".to_string(),
                    image: "https://images.unsplash.com/photo-1491841573634-28140fc7ced7?w=400"
                        .to_string(),
                },
                Books {
                    title: "Man's Search for Meaning".to_string(),
                    creator: "Viktor E. Frankl".to_string(),
                    image: "https://images.unsplash.com/photo-1521587760476-6c12a4b040da?w=400"
                        .to_string(),
                },
                Books {
                    title: "The Lean Startup".to_string(),
                    creator: "Eric Ries".to_string(),
                    image: "https://images.unsplash.com/photo-1485988412941-77a35537dae4?w=400"
                        .to_string(),
                },
                Books {
                    title: "Grit".to_string(),
                    creator: "Angela Duckworth".to_string(),
                    image: "https://images.unsplash.com/photo-1537498425277-c283d32ef9db?w=400"
                        .to_string(),
                },
            ],
        },
    ];

    view! {
            <Title text="BookFlix"/>
            <div class="relative h-screen text-white">
                <div class="absolute w-40 inset-0 bg-r from-gray-900 via-gray-900/70 to-transparent z-10"></div>
                <img loading="lazy" src="https://images.unsplash.com/photo-1481627834876-b7833e8f5570?w=1600" alt="Hero Book" class="w-full h-full object-cover"/>
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
                            <button class="bg-white text-black px-8 py-3 rounded font-semibold hover:bg-gray-200 transition flex items-center space-x-2 cur">
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

            <div class="relative z-30 -mt-32 space-y-12 pb-16">
                <For
                    each=move || book_list.clone()
                    key=|book| book.title.clone()
                    children=move |book| {
                        view! {
                            <BookSections title=book.title book_data=book.books/>
                        }
                    }
                />
            </div>
        }
}
