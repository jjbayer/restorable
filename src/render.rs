use crate::linefile::{Layer, Stroke};
use crate::notebook::Notebook;
use crate::page::Page;
use svg::node::element::{Circle, Group};

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

pub fn render_stroke(stroke: Stroke) -> Group {
    let mut group = Group::new(); //.move_to((0, 0));

    for segment in stroke.segments {
        group = group.add(
            Circle::new()
                .set("cx", segment.x)
                .set("cy", segment.y)
                .set("r", 2)
                .set("fill", "black")
                .set("stroke", "none"),
        );
    }

    group

    // Path::new()
}
