use sycamore::prelude::*;

#[derive(PartialEq)]
enum Mode {
    Answer,
    Question,
}

impl Mode {
    fn title(&self) -> String {
        let title = match self {
            Mode::Answer => "Следующий вопрос",
            Mode::Question => "Посмотреть ответ",
        };
        format!("{}", title)
    }
}

struct AppState {
    question: String,
    answer: String,
    mode: Mode,
}

#[component]
pub fn App<G: Html>(ctx: ScopeRef) -> View<G> {
    let (question, answer) = crate::exercises::random_exercise();
    let initial_state = AppState {
        question,
        answer,
        mode: Mode::Question,
    };
    let state = ctx.create_signal(initial_state);

    let handle_click = |_e| {
        if state.get().mode.eq(&Mode::Question) {
            state.set(AppState {
                mode: Mode::Answer,
                question: state.get().question.clone(),
                answer: state.get().answer.clone(),
            });
        } else {
            let (question, answer) = crate::exercises::random_exercise();
            state.set(AppState {
                mode: Mode::Question,
                question,
                answer,
            });
        };
    };

    view! { ctx,
        div(class="wrap") {
            div(class="question") {
                div(class="title") { "Вопрос" }
                (state.get().question)
            }
            div(class="answer") {
                (if state.get().mode.eq(&Mode::Question) {
                    View::empty()
                } else {
                    view! { ctx,
                        div(class="title") { "Ответ" }
                        div(class="hello", on:key_press=handle_click) { (state.get().answer)}
                    }
                })
            }
            button(on:click=handle_click) { (state.get().mode.title()) }
        }
    }
}
