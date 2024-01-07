use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
    use leptos_use::{UseWebsocketReturn, use_websocket, core::ConnectionReadyState};

    use cfg_if::cfg_if;

    let mut hostname = "".to_string();
    cfg_if! {
        if #[cfg(feature="ssr")] {
            hostname = "ssr".to_string();
        } else {
            hostname = document().location().unwrap().host().unwrap();
        }
    }
    
    let protocol = if hostname.starts_with("localhost") { "ws" } else { "wss" };
    let url = format!("{}://{}/ws/", protocol, hostname);

    let UseWebsocketReturn {
        ready_state,
        message,
        message_bytes,
        send,
        send_bytes,
        open,
        close,
        ..
    } = use_websocket(&url);

    let (log, set_log) = create_signal("log\n---\n".to_string());

    let l = move |s| {
        set_log(format!("{}{}\n", log(), s));
    };
    l(format!("hostname found: {}", hostname));
    l(format!("connecting to {}", url));

    let send_message = move |_| {
        l("send_message".to_string());
        send("Hello, world!");
    };

    let send_byte_message = move |_| {
        l("send_byte_message()".to_string());
        send_bytes(b"Hello, world!\r\n".to_vec());
    };

    let status = Signal::derive(move || ready_state().to_string());

    message.with(move |msg| {
        l(format!("received message: {:?}\n", msg));
    });

    watch(
        move || message(),
        move |message, _, _| { l(format!("message: {:?}\n", message.clone().unwrap_or_default())); },
        false,
    );
    watch(
        move || message_bytes(),
        move |message_bytes, _, _| { l(format!("message: {:?}\n", message_bytes.clone().unwrap_or_default())); },
        false,
    );
    watch(
        move || ready_state(),
        move |ready_state, prev_ready_state, _| { l(format!("ready state changed from: {:?} to: {:?}\n", prev_ready_state, ready_state)); },
        false,
    );

    let connected = Signal::derive(move || ready_state() == ConnectionReadyState::Open);

    let open_connection = move|_| {
        l("open".to_string());
        open();
    };

    let close_connection = move |_| {
        l("close".to_string());
        close();
    };

    view! {
        <div>
            <p>"status: " {status}</p>

            <button on:click=send_message disabled=move || !connected()>"Send"</button>
            <button on:click=send_byte_message disabled=move || !connected()>"Send bytes"</button>:want
            <button on:click=open_connection disabled=connected>"Open"</button>
            <button on:click=close_connection disabled=move || !connected()>"Close"</button>

            <p>"Receive message: " {move || format!("{:?}", message())}</p>
            <p>"Receive byte message: " {move || format!("{:?}", message_bytes())}</p>

            <pre>{log}</pre>
        </div>
    }.into_view()
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
