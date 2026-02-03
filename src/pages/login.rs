use crate::server::auth::Login;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{components::Redirect, lazy_route, LazyRoute};

pub struct LoginPage {
    submit: ServerAction<Login>,
}

#[lazy_route]
impl LazyRoute for LoginPage {
    fn data() -> Self {
        let submit = ServerAction::<Login>::new();
        LoginPage { submit }
    }

    fn view(this: Self) -> AnyView {
        let LoginPage { submit } = this;
        view! {
            <Title text="Masuk - BookFlix"/>
            <div class="min-h-screen bg-black relative overflow-hidden">

              <div class="relative z-10 px-8 py-6">
                <h1 class="text-3xl font-bold text-red-600">BookFlix</h1>
              </div>

              <div class="relative z-10 flex items-center justify-center min-h-[calc(100vh-100px)] px-4">
                <div class="w-full max-w-md bg-black bg-opacity-75 rounded-lg p-16">
                  <h1 class="text-white text-3xl font-semibold mb-8">"Masuk"</h1>

                  <ActionForm action=submit attr:class="space-y-5">
                    { move || {
                        submit.value().get().map(|result| {
                            match result {
                              Err(e) => view! { <p class="text-red-500 text-sm">{e.to_string().replace("error running server function: ", "")}</p> }.into_any(),
                              Ok(_) => view! { <Redirect path="/member" /> }.into_any()
                            }
                        })
                    }}

                    <div>
                      <input
                        name="username"
                        type="text"
                        placeholder="Username"
                        required
                        class="w-full px-5 py-4 bg-zinc-800 text-white rounded border border-zinc-700 focus:outline-none focus:border-white placeholder-zinc-400"
                      />
                    </div>

                    <div>
                      <input
                        name="password"
                        type="password"
                        placeholder="Password"
                        required
                        class="w-full px-5 py-4 bg-zinc-800 text-white rounded border border-zinc-700 focus:outline-none focus:border-white placeholder-zinc-400"
                      />
                    </div>

                    <button
                      type="submit"
                      class="w-full bg-red-600 text-white font-semibold py-3 rounded hover:bg-red-700 transition duration-200"
                    >
                      {move || if submit.pending().get() {
                        "Memproses..."
                      } else {
                        "Masuk"
                      }}
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
                      <a href="/register" class="text-white hover:underline">
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
        }.into_any()
    }
}
