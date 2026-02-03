use crate::models::Books;
use leptos::{html::*, prelude::*};

#[allow(non_snake_case)]
#[component]
pub fn BookSections(title: String, book_data: Vec<Books>) -> impl IntoView {
    view! {
        <div class="mb-8 mx-8 md:mx-16">
            <h3 class="text-3xl font-bold mb-6 bg-gradient-to-r from-indigo-600 to-purple-600 bg-clip-text text-transparent">
                {title}
            </h3>
            <div class="flex space-x-6 overflow-x-visible overflow-y-scroll scroll-container pb-6 px-2">
                <For
                    each=move || book_data.clone()
                    key=|data| data.title.clone()
                    children=move |data| {
                        view! {
                            <div class="book-card w-44 flex-shrink-0 cursor-pointer group">
                                <div class="relative overflow-hidden rounded-xl">
                                    <img 
                                        loading="lazy" 
                                        src=format!("{}", data.image) 
                                        alt={data.title.clone()}
                                        class="w-full h-64 object-cover"
                                    />
                                    <div class="absolute inset-0 bg-gradient-to-t from-black/70 via-black/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-end p-4">
                                        <span class="text-white text-xs font-medium">Lihat Detail</span>
                                    </div>
                                </div>
                                <div class="mt-3 px-1">
                                    <h4 class="font-semibold text-sm text-gray-900 line-clamp-2 group-hover:text-indigo-600 transition-colors duration-300">
                                        {data.title}
                                    </h4>
                                    <p class="text-xs text-gray-500 mt-1 line-clamp-1">
                                        {data.creator}
                                    </p>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
