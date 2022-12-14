use super::*;
use snippet::Snippet;

#[function_component(Shatters)]
pub fn shatters(props: &Props) -> Html {
    html! {
        <div id="shatters">
            <Snippet signature="浅原つぼみ" modified_time="2022/12/14 17:24">
                <p>{"“这是我和你的最后一笔交易了，老东西。一分钟之后，就连牛顿定律也管不着我啦！”"}</p>
            </Snippet>
            <Snippet signature="Cecilia" modified_time="2022/12/14 09:08">
                <p>{"尝试用GitHub Actions自动部署，发现每次都要从头重新编译一遍Rust的依赖，好慢呜。"}</p>
                <p>{"不懂懂😵‍💫"}</p>
            </Snippet>
            <Snippet signature="浅原つぼみ" modified_time="2022/12/14 03:09">
                <p>{"我爱你们，我爱全地上所有的人。"}</p>
                <p>{"正因如此，我必须一个人孤独死去。"}</p>
            </Snippet>
            <Snippet signature="浅原つぼみ" modified_time="2022/12/11 03:21">
                <p>{"被好多好多的她和她们传唱过的隐秘童话。"}</p>
                <p class="math">{r"\(zx=zy\to x=y\)"}</p>
                <p class="math">{r"\(xz=yz\to x=y\)"}</p>
                <p class="math">{r"\(xz=yz\to z=z\)"}</p>
            </Snippet>
            <Snippet signature="浅原つぼみ" modified_time="2022/12/11 01:56">
                <p>{"靠在枕头上工作很久之后肩膀好痛痛痛痛痛痛。不明白为什么有人喜欢work from bed..."}</p>
            </Snippet>
            <Snippet signature="Cecilia" modified_time="2022/12/11 00:31">
                <p>{"「私は明日死ぬだろう」"}</p>
                <p>{"与过去决裂之后，去往那之前的更久的过去……"}</p>
            </Snippet>
            <Snippet signature="浅原つぼみ" modified_time="2022/12/10 23:51">
                <p>{"こちら、群青学院放送部"}</p>
                <p>{"生きている人、いますか？"}</p>
            </Snippet>
        </div>
    }
}
