use super::*;

#[function_component(LeftPanel)]
pub fn left_panel(props: &Props) -> Html {
    html!{
        <div id="left-panel">
            <div id="friend-links">
                <span>{"Friend Links"}</span>
                <span class="border-line"></span>
                <p>{"Cecilia has no friends yet."}</p>
            </div>
        </div>
    }
}
