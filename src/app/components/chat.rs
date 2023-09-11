use crate::model::convo::Conversation;
use leptos::*;
const USER_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-end";
const MODEL_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-start";

#[component]
pub fn Chat(cx: Scope, convo: ReadSignal<Conversation>) -> impl IntoView {
    view! { cx,
    <div >
      {move || convo.get().messages.iter().map(move |message| {
        let class_str = if message.user {USER_MESSAGE_CLASS} else {MODEL_MESSAGE_CLASS};
        view! { cx,
          <div class = {class_str}>
            {message.text.clone()}
          </div>
        }
      }).collect::<Vec<_>>()}
    </div>
    }
}
