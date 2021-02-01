use crate::linefile::{Color, Layer, Pen, Stroke};
use crate::notebook::Notebook;
use crate::page::Page;
use skia_safe as skia;
use std::io::Write;

const BASE_STROKE_WIDTH: f32 = 4.0;

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

pub fn render_page(page: Page, document: skia::Document) -> Result<skia::Document, std::io::Error> {
    let mut document = document.begin_page((1404, 1874), None);

    // TODO:
    // canvas.set_line_cap_style(CapStyle::Round)?;
    // canvas.set_line_join_style(JoinStyle::Round)?;

    for layer in page.linefile.layers {
        render_layer(layer, document.canvas())?;
    }

    Ok(document.end_page())
}

pub fn render_layer(layer: Layer, canvas: &mut skia::Canvas) -> Result<(), std::io::Error> {
    for stroke in layer.strokes {
        render_stroke(stroke, canvas)?;
    }

    Ok(())
}

pub fn render_stroke(stroke: Stroke, canvas: &mut skia::Canvas) -> Result<(), std::io::Error> {
    // TODO: set fill color, pressure, etc.

    let mut paint = skia::Paint::default();
    paint.set_color(color(&stroke));
    paint.set_stroke_width(stroke_width(&stroke));
    paint.set_style(skia::PaintStyle::Stroke);
    paint.set_stroke_cap(skia::paint::Cap::Round);
    paint.set_stroke_join(skia::paint::Join::Round);

    let mut path = skia::Path::new();
    for (i, segment) in stroke.segments.iter().enumerate() {
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

fn color(stroke: &Stroke) -> skia::Color {
    match stroke.pen {
        Pen::Highlighter => skia::Color::from_rgb(255, 255, 0),
        _ => match stroke.color {
            Color::Black => skia::Color::BLACK,
            Color::Gray => skia::Color::GRAY,
            Color::White => skia::Color::WHITE,
        },
    }
}

fn stroke_width(stroke: &Stroke) -> f32 {
    // Determined by trial and error
    let w = stroke.width;
    pen_scale(&stroke.pen) * (w * w * w - 4.0)
}

fn pen_scale(pen: &Pen) -> f32 {
    // Determined by trial and error
    match pen {
        Pen::Marker => 3.0,
        Pen::PaintBrush => 2.5,
        Pen::Pencil => 2.0,
        Pen::Highlighter => 5.0,
        _ => 1.0,
    }
}
