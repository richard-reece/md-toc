mod config;
mod error;
mod toc;

#[macro_use]
extern crate seed;
use seed::prelude::*;

struct Model {
    pub content: String,
    pub rendered: String,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            content: "paste your markdown here".to_string(),
            rendered: "TOC will appear here".to_string(),
        }
    }
}

#[derive(Clone)]
enum Msg {
    NewContent(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::NewContent(s) => generate_toc(model, s),
    }
}

fn generate_toc(model: &mut Model, content: String) {
    model.content = content;
    let lines: Vec<String> = model.content.split("\n").map(|s| String::from(s)).collect();
    model.rendered = toc::find_headings(lines, 2).join("\n");
} 

fn view(model: &Model) -> impl View<Msg> {
    div![
        h1![ "Enter your Markdown below:" ],
        textarea![ model.content, attrs!{At::Rows => 10, At::Cols => 80}, input_ev(Ev::Input, Msg::NewContent) ],
        pre![model.rendered]
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Model::default(), update, view)
        .finish()
        .run();

}