use crate::cfg_if;
use crate::model::convo::Conversation as Convo;
use leptos::{server, Scope, ServerFnError};

#[server(Converse "/api")]
pub async fn converse(cx: Scope, prompt: Convo) -> Result<String, ServerFnError> {
    use actix_web::dev::ConnectionInfo;
    use actix_web::web::Data;
    use leptos_actix::extract;
    use llm::models::Llama;

    let model = extract(cx, |data: Data<Llama>, _connection: ConnectionInfo| async { data.into_inner() }).await.unwrap();

    use llm::KnownModel;
    // prefix prompt
    let char_name = "### Assistant";
    let user_name = "### User";
    let persona = "chat between user and assistant";
    let mut history = format!(
        "{char_name}:Hi, How can I help you?\n\
      {user_name}:What is the largest planet in our solar system?\n\
      {char_name}:Jupiter is the largest planet in our solar system.\n",
    );

    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let curr_line = if message.user { format!("{char_name}:{msg}\n") } else { format!("{user_name}:{msg}\n") };
        history.push_str(&curr_line);
    }

    let mut res = String::new();
    let mut random = rand::thread_rng();
    let mut buf = String::new();

    let mut session = model.start_session(Default::default());
    session
        .infer(
            model.as_ref(),
            &mut random,
            &llm::InferenceRequest {
                prompt: format!("{persona}\n{history}\n{char_name}:").as_str().into(),
                parameters: &llm::InferenceParameters::default(),
                play_back_previous_tokens: false,
                maximum_token_count: None,
            },
            &mut Default::default(),
            inference_callback(String::from(user_name), &mut buf, &mut res),
        )
        .unwrap_or_else(|err| panic!("{err}"));

    Ok(res)
}

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use std::convert::Infallible;
    fn inference_callback<'a>(
      stop_sequence: String, buf: &'a mut String,
      out_str: &'a mut String,
    ) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
      use llm::InferenceFeedback::Halt;
      use llm::InferenceFeedback::Continue;

      move |res| match res {
        llm::InferenceResponse::InferredToken(t) => {
          let mut reverse_buf = buf.clone();
          reverse_buf.push_str(t.as_str());
          if stop_sequence.as_str().eq(reverse_buf.as_str()) {
            buf.clear();
            return Ok::<llm::InferenceFeedback, Infallible>(Halt);
          } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
            buf.push_str(t.as_str());
            return Ok(Continue);
          }
          if buf.is_empty() {
            buf.push_str(t.as_str());
          } else {
            buf.push_str(&reverse_buf);
          }
          Ok(Continue)
        }
        llm::InferenceResponse::EotToken => Ok(Halt),
        _ => Ok(Continue),
      }
    }
  }
}
