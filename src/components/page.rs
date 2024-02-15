use super::Sidenav;
use leptos::*;

#[component]
pub fn Page(children: Children, use_layout: bool) -> impl IntoView {
    if use_layout {
        view! {
            <>
                <html lang="en">
                    <head>
                        <meta name="viewport" content="width=device-width, initial-scale=1" />
                        <meta charset="UTF-8" />
                        <title>rust-htmx</title>
                        <link rel="stylesheet" href="https://unpkg.com/minireset.css@0.0.7/minireset.css" />
                        <link rel="stylesheet" href="https://unpkg.com/normalize.css@7.0.0/normalize.css" />
                        <link rel="stylesheet" href="/assets/css/main.min.css" />
                        <script src="https://unpkg.com/hyperscript.org@0.9.12"></script>
                        <script src="https://unpkg.com/htmx.org@1.9.10"></script>
                    </head>
                    <body>
                        <Sidenav />
                        <main id="main" class="h-screen ml-48" style="width: calc(100vw - 12rem)">
                            {children()}
                        </main>
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
