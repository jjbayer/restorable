use crate::linefile::{Layer, Pen, Stroke};
use crate::notebook::Notebook;
use crate::page::Page;
use skia_safe::{Canvas, Color, Document, Paint, PaintStyle, Path};
use std::io::Write;

// For size, see https://remarkablewiki.com/tech/filesystem

pub fn render_notebook<Output: Write>(
    notebook: Notebook,
    output: &mut Output,
) -> Result<(), std::io::Error> {
    // TODO: metadata

    let mut document = skia_safe::pdf::new_document(None);

    for page in notebook.pages {
        document = render_page(page, document)?;
    }

    let data = document.close();

    output.write_all(data.as_ref())?;

    Ok(())
}

pub fn render_page(page: Page, document: Document) -> Result<Document, std::io::Error> {
    let mut document = document.begin_page((1404, 1874), None);

    // TODO:
    // canvas.set_line_cap_style(CapStyle::Round)?;
    // canvas.set_line_join_style(JoinStyle::Round)?;

    for layer in page.linefile.layers {
        render_layer(layer, document.canvas())?;
    }

    Ok(document.end_page())
}

pub fn render_layer(layer: Layer, canvas: &mut Canvas) -> Result<(), std::io::Error> {
    for stroke in layer.strokes {
        render_stroke(stroke, canvas)?;
    }

    Ok(())
}

pub fn render_stroke(stroke: Stroke, canvas: &mut Canvas) -> Result<(), std::io::Error> {
    // TODO: set fill color, pressure, etc.

    let mut paint = Paint::default();
    paint.set_color(match stroke.pen {
        Pen::Highlighter => Color::from_rgb(255, 255, 0),
        _ => Color::BLACK,
    });
    paint.set_stroke_width(stroke.width);
    paint.set_style(PaintStyle::Stroke);

    let mut path = Path::new();
    for (i, segment) in stroke.segments.iter().enumerate() {
        // canvas.set_line_width(STROKE_WIDTH * segment.pressure * stroke.width)?;
        let (x, y) = (segment.x, segment.y);
        if i == 0 {
            path.move_to((x, y));
        } else {
            path.line_to((x, y));
        }
    }

    canvas.draw_path(&path, &paint);

    Ok(())
}
