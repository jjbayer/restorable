use crate::linefile::{Layer, Stroke};
use crate::notebook::Notebook;
use crate::page::Page;
use svg::node::element::path::Data;
use svg::node::element::{Group, Path};

pub fn render_notebook(notebook: Notebook) -> svg::Document {
    let mut document = svg::Document::new();

    for page in notebook.pages {
        document = document.add(render_page(page));
    }

    document
}

pub fn render_page(page: Page) -> Group {
    let mut group = Group::new();

    for layer in page.linefile.layers {
        group = group.add(render_layer(layer));
    }

    group
}

pub fn render_layer(layer: Layer) -> Group {
    let mut group = Group::new();

    for stroke in layer.strokes {
        group = group.add(render_stroke(stroke));
    }

    group
}

pub fn render_stroke(stroke: Stroke) -> Path {
    let mut data = Data::new();

    for (i, segment) in stroke.segments.iter().enumerate() {
        let coords = (segment.x, segment.y);
        if i == 0 {
            data = data.move_to(coords);
        } else {
            data = data.line_to(coords);
        }
    }

    Path::new()
        .set("fill", "none")
        .set("stroke", "black") // TODO: map
        .set("stroke-width", stroke.width)
        .set("d", data)
}
