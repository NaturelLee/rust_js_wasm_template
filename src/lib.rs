use wasm_bindgen::prelude::*;

// use js in rust
// 声明类似头文件
#[wasm_bindgen(module = "/myClass.js")]
extern "C" {
    type MyClass;
    #[wasm_bindgen(constructor)]
    fn new() -> MyClass;
    #[wasm_bindgen(method, getter)]
    fn num(this: &MyClass) -> u32;
    #[wasm_bindgen(method, setter)]
    fn set_num(this: &MyClass, n: u32) -> MyClass;
    #[wasm_bindgen(method)]
    fn render(this: &MyClass) -> String;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(string_item: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // for js to call
    let window = web_sys::window().expect("got window error");
    let document = window.document().expect("got document error");
    let body = document.body().expect("got body error");

    let new_element = document.create_element("p")?;
    new_element.set_inner_html("hello rust and wasm");

    body.append_child(&new_element)?;

    // use js class in rust
    log("hello js, this is rust");

    let ins = MyClass::new();

    assert_eq!(ins.num(), 78);

    log(&ins.num().to_string());

    ins.set_num(90);

    assert_eq!(ins.num(), 90);

    log(&ins.render());

    Ok(())
}
