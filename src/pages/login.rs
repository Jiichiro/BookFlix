use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::Redirect;

use crate::server::auth::Login;

#[component]
pub fn Login() -> impl IntoView {
    let submit = ServerAction::<Login>::new();

    view! {
        <Title text="Masuk - BookFlix"/>
        <div class="min-h-screen bg-black relative overflow-hidden">
          <div
            class="absolute inset-0 bg-cover bg-center"
            // style:background-image="url(https://assets.nflxext.com/ffe/siteui/vlv3/b2c3e95b-b7b5-4bb7-a883-f4bfc7472fb7/19fc1a4c-82db-4481-ad08-3a1dffbb8c39/ID-en-20240805-POP_SIGNUP_TWO_WEEKS-perspective_WEB_24a485f6-1820-42be-9b60-1b066f1eb869_large.jpg)"
          >
            <div class="absolute inset-0 bg-black bg-opacity-60"></div>
          </div>

          // Netflix Logo
          <div class="relative z-10 px-8 py-6">
            <h1 class="text-3xl font-bold text-red-600">BookFlix</h1>
          </div>

          // Login Form
          <div class="relative z-10 flex items-center justify-center min-h-[calc(100vh-100px)] px-4">
            <div class="w-full max-w-md bg-black bg-opacity-75 rounded-lg p-16">
              <h1 class="text-white text-3xl font-semibold mb-8">"Masuk"</h1>

              <ActionForm action=submit attr:class="space-y-5">
                { move || {
                    submit.value().get().map(|result| {
                        match result {
                          Err(e) => view! { <p class="text-red-500 text-sm">{e.to_string()}</p> }.into_any(),
                          Ok(_) => view! { <Redirect path="/dash" /> }.into_any()
                        }
                    })
                }}

                <div>
                  <input
                    name="username"
                    type="text"
                    placeholder="Username"
                    class="w-full px-5 py-4 bg-zinc-800 text-white rounded border border-zinc-700 focus:outline-none focus:border-white placeholder-zinc-400"
                  />
                </div>

                <div>
                  <input
                    name="password"
                    type="password"
                    placeholder="Password"
                    class="w-full px-5 py-4 bg-zinc-800 text-white rounded border border-zinc-700 focus:outline-none focus:border-white placeholder-zinc-400"
                  />
                </div>

                <button
                  type="submit"
                  class="w-full bg-red-600 text-white font-semibold py-3 rounded hover:bg-red-700 transition duration-200"
                >
                  "Masuk"
                </button>

                <div class="flex items-center justify-between text-sm">
                  <label class="flex items-center text-zinc-400">
                    <input type="checkbox" class="mr-2" />
                    "Ingat saya"
                  </label>
                  <a href="#" class="text-zinc-400 hover:underline">
                    "Butuh bantuan?"
                  </a>
                </div>
              </ActionForm>

              <div class="mt-16 text-zinc-400 text-sm">
                <p>
                  "Baru di BookFlix? "
                  <a href="#" class="text-white hover:underline">
                    "Daftar sekarang"
                  </a>
                  "."
                </p>
                <p class="mt-3 text-xs">
                  "Halaman ini dilindungi oleh Google reCAPTCHA untuk memastikan kamu bukan bot."
                </p>
              </div>
            </div>
          </div>
        </div>
    }
}
