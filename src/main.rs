use std::time::Duration;

use leptos::*;

fn main() {
    mount_to_body(|| view! { <List /> })
}

#[component]
pub fn List() -> impl IntoView {
    let (raw_items, set_raw_items) = create_signal(vec!["One".to_string()]);
    let items = create_resource(
        move || raw_items.get(),
        |raw_items| async move { raw_items },
    );

    create_effect(move |_| {
        set_timeout(
            move || {
                set_raw_items.update(|raw_items| raw_items.push("Two".to_string()));
            },
            Duration::from_secs(1),
        );
    });

    view! {
        <Transition fallback=move || ()>
            {move || {
                items.read()
                    .map(|items| {
                        view! {
                            <For
                                each=move || items.clone()
                                key=|item| item.clone()
                                view=move |item| {
                                    view! {
                                        <Row item=item />
                                    }
                                }
                            />
                        }
                    })
            }}
        </Transition>
    }
}

#[component]
pub fn Row(item: String) -> impl IntoView {
    let (my_signal, _set_my_signal) = create_signal(item);
    on_cleanup(move || {
        leptos::log!("{}", my_signal.get()); // Fix is to use .get_untracked()
    });

    view! {
        <span>{my_signal}</span>
    }
}
