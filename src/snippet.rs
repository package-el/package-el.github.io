use super::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub modified_time: String,
    pub signature: String,
}

#[function_component(Snippet)]
pub fn snippet(props: &Props) -> Html {
    html! {
        <div class="snippet">
            <div class="snippet-inner-content">
                { for props.children.iter() }
            </div>
            <div class="snippet-adjuncts">
                <span class="snippet-signature"> {props.signature.clone()} </span>
                <span class="snippet-modified-time"> {props.modified_time.clone()} </span>
            </div>
        </div>
    }
}
