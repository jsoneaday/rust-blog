use leptos::*;
use leptos::html::Div;
use leptos_use::on_click_outside;

#[component]
pub fn Modal(children: ChildrenFn, disable_dismiss: bool, open_state: ReadSignal<bool>, set_open_state: WriteSignal<bool>) -> impl IntoView {
    let container_ref = create_node_ref::<Div>();
    let children = store_value(children);

    if !disable_dismiss {
        let dismiss_modal_with_keyboard = window_event_listener(ev::keydown, move |ev| {
            if ev.key() == "Escape" || ev.key() == "q" || ev.key() == "Q" {
                set_open_state(false);
            }
        });
        on_cleanup(move || dismiss_modal_with_keyboard.remove());

        on_cleanup(on_click_outside(container_ref, move |_| {
            set_open_state(false);
        }));
    }

    view! {
        <Show when=open_state fallback=|| ()>
            <Portal mount=document().get_element_by_id("portal_root").unwrap()>
                <div class="modal-container">
                    <div node_ref=container_ref>
                        <dialog
                            open=open_state
                            class="modal-content"
                        >
                            {children.with_value(|children| children())}
                        </dialog>
                    </div>
                </div>
            </Portal>
        </Show>
    }
}