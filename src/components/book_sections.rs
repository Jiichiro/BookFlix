use leptos::{*, html::*, prelude::*};
use crate::models::Books;

#[allow(non_snake_case)]
#[component]
pub fn BookSections(title: String, book_data: Vec<Books>) -> impl IntoView {
    view! {
        <h3 class="text-2xl font-bold mb-4 mix-blend-difference">{title}</h3>
        <div class="flex space-x-4 overflow-visible scroll-container pb-4">
            <For
                each=move || book_data.clone()
                key=|data| data.title.clone()
                children=move |data| {
                    view! {
                        <div class="book-card w-40 flex-shrink-0 cursor-pointer">
                            <img 
                                loading="lazy" 
                                src=format!("{}", data.image) 
                                alt="Book" 
                                class="w-full h-60 object-cover rounded shadow-lg"
                            />
                            <h4 class="mt-2 font-semibold text-sm">{data.title}</h4>
                            <p class="text-xs text-gray-400">{data.creator}</p>
                        </div>
                    }
                }
            />
        </div>
    }
}
