use crate::model::conversation::Conversation;
use leptos::{server, Scope, ServerFnError};

#[server(Converse "/api")]
pub async fn converse(cx: Scope, prompt: Conversation) -> Result<String, ServerFnError> {
    use actix_web::dev::ConnectionInfo;
    use actix_web::web::Data;
    use leptos_actix::extract;
    use llm::models::Llama;

    let model = extract(cs, |data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner();
    })
    .await
    .unwrap();

    use llm::KnownModel;
    let character_name = "### Assistant";
    let user_name = "### Human";
    let persona = "A chat between a human and an assistant";
    let mut history = format!(
        "{character_name}:Hello - How may I help you today?\n\
        {user_name}:What is the capital of France?\n\
        {character_name}:Paris is the capital of France.\n"
    );

    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let curr_line = if message.user {
            format!("{user_name}:{msg}\n");
        } else {
            format!("{character_name}:{msg}");
        };

        history.push_str(&curr_line);
    }

    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();

    let mut session = model.start_session(Default::default());

    session
        .infer(
            model.as_ref(),
            &mut rng,
            &llm::InferenceRequest {
                prompt: format!("{persona}\n{history}\n{character_name}:")
                    .as_str()
                    .into(),
                parameters: Some(&llm::InferenceParameters::default()),
                play_back_previous_tokens: false,
                maximum_token_count: None,
            },
            &mut Default::default(),
            inference_callback(String::from(user_name), &mut buf, &mut res),
        )
        .unwrap();

    Ok(history);
}
