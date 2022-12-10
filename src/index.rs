use super::*;
use super::body::Body;
use super::header::Header;

pub struct Index {
    current_tab: Tab,
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        route(ctx.link().callback(Msg::Routing));
        Index { current_tab: Tab::Home }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Routing(tab) => {
                if self.current_tab == tab {
                    false
                } else {
                    self.current_tab = tab;
                    true
                }
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <Header tab={ self.current_tab } />
            <Body tab={ self.current_tab } />
            </>
        }
    }
}