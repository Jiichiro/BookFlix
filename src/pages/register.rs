use crate::server::auth::Register;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{LazyRoute, lazy_route, components::A};

pub struct RegisterPage {
    submit: ServerAction<Register>,
}

#[lazy_route]
impl LazyRoute for RegisterPage {
    fn data() -> Self {
        let submit = ServerAction::<Register>::new();
        RegisterPage { submit }
    }

    fn view(this: Self) -> AnyView {
        let RegisterPage { submit } = this;
        let (is_success, set_is_success) = signal(false);
        view! {
            <Title text="Daftar - BookFlix"/>
            <div class="min-h-screen bg-black relative overflow-hidden">

              <div class="relative z-10 px-8 py-6">
                <h1 class="text-3xl font-bold text-red-600">BookFlix</h1>
              </div>

              <div class="relative z-10 flex items-center justify-center min-h-[calc(100vh-100px)] px-4">
                <div class="w-full max-w-md bg-black bg-opacity-75 rounded-lg p-16">
                  <h1 class="text-white text-3xl font-semibold mb-8">"Daftar"</h1>

                  <ActionForm action=submit attr:class="space-y-5">
                    { move || {
                        submit.value().get().map(|result| {
                            match result {
                              Err(e) => view! { <p class="text-red-500 text-sm">{e.to_string().replace("error running server function: ", "")}</p> }.into_any(),
                              Ok(msg) => {
                                set_is_success.set(true);
                                view! { <p class="text-green-500 text-xl">{msg}</p> }.into_any()
                              }
                            }
                        })
                    }}

                    <Show
                      when=move || {!is_success.get()}
                      fallback=|| view! {
                        <A
                          href="/login"
                          attr:class=move || "block w-full bg-red-600 text-white font-semibold py-3 rounded hover:bg-red-700 transition duration-200 text-center"
                        >
                          "Back to Login"
                        </A>
                      }
                      >
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
                            name="email"
                            type="email"
                            placeholder="Email"
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

                        <div>
                          <input
                            name="confirm_password"
                            type="password"
                            placeholder="Konfirmasi Password"
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
                            "Daftar"
                          }}
                        </button>

                        <div class="text-xs text-zinc-400">
                          <label class="flex items-start">
                            <input type="checkbox" required class="mr-2 mt-1" />
                            <span>
                              "Saya setuju dengan "
                              <A href="#" attr:class="text-white hover:underline">"Syarat dan Ketentuan"</A>
                              " serta "
                              <A href="#" attr:class="text-white hover:underline">"Kebijakan Privasi"</A>
                              " BookFlix"
                            </span>
                          </label>
                        </div>
                      </Show>

                    
                  </ActionForm>

                  <div class="mt-16 text-zinc-400 text-sm">
                    <p>
                      "Sudah punya akun? "
                      <A href="/login" attr:class="text-white hover:underline">
                        "Masuk sekarang"
                      </A>
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
