use yew::prelude::*;
use crate::header::*;

#[function_component(Home)]
pub fn home() -> Html {
    /*
    <Text fontSize={{ base: "2xl", md: "3xl" }}>
    I am a <span className="yellow-pink">Software Engineer</span>
    {isDesktop && " | "} {/* Only render the vertical bar on larger screens */}
    <br className={{ base: isDesktop ? "hidden" : "block", md: "hidden" }} /> {/* Only render the line break on smaller screens */}
    <span className="yellow-pink">Full stack dev</span> by day.
</Text>
<Text fontSize={{ base: "2xl", md: "3xl" }}>
    <span className="solana">Solana dev</span> by night.
</Text>

     */

    html! {
        <div>
            <Header/>
            <div/>
            <h2>{ "Eduardo Lanasca" }</h2>
            <p>
            {"I am a "}
            {
                html!{
                    <>
                        <span>{"Software Engineer"}</span>
                        <span>{" | "}</span>
                        <br/>
                        <span>{"Full stack dev"}</span>
                    </>
                }
            }
            {" by day."}
        </p>
        </div>
    }
}