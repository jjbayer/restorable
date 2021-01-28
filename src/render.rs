use crate::linefile::{Layer, Pen, Stroke};
use crate::notebook::Notebook;
use crate::page::Page;
use pdf_canvas::{
    graphicsstate::{CapStyle, Color, JoinStyle, Matrix},
    Canvas, Pdf,
};

// For size, see https://remarkablewiki.com/tech/filesystem
const DOCUMENT_WIDTH: f32 = 1404.0;
const DOCUMENT_HEIGHT: f32 = 1872.0;
const STROKE_WIDTH: f32 = 4.0;

pub fn render_notebook(notebook: Notebook, output_path: &str) -> Result<(), std::io::Error> {
    let mut document = Pdf::create(output_path)?;

    for page in notebook.pages {
        render_page(page, &mut document)?;
    }

    document.finish()
}

pub fn render_page(page: Page, document: &mut Pdf) -> Result<(), std::io::Error> {
    document.render_page(DOCUMENT_WIDTH, DOCUMENT_HEIGHT, |canvas| {
        // Flip y coordinates:
        canvas.concat(Matrix::scale(1.0, -1.0))?;
        canvas.concat(Matrix::translate(0.0, -DOCUMENT_HEIGHT))?;

        canvas.set_line_cap_style(CapStyle::Round)?;
        canvas.set_line_join_style(JoinStyle::Round)?;

        for layer in page.linefile.layers {
            render_layer(layer, canvas)?;
        }

        Ok(())
    })
}

pub fn render_layer(layer: Layer, canvas: &mut Canvas) -> Result<(), std::io::Error> {
    for stroke in layer.strokes {
        render_stroke(stroke, canvas)?;
    }

    Ok(())
}

pub fn render_stroke(stroke: Stroke, canvas: &mut Canvas) -> Result<(), std::io::Error> {
    // TODO: set fill color, pressure, etc.
    canvas.set_line_width(stroke.width)?;
    canvas.set_stroke_color(match stroke.pen {
        Pen::Highlighter => Color::rgb(0, 255, 255),
        _ => Color::gray(0),
    })?;

    for (i, segment) in stroke.segments.iter().enumerate() {
        canvas.set_line_width(STROKE_WIDTH * segment.pressure * stroke.width)?;
        let (x, y) = (segment.x, segment.y);
        if i == 0 {
            canvas.move_to(x, y)?;
        } else {
            canvas.line_to(x, y)?;
        }
    }

    canvas.stroke()?;

    Ok(())
}
