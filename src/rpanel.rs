use super::*;

#[function_component(RightPanel)]
pub fn right_panel(props: &Props) -> Html {
    html!{
        <div id="right-panel">
            <div id="personal-links">
                <span>{"Personal Links"}</span>
                <span class="border-line"></span>
                <p><a href="mailto://i@cecilia.moe">{"i@cecilia.moe"}<i class="bi bi-envelope-fill"></i></a></p>
                <p><a href="mailto://package_el@pm.me">{"package_el@pm.me"}<i class="bi bi-envelope-fill"></i></a></p>
                <p><a href="https://t.me/package_el">{"package_el"}<i class="bi bi-telegram"></i></a></p>
                <p><a href="https://t.me/+GJgbO1uno4I1MWJl">{".emacs.d"}<i class="bi bi-telegram"></i></a></p>
                <p><a href="https://twitter.com/CeciliaK__">{"CeciliaK__"}<i class="bi bi-twitter"></i></a></p>
                <p><a href="https://github.com/package-el">{"package-el"}<i class="bi bi-github"></i></a></p>
            </div>
        </div>
    }
}
