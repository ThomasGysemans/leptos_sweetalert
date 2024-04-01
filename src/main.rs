use leptos_sweetalert::*;
use leptos::*;
use log::info;

pub fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    Swal::init_escape_key_handler();
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    let success = move |_| {
        // Note: there is no async callbacks.
        Swal::fire(SwalOptions {
            title: "This is a title",
            text: "This is some text",
            icon: &SwalIcon::SUCCESS,
            confirm_button_text: "LETS GO",
            show_cancel_button: true,
            show_deny_button: true,
            pre_confirm: || {
                // This callback gets executed when the
                // confirmation button is pressed.
                info!("Confirmed !!");
            },
            pre_deny: || {
                // Same as "pre_confirm" but for the "Deny" button.
                Swal::fire(SwalOptions {
                    title: "You denied!",
                    then: |_| {
                        // This will get executed after the "then"
                        // of the parent swal.
                        info!("Inner Swal was dismissed");
                    },
                    ..SwalOptions::default()
                });
            },
            then: |result: SwalResult| {
                // "pre_confirm" and "pre_deny" execute BEFORE "then". Hence the "pre" prefix.
                // You don't actually need these functions since "then" contains the result
                // from which you can know if the popup was confirmed or denied.
                //
                // Note: this will get executed before the "then" of the inner swal
                // that is being open when the "Deny" button is pressed (look above).
                info!("The result of this alert is {:?}", result);
            },
            ..SwalOptions::default()
        });
        info!("This print statement will appear before the alert is dismissed.");
    };

    let warning = move |_| {
        Swal::fire(SwalOptions::basic_icon("This is a title", &SwalIcon::WARNING));
    };

    let error = move |_| {
        Swal::fire(SwalOptions::basic_icon("This is a title", &SwalIcon::ERROR));
    };

    let info = move |_| {
        Swal::fire(SwalOptions::basic_icon("This is a title", &SwalIcon::INFO));
    };

    let question = move |_| {
        Swal::fire(SwalOptions::basic_icon("This is a title", &SwalIcon::QUESTION));
    };

    view! {
        <div>
            <h2>"EXAMPLES"</h2>
            <div>
                <p>"A basic message"</p>
                <button on:click=success>"Try success!"</button>
                <button on:click=warning>"Try warning !"</button>
                <button on:click=error>"Try error !"</button>
                <button on:click=info>"Try info !"</button>
                <button on:click=question>"Try question !"</button>
            </div>
        </div>
    }
}
