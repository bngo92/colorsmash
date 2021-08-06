use rand::Rng;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, HtmlInputElement, HtmlButtonElement};
const COLOR_MAP: [(&str, (u16, u16, u16)); 8] = [
    ("black", (0, 0, 0)),
    ("red", (255, 0, 0)),
    ("green", (0, 255, 0)),
    ("blue", (0, 0, 255)),
    ("white", (255, 255, 255)),
    ("cyan", (0, 255, 255)),
    ("yellow", (255, 255, 0)),
    ("magenta", (255, 0, 255)),
];

struct State {
    color: (u8, u8, u8),
    smash: (u8, u8, u8),
    elements: StateElements,
    history: VecDeque<HtmlButtonElement>,
}

struct StateElements {
    document: Document,
    body: HtmlElement,
    navbar: HtmlElement,
    red_input: HtmlInputElement,
    green_input: HtmlInputElement,
    blue_input: HtmlInputElement,
    history: Element,
    bottom: HtmlElement,
}

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let red_input = document
        .get_element_by_id("red-input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    let green_input = document
        .get_element_by_id("green-input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    let blue_input = document
        .get_element_by_id("blue-input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    let state = Rc::new(RefCell::new(State {
        color: (128, 128, 128),
        smash: (0, 0, 0),
        elements: StateElements {
            document: document.clone(),
            body: document.body().unwrap(),
            navbar: document
                .get_element_by_id("navbar")
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap(),
            red_input: red_input.clone(),
            green_input: green_input.clone(),
            blue_input: blue_input.clone(),
            history: document.get_element_by_id("history").unwrap(),
            bottom: document
                .get_element_by_id("bottom")
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap(),
        },
        history: VecDeque::with_capacity(16),
    }));

    let state_ref = Rc::clone(&state);
    let a = Closure::wrap(Box::new(move || {
        if let Ok(red_value) = red_input.value().parse() {
            if let Ok(green_value) = green_input.value().parse() {
                if let Ok(blue_value) = blue_input.value().parse() {
                    state_ref.borrow_mut().set_background_color((
                        red_value,
                        green_value,
                        blue_value,
                    ));
                }
            }
        }
    }) as Box<dyn FnMut()>);
    document
        .get_element_by_id("input")
        .expect("should have #green-square on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#green-square be an `HtmlElement`")
        .set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();

    let red_smash = document
        .get_element_by_id("red-smash")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    let green_smash = document
        .get_element_by_id("green-smash")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    let blue_smash = document
        .get_element_by_id("blue-smash")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    let red_smash_ref = red_smash.clone();
    let green_smash_ref = green_smash.clone();
    let blue_smash_ref = blue_smash.clone();
    let state_ref = Rc::clone(&state);
    let a = Closure::wrap(Box::new(move || {
        if let Ok(red_value) = red_smash_ref.value().parse() {
            if let Ok(green_value) = green_smash_ref.value().parse() {
                if let Ok(blue_value) = blue_smash_ref.value().parse() {
                    state_ref
                        .borrow_mut()
                        .smash(&(red_value, green_value, blue_value));
                }
            }
        }
    }) as Box<dyn FnMut()>);
    document
        .get_element_by_id("smash")
        .expect("should have #green-square on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#green-square be an `HtmlElement`")
        .set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();

    let red_smash_ref = red_smash.clone();
    let green_smash_ref = green_smash.clone();
    let blue_smash_ref = blue_smash.clone();
    let state_ref = Rc::clone(&state);
    let a = Closure::wrap(Box::new(move || {
        if let Ok(red_value) = red_smash_ref.value().parse() {
            if let Ok(green_value) = green_smash_ref.value().parse() {
                if let Ok(blue_value) = blue_smash_ref.value().parse() {
                    state_ref
                        .borrow_mut()
                        .set_smash((red_value, green_value, blue_value));
                }
            }
        }
    }) as Box<dyn FnMut()>);
    red_smash.set_onchange(Some(a.as_ref().unchecked_ref()));
    green_smash.set_onchange(Some(a.as_ref().unchecked_ref()));
    blue_smash.set_onchange(Some(a.as_ref().unchecked_ref()));
    red_smash.set_onkeypress(Some(a.as_ref().unchecked_ref()));
    green_smash.set_onkeypress(Some(a.as_ref().unchecked_ref()));
    blue_smash.set_onkeypress(Some(a.as_ref().unchecked_ref()));
    red_smash.set_oninput(Some(a.as_ref().unchecked_ref()));
    green_smash.set_oninput(Some(a.as_ref().unchecked_ref()));
    blue_smash.set_oninput(Some(a.as_ref().unchecked_ref()));
    a.forget();

    let state_ref = Rc::clone(&state);
    let mut rng = rand::thread_rng();
    let a = Closure::wrap(Box::new(move || {
        state_ref
            .borrow_mut()
            .set_background_color_and_update((rng.gen(), rng.gen(), rng.gen()));
    }) as Box<dyn FnMut()>);
    document
        .get_element_by_id("random")
        .expect("should have #green-square on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#green-square be an `HtmlElement`")
        .set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();

    let state_ref = Rc::clone(&state);
    let mut rng = rand::thread_rng();
    let a = Closure::wrap(Box::new(move || {
        let red = rng.gen();
        let green = rng.gen();
        let blue = rng.gen();
        state_ref
            .borrow_mut()
            .set_smash((red, green, blue));
        red_smash.set_value(&red.to_string());
        green_smash.set_value(&green.to_string());
        blue_smash.set_value(&blue.to_string());
    }) as Box<dyn FnMut()>);
    document
        .get_element_by_id("random-smash")
        .expect("should have #green-square on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#green-square be an `HtmlElement`")
        .set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();

    for (c, rgb) in &COLOR_MAP {
        let state_ref = Rc::clone(&state);
        let a = Closure::wrap(Box::new(move || {
            state_ref.borrow_mut().smash(rgb);
        }) as Box<dyn FnMut()>);
        document
            .get_element_by_id(c)
            .expect("should have #green-square on the page")
            .dyn_ref::<HtmlElement>()
            .expect("#green-square be an `HtmlElement`")
            .set_onclick(Some(a.as_ref().unchecked_ref()));
        a.forget();
    }

    Ok(())
}

impl State {
    fn smash(&mut self, color: &(u16, u16, u16)) {
        self.set_background_color_and_update((
            ((7 * self.color.0 as u16 + color.0) / 8) as u8,
            ((7 * self.color.1 as u16 + color.1) / 8) as u8,
            ((7 * self.color.2 as u16 + color.2) / 8) as u8,
        ));
    }

    fn set_background_color_and_update(&mut self, color: (u8, u8, u8)) {
        self.set_background_color(color);
        self.elements.red_input.set_value(&color.0.to_string());
        self.elements.green_input.set_value(&color.1.to_string());
        self.elements.blue_input.set_value(&color.2.to_string());
    }

    fn set_background_color(&mut self, color: (u8, u8, u8)) {
        if self.color == color {
            return;
        }
        self.color = color;

        let text = format!("#{:x}{:x}{:x}", color.0, color.1, color.2);
        self.elements
            .body
            .style()
            .set_property("background", &text)
            .unwrap();
        self.elements
            .navbar
            .style()
            .set_property(
                "background",
                &format!(
                    "linear-gradient(to right, rgb(255,255,255), rgb({},{},{}))",
                    color.0, color.1, color.2
                ),
            )
            .unwrap();
        self.set_bottom();

        let val = self
            .elements
            .document
            .create_element("button")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        val.set_type("button");
        val.set_class_name("button big");
        val.set_text_content(Some(&text));
        val.style().set_property("background", &text).unwrap();
        val.style().set_property("display", "block").unwrap();

        if self.history.len() == 16 {
            let val = self.history.pop_front().unwrap();
            self.elements.history.remove_child(&val).unwrap();
        }
        self.elements.history.append_child(&val).unwrap();
        self.history.push_back(val);
    }

    fn set_smash(&mut self, color: (u8, u8, u8)) {
        self.smash = color;
        self.set_bottom();
    }

    fn set_bottom(&self) {
        self.elements
            .bottom
            .style()
            .set_property(
                "background",
                &format!(
                    "linear-gradient(to right, rgb({},{},{}), rgb({},{},{}))",
                    self.color.0,
                    self.color.1,
                    self.color.2,
                    self.smash.0,
                    self.smash.1,
                    self.smash.2
                ),
            )
            .unwrap();
    }
}
