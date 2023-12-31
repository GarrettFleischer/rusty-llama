use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::api::converse;

mod components;


use crate::model::conversation::{Conversation, Message};
use components::chat_area::ChatArea;
use components::type_area::TypeArea;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Rusty Llama"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=ChatPage/>
                    <Route path="/*any" view=NotFoundPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn ChatPage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(cx, 0);
    // let on_click = move |_| set_count.update(|count| *count += 1);

    let (conversation, set_conversation) = create_signal(cx, Conversation::new());

    let send = create_action(cx, move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };

        set_conversation.update(move |c| c.messages.push(user_message));
        
        converse(cx, conversation.get())
    });

    create_effect(cx, move |_| {
       if let Some(_) = send.input().get() {
           let model_message = Message {
               user: false,
               text: String::from("...")
           };
           
           set_conversation.update(move |c| {
               c.messages.push(model_message);
           })
       } 
    });
    
    create_effect(cx, move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            });
        }
    });
    
    view! { cx,
        <ChatArea conversation/>
        <TypeArea send/>
        // <h1>"Welcome to Rusty Llama!"</h1>
        // <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFoundPage(cx: Scope) -> impl IntoView {
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
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}
