use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::core::ConnectionReadyState;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    use leptos_use::*;

    let UseWebsocketReturn {
        ready_state,
        message,
        message_bytes,
        send,
        send_bytes,
        open,
        close,
        ..
    } = use_websocket("wss://localhost:3000/ws/");

    let (log, set_log) = create_signal("log\n---\n".to_string());

    let send_message = move |_| {
        set_log(log() + "send_message()\n");
        send("Hello, world!");
    };

    let send_byte_message = move |_| {
        set_log(log() + "send_byte_message()\n");
        send_bytes(b"Hello, world!\r\n".to_vec());
    };

    let status = move || ready_state().to_string();
    ready_state.with(|status| {
        set_log(log() + format!("ready state changed to: {:?}\n", status).as_str())
    });

    let connected = move || ready_state() == ConnectionReadyState::Open;

    let open_connection = move|_| {
        set_log(log() + "open\n");
        open();
    };

    let close_connection = move |_| {
        set_log(log() + "close\n");
        close();
    };

    view! {
        <div>
            <pre>{log}</pre>
            <p>"status: " {status}</p>

            <button on:click=send_message disabled=move || !connected()>"Send"</button>
            <button on:click=send_byte_message disabled=move || !connected()>"Send bytes"</button>:want
            <button on:click=open_connection disabled=connected>"Open"</button>
            <button on:click=close_connection disabled=move || !connected()>"Close"</button>

            <p>"Receive message: " {move || format!("{:?}", message())}</p>
            <p>"Receive byte message: " {move || format!("{:?}", message_bytes())}</p>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
