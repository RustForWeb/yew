#![cfg(target_arch = "wasm32")]

mod common;

use std::time::Duration;

use common::obtain_result;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_composed_ref_works() {
    #[function_component(UseComposedRefComponent)]
    fn use_ref_comp() -> Html {
        let node_ref1 = use_node_ref();
        let node_ref2 = use_node_ref();
        let composed_ref = use_composed_ref(&[node_ref1.clone(), node_ref2.clone()]);

        let value1 = use_state_eq(|| None::<String>);
        use_effect_with(node_ref1.clone(), {
            let value1 = value1.clone();

            move |node_ref1| {
                value1.set(
                    node_ref1
                        .cast::<web_sys::HtmlInputElement>()
                        .map(|element| element.value()),
                );
            }
        });

        let value2 = use_state_eq(|| None::<String>);
        use_effect_with(node_ref2.clone(), {
            let value2 = value2.clone();

            move |node_ref2| {
                value2.set(
                    node_ref2
                        .cast::<web_sys::HtmlInputElement>()
                        .map(|element| element.value()),
                );
            }
        });

        let composed_value = use_state_eq(|| None::<String>);
        use_effect_with(composed_ref.clone(), {
            let composed_value = composed_value.clone();

            move |composed_ref| {
                composed_value.set(
                    composed_ref
                        .cast::<web_sys::HtmlInputElement>()
                        .map(|element| element.value()),
                );
            }
        });

        html! {
            <>
                <input ref={composed_ref.clone()} value="123" />

                <div>
                    {"The test output is: "}
                    <div id="result">
                        {(*value1).clone()}{'|'}{(*value2).clone()}{'|'}{(*composed_value).clone()}
                    </div>
                    {"\n"}
                </div>
            </>
        }
    }

    yew::Renderer::<UseComposedRefComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::from_millis(100)).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "123|123|123");
}
