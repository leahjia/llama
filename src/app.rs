use leptos::*;
use leptos_meta::*;

mod components;
use components::chat::Chat;
use components::input::Input;

use crate::api::converse;
use crate::model::convo::{Conversation as Convo, Message as Msg};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    // read, write
    let (convo, set_convo) = create_signal(cx, Convo::new());
    let send = create_action(cx, move |new_msg: &String| {
        let user_msg = Msg { text: new_msg.clone(), user: true };
        set_convo.update(move |c| {
            c.messages.push(user_msg);
        });

        converse(cx, convo.get())
    });

    create_effect(cx, move |_| {
        if let Some(_) = send.input().get() {
            let model_msg = Msg {
                text: String::from("..."),
                user: false,
            };
            set_convo.update(move |c| {
                c.messages.push(model_msg);
            });
        }
    });

    create_effect(cx, move |_| {
        if let Some(Ok(res)) = send.value().get() {
            set_convo.update(move |c| {
                c.messages.last_mut().unwrap().text = res;
            });
        }
    });

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/llama.css"/>

        // sets the document title
        <Title text="Llama"/>
        <Chat convo/>
        <Input send/>
    }
}
