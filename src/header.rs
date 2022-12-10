use super::*;

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    let is_current = |tab| {
        if tab == props.tab {
            "active-tab"
        } else {
            "inactive-tab"
        }
    };

    html! {
        <header>
            <nav>
                <a href={HOME} class={
                    classes!("button", is_current(Tab::Home))
                }>{"Home"}</a>
                <a href={SHATTERS} class={
                    classes!("button", is_current(Tab::Shatters))
                }>{"Shatters"}</a>
                <a href={BIBLIOTHEK} class={
                    classes!("button", is_current(Tab::Bibliothek))
                }>{"Bibliothek"}</a>
                <a href={ABOUTME} class={
                    classes!("button", is_current(Tab::AboutMe))
                }>{"About"}</a>
            </nav>
        </header>
    }
}
