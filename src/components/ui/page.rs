use super::{Header, Sidenav};
use leptos::*;

#[component]
pub fn Page(children: Children, use_layout: bool) -> impl IntoView {
    if use_layout {
        view! {
            <>
                <html lang="en" data-theme="app">
                    <head>
                        <meta name="viewport" content="width=device-width, initial-scale=1" />
                        <meta charset="UTF-8" />
                        <title>rust-htmx</title>
                        <link rel="stylesheet" href="https://unpkg.com/minireset.css@0.0.7/minireset.css" />
                        <link rel="stylesheet" href="https://unpkg.com/normalize.css@7.0.0/normalize.css" />
                        <link rel="stylesheet" href="/assets/css/main.min.css" />
                        <link rel="preconnect" href="https://fonts.googleapis.com" />
                        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                        <link href="https://fonts.googleapis.com/css2?family=Inter:wght@100..900&display=swap" rel="stylesheet" />
                        <script src="//unpkg.com/alpinejs" defer></script>                       <script src="https://unpkg.com/htmx.org@1.9.10"></script>
                    </head>
                    <body>
                        <Header />
                        <div class="pt-16 h-screen">
                            <Sidenav />
                            <main id="main" class="ml-48">
                                {children()}
                            </main>
                        </div>
                    </body>
                </html>
            </>
        }
    } else {
        view! {
            <>
               {children()}
            </>
        }
    }
}
