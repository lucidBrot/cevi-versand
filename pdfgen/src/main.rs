fn main() {
    println!("Hello, world!");
    sample_page();
    sample_graphical_page();
    sample_image();
}

fn sample_page(){
    use printpdf::*;
    use std::fs::File;
    use std::io::BufWriter;

    let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
    let (page2, layer1) = doc.add_page(Mm(10.0), Mm(250.0),"Page 2, Layer 1");

    doc.save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();
}

fn sample_graphical_page(){
    use printpdf::*;
    use std::fs::File;
    use std::io::BufWriter;
    use std::iter::FromIterator;

    let (doc, page1, layer1) = PdfDocument::new("printpdf graphics test", Mm(500.0), Mm(500.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Quadratic shape. The "false" determines if the next (following)
    // point is a bezier handle (for curves)
    // If you want holes, simply reorder the winding of the points to be
    // counterclockwise instead of clockwise.
    let points1 = vec![(Point::new(Mm(100.0), Mm(100.0)), false),
    (Point::new(Mm(100.0), Mm(200.0)), false),
    (Point::new(Mm(300.0), Mm(200.0)), false),
    (Point::new(Mm(300.0), Mm(100.0)), false)];

    // Is the shape stroked? Is the shape closed? Is the shape filled?
    let line1 = Line {
        points: points1,
        is_closed: true,
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };

    // Triangle shape
    // Note: Line is invisible by default, the previous method of
    // constructing a line is recommended!
    let mut line2 = Line::from_iter(vec![
                                    (Point::new(Mm(150.0), Mm(150.0)), false),
                                    (Point::new(Mm(150.0), Mm(250.0)), false),
                                    (Point::new(Mm(350.0), Mm(250.0)), false)]);

    line2.set_stroke(true);
    line2.set_closed(false);
    line2.set_fill(false);
    line2.set_as_clipping_path(false);

    let fill_color = Color::Cmyk(Cmyk::new(0.0, 0.23, 0.0, 0.0, None));
    let outline_color = Color::Rgb(Rgb::new(0.75, 1.0, 0.64, None));
    let mut dash_pattern = LineDashPattern::default();
    dash_pattern.dash_1 = Some(20);

    current_layer.set_fill_color(fill_color);
    current_layer.set_outline_color(outline_color);
    current_layer.set_outline_thickness(10.0);

    // Draw first line
    current_layer.add_shape(line1);

    let fill_color_2 = Color::Cmyk(Cmyk::new(0.0, 0.0, 0.0, 0.0, None));
    let outline_color_2 = Color::Greyscale(Greyscale::new(0.45, None));

    // More advanced graphical options
    current_layer.set_overprint_stroke(true);
    current_layer.set_blend_mode(BlendMode::Seperable(SeperableBlendMode::Multiply));
    current_layer.set_line_dash_pattern(dash_pattern);
    current_layer.set_line_cap_style(LineCapStyle::Round);

    current_layer.set_fill_color(fill_color_2);
    current_layer.set_outline_color(outline_color_2);
    current_layer.set_outline_thickness(15.0);

    // draw second line
    current_layer.add_shape(line2);

    //save 
    doc.save(&mut BufWriter::new(File::create("test_graphic.pdf").unwrap())).unwrap();
}

fn sample_image(){
    // imports the `image` library with the exact version that we are using
    use printpdf::*;

    use std::convert::From;
    use std::fs::File;
    use std::io::BufWriter;

    
        let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // currently, the only reliable file format is bmp (jpeg works, but not in release mode)
        // this is an issue of the image library, not a fault of printpdf
        let image_bytes = include_bytes!("1200px-cevi.svg.bmp");
        let mut reader = std::io::Cursor::new(image_bytes.as_ref());
        let decoder = image::bmp::BMPDecoder::new(&mut reader).unwrap();
        let image = Image::try_from(decoder).unwrap();

        //let mut image_file = File::open("1200px-Cevi.svg.bmp").unwrap();
        //let image = Image::try_from(image::bmp::BMPDecoder::new(&mut image_file).unwrap()).unwrap();

        // translate x, translate y, rotate, scale x, scale y
        // by default, an image is optimized to 300 DPI (if scale is None)
        // rotations and translations are always in relation to the lower left corner
        image.add_to_layer(current_layer.clone(), None, None, None, None, Some(600.0), Some(600.0));

        /*
        // you can also construct images manually from your data:
        let mut image_file_2 = ImageXObject {
            width: Px(200),
            height: Px(200),
            color_space: ColorSpace::Greyscale,
            bits_per_component: ColorBits::Bit8,
            interpolate: true,
            /* put your bytes here. Make sure the total number of bytes =
               width * height * (bytes per component * number of components)
               (e.g. 2 (bytes) x 3 (colors) for RGB 16bit) */
            image_data: Vec::new(),
            image_filter: None, /* does not work yet */
            clipping_bbox: None, /* doesn't work either, untested */
        };

        let image2 = Image::from(image_file_2);
        */

        //save 
        doc.save(&mut BufWriter::new(File::create("test_image.pdf").unwrap())).unwrap();
    
}
