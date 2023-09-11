use leptos::{html::Input, *};

#[component]
pub fn Input(cx: Scope, send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    view! { cx,
      <div>
        <form on:submit={move |e| {
          e.prevent_default();
          let input = input_ref.get().expect("input to exist");
          send.dispatch(input.value());
          input.set_value("");
        }}>
          <input type="text" />
          <button type="submit">Send</button>
        </form>
      </div>
    }
}
