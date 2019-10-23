extern crate kagura;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main(){
    kagura::run(kagura::Component::new(0, update, render), "app");
}
type State = u64;

enum Msg {
    CountUp,
}

struct Sub;

fn update(state: &mut State, msg: &Msg)-> Option<Sub>{
    match msg {
        Msg::CountUp => {
            *state += 1;
        }
    }
     None
}

fn render(state: &State) -> kagura::Html<Msg> {
    use kagura::Attributes;
    use kagura::Events;
    use kagura::Html;

    let text = if state % 15 == 0 {
        "fizzbuzz".to_string()
    } else if state % 5 == 0 {
        "buzz".to_string()
    } else if state % 3 == 0 {
        "fizz".to_string()
    } else {
        state.to_string()
    };

    let color = if state % 3 == 0 { "#00f" } else { "#000" };

    let bg_color = if state % 5 == 0 { "#f00" } else { "#fff" };

    Html::button(
        Attributes::new()
            .style("color", color)
            .style("background-color", bg_color)
            .style("width", "10ch")
            .style("height", "2rem"),
        Events::new().on_click(|_| Msg::CountUp),
        vec![Html::unsafe_text(text)],
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
