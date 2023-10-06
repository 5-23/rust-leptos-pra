use leptos::*;


#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                // set_count(count.get() + 1);
                set_count.update(|x|*x += 1);
            }
        >
            "Click me: "
            {move || count.get()}
        </button>
        <progress max="50" value={count}/>
    }
}

fn main() {
    mount_to_body(|| view! { 
        <App/>
     })
}
