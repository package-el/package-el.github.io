use super::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children
}

#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    html! {
        <div id="content">
            { for props.children.iter() }
        </div>
    }
}