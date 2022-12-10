use super::*;
use lpanel::LeftPanel;
use rpanel::RightPanel;

use content::Content;
use shatters::Shatters;

#[function_component(Body)]
pub fn body(props: &Props) -> Html {
    let b = web_sys::window().unwrap().document().unwrap().body().unwrap();
    if props.tab == Tab::AboutMe {
        b.set_id("b1");
    } else {
        b.set_id("b0");
    }
    match props.tab {
        Tab::Home => html! {
            <div id="main">
                <div id="home-vfill">
                    <div id="home"/>
                </div>
            </div>
        },
        Tab::Shatters => html! {
            <div id="main">
                <LeftPanel />
                <Content>
                    <Shatters />
                </Content>
                <RightPanel />
        </div>
        },
        Tab::Bibliothek => html! {
            <div id="main">
                <LeftPanel />
                <Content>
                    {"nothing here right now"}
                </Content>
                <RightPanel />
            </div>
        },
        Tab::AboutMe => html! {
            <></>
        },
    }
}