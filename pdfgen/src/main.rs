mod images;

fn main() {
    println!("Hello, world!");
    //sample_page();
    //sample_graphical_page();
    //sample_image();
    //sample_im2();
    //images::main();
    
    let filename = "sample_couvert.pdf";
    couvert_doc().save(&mut std::io::BufWriter::new(std::fs::File::create(filename).unwrap())).unwrap();
}

fn couvert_doc() -> printpdf::PdfDocumentReference {
    use printpdf::*;

    // document config
    let document_title = "Versand";
    let main_font_size = 12;
    let names_font_size = 11;
    let page_width = Mm(162.0);
    let page_height = Mm(114.0);
    let address_offset_x = Mm(110.0);
    let address_offset_y = Mm(50.0);
    let names_offset_x = Mm(22.0);
    let names_offset_y = page_height - Mm(20.0);
    
    // sample couvert config
    let sample_text = "Levanzo";

    // create the document
    let (doc, page1, layer1) : (PdfDocumentReference, indices::PdfPageIndex, indices::PdfLayerIndex) = PdfDocument::new(document_title, page_width, page_height, /*initial_layer_name*/"Layer 1");
    // load a font
    let font_calibri = doc.add_external_font(std::fs::File::open("res/fonts/calibri.ttf").unwrap()).unwrap();
    let font_calibri_light = doc.add_external_font(std::fs::File::open("res/fonts/calibril.ttf").unwrap()).unwrap();
    // prepare usage variables
    let current_page = doc.get_page(page1);
    let current_layer = current_page.get_layer(layer1);

    // place the logo first, so that it is in the background
    // original logo is at 300 dpi approx 16/0.15 = 106mm
    add_bitmap_to_layer(&current_layer,
                        Some(Mm(5.0)), Some(page_height - Mm(16.0) - Mm(5.0) ),
                        Some(0.15), Some(0.15)
                        );

    // position the names
    current_layer.use_text(sample_text, names_font_size, names_offset_x, names_offset_y, &font_calibri);

    // position sample address
    {
        let font_addresses = font_calibri_light;
        current_layer.begin_text_section();
        current_layer.set_font(&font_addresses, main_font_size);
        current_layer.set_text_cursor(address_offset_x, address_offset_y);
        current_layer.set_line_height(main_font_size);
        current_layer.set_word_spacing(3000);
        current_layer.set_character_spacing(0);
        current_layer.set_text_rendering_mode(/*Fill, Stroke, FillStroke, Invisible, FillClip, StrokeClip, FillStrokeClip, Clip*/TextRenderingMode::Fill);

        let address = vec!["Familie Mink", "Neuwiesenstr. 2", "8332 Russikon"];
        for line in address {
            current_layer.write_text(line, &font_addresses);
            current_layer.add_line_break();
        }

        current_layer.end_text_section();
    }

    // position sample sidebadge
    draw_sidebadge(&current_layer, Mm(0.0), Mm(40.0));

    return doc;
} 

fn draw_sidebadge (current_layer: &printpdf::PdfLayerReference,
                   origin_x: printpdf::Mm,
                   origin_y: printpdf::Mm) {
    use printpdf::{Point, Line, Mm};

    let badge_height = Mm(20.0);
    let badge_width = Mm(40.0);

    // point relative to lower left corner (pos_x, pos_y)
    let mut point = |posx: f64, posy: f64| -> Point {
        let printpdf::Mm(pos_x) = origin_x;
        let printpdf::Mm(pos_y) = origin_y;
        Point::new(Mm(pos_x + posx), Mm(pos_y + posy))
    };

    // The "false" determines if the next (following)
    // point is a bezier handle (for curves)
    // If you want holes, simply reorder the winding of the points to be
    // counterclockwise instead of clockwise.
    let points1 = vec![
        (Point::new(origin_x              , badge_height + origin_y), false),
        (Point::new(origin_x + badge_width, origin_y + badge_height), false),
        (Point::new(origin_x + badge_width, origin_y               ), false),
        (Point::new(origin_x              , origin_y               ), false)
    ];

    // Is the shape stroked? Is the shape closed? Is the shape filled?
    let line1 = Line {
        points: points1,
        is_closed: true,
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };

    // draw
    current_layer.add_shape(line1);
}

fn add_bitmap_to_layer(current_layer : &printpdf::PdfLayerReference, 
                       posx : Option<printpdf::Mm>,
                       posy : Option<printpdf::Mm>,
                       scalex : Option<f64>,
                       scaley : Option<f64>) {
    use printpdf::*;
    use std::io::Cursor;
    use image::bmp::BMPDecoder;
    use std::io::BufWriter;
    let mut image_file = std::fs::File::open("res/images/logo.bmp").unwrap();
    let decoder = BMPDecoder::new(&mut image_file).unwrap();
    let image = Image::try_from(decoder).unwrap();
    // translate x, translate y, rotate, scale x, scale y, dpi
    image.add_to_layer(current_layer.clone(), posx, posy, None, scalex, scaley, None);
}

fn sample_page(){
    use printpdf::*;
    use std::fs::File;
    use std::io::BufWriter;

    let (doc, page1, layer1) : (PdfDocumentReference, indices::PdfPageIndex, indices::PdfLayerIndex) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");

    //text
    let text = "unicode: как теба завут?";
    let font = doc.add_external_font(File::open("src/assets/fonts/calibri.ttf").unwrap()).unwrap();
    // text, font size, x from left edge, y from bottom edge, font
    doc.get_page(page1).get_layer(layer1).use_text(text, 48, Mm(10.0), Mm(10.0), &font);

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
        let mut image_file = File::open(("cevisvg.bmp")).unwrap();
        let decoder = image::bmp::BMPDecoder::new(&mut image_file).unwrap();
        let image = printpdf::Image::try_from(decoder).unwrap();

        //let mut image_file = File::open("1200px-Cevi.svg.bmp").unwrap();
        //let image = Image::try_from(image::bmp::BMPDecoder::new(&mut image_file).unwrap()).unwrap();

        // translate x, translate y, rotate, scale x, scale y
        // by default, an image is optimized to 300 DPI (if scale is None)
        // rotations and translations are always in relation to the lower left corner
        image.add_to_layer(current_layer.clone(), None, None, None, None, None, None);

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


fn sample_im2() {
    use printpdf::*;
    use std::io::Cursor;
    use image::bmp::BMPDecoder;
    use std::fs::File;
    use std::io::BufWriter;

    let (doc, page1, layer1) = PdfDocument::new("printpdf graphics test", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // currently, the only reliable file format is bmp (jpeg works, but not in release mode)
    // this is an issue of the image library, not a fault of printpdf

    let image_bytes = include_bytes!("cevisvg.bmp");
    let mut reader = Cursor::new(image_bytes.as_ref());

    let decoder = BMPDecoder::new(&mut reader).unwrap();
    let image2 = Image::try_from(decoder).unwrap();

    // layer,
    image2.add_to_layer(current_layer.clone(), None, None, None, None, None, None);

    doc.save(&mut BufWriter::new(File::create("test_image2.pdf").unwrap())).unwrap();
}
