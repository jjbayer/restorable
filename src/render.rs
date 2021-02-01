use crate::linefile::{Layer, Pen, Stroke};
use crate::notebook::Notebook;
use crate::page::Page;
use skia_safe::Document;
use std::io::Write;

// For size, see https://remarkablewiki.com/tech/filesystem
const DOCUMENT_WIDTH: i32 = 1404;
const DOCUMENT_HEIGHT: i32 = 1872;
const STROKE_WIDTH: f32 = 4.0;

pub fn render_notebook<Output: Write>(
    notebook: Notebook,
    output: &mut Output,
) -> Result<(), std::io::Error> {
    let mut document = skia_safe::pdf::new_document(None);

    for page in notebook.pages {
        document = render_page(page, document)?;
    }

    let data = document.close();

    output.write_all(data.as_ref())?;

    Ok(())
}

pub fn render_page(page: Page, document: Document) -> Result<Document, std::io::Error> {
    let document = document.begin_page((1404, 1874), None);

    // document.render_page(DOCUMENT_WIDTH, DOCUMENT_HEIGHT, |canvas| {
    //     // Flip y coordinates:
    //     canvas.concat(Matrix::scale(1.0, -1.0))?;
    //     canvas.concat(Matrix::translate(0.0, -DOCUMENT_HEIGHT))?;

    //     canvas.set_line_cap_style(CapStyle::Round)?;
    //     canvas.set_line_join_style(JoinStyle::Round)?;

    //     for layer in page.linefile.layers {
    //         render_layer(layer, canvas)?;
    //     }

    // })
    Ok(document.end_page())
}

// pub fn render_layer(layer: Layer, canvas: &mut Canvas) -> Result<(), std::io::Error> {
//     for stroke in layer.strokes {
//         render_stroke(stroke, canvas)?;
//     }

//     Ok(())
// }

// pub fn render_stroke(stroke: Stroke, canvas: &mut Canvas) -> Result<(), std::io::Error> {
//     // TODO: set fill color, pressure, etc.
//     canvas.set_line_width(stroke.width)?;
//     canvas.set_stroke_color(match stroke.pen {
//         Pen::Highlighter => Color::rgb(0, 255, 255),
//         _ => Color::gray(0),
//     })?;

//     for (i, segment) in stroke.segments.iter().enumerate() {
//         canvas.set_line_width(STROKE_WIDTH * segment.pressure * stroke.width)?;
//         let (x, y) = (segment.x, segment.y);
//         if i == 0 {
//             canvas.move_to(x, y)?;
//         } else {
//             canvas.line_to(x, y)?;
//         }
//     }

//     canvas.stroke()?;

//     Ok(())
// }
