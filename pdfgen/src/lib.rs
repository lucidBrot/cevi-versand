mod pluralizable;
use pluralizable::Pluralizable;
use std::collections::HashMap;
use std::env;

pub fn main() {
    println!("Hello, world!");

    let filename = "sample_couvert.pdf";

    let receiver1 = Receiver {
        nickname: String::from("Focus"),
        group: String::from("Skapande"),
        role: Role::Leiter,
    };
    let receiver2 = Receiver {
        nickname: String::from("Levanzo"),
        group: String::from("Holon"),
        role: Role::Leiter,
    };
    let receiver3 = Receiver {
        nickname: String::from("Pseudo"),
        group: String::from("Trägerkreis"),
        role: Role::Traegerkreis,
    };
    let mut receivers = vec![receiver1, receiver2, receiver3];

    for arg in env::args().skip(1){
        receivers.push(Receiver {
            nickname: String::from(arg),
            group: String::from("Arg Group"),
            role: Role::Teilnehmer
        });
    }


    let address = vec!["Familie Mink", "Neuwiesenstr. 2", "8332 Russikon"];
    let couverts : Vec<CouvertInfo> = vec![CouvertInfo {
        receivers: receivers,
        address: vec_str_to_vec_string(&address),
    }];
    generate_couverts(couverts).save(&mut std::io::BufWriter::new(std::fs::File::create(filename).unwrap())).unwrap();
}

pub struct CouvertInfo {
    pub receivers: Vec<Receiver>,
    pub address: Vec<String>,
}

pub fn vec_str_to_vec_string(v: &Vec<&str>) -> Vec<String> {
    let mut vec: Vec<String> = Vec::<String>::new();
    for s in v.iter(){
        vec.push(String::from(*s));
    }
    return vec;
}

pub fn generate_couverts(couverts : Vec<CouvertInfo>) -> printpdf::PdfDocumentReference {
    use printpdf::*;

    // document config
    let document_title = "Versand";
    let address_font_size = 18;
    let names_font_size = 11;
    let badge_text_font_size = 11;
    let page_width = Mm(229.0);
    let page_height = Mm(162.0);
    let address_offset_x = Mm(120.0);
    let address_offset_y = Mm(65.0);
    let border_wh = Mm(12.0);
    let names_offset_x = border_wh + Mm(20.0);
    let names_offset_y = page_height - Mm(18.0);

    // create the document
    let (doc, page1, layer1) : (PdfDocumentReference, indices::PdfPageIndex, indices::PdfLayerIndex) = PdfDocument::new(document_title, page_width, page_height, /*initial_layer_name*/"Layer 1");
    // load a font
    let font_calibri = doc.add_external_font(std::fs::File::open("res/fonts/calibri.ttf").unwrap()).unwrap();
    let font_calibri_light = doc.add_external_font(std::fs::File::open("res/fonts/calibril.ttf").unwrap()).unwrap();

    for (num, couvert) in couverts.iter().enumerate() {
        // add new page
        println!("Generating page {}", num);
        let (next_page, layer1) = doc.add_page(page_width, page_height, format!("Page {}, Layer 1", num));

        // prepare usage variables
        let current_page = doc.get_page(next_page);
        let current_layer = current_page.get_layer(layer1);

        // place the logo first, so that it is in the background
        // original logo is at 300 dpi approx 16/0.15 = 106mm
        add_bitmap_to_layer(&current_layer,
                            Some(border_wh), Some(page_height - Mm(16.0) - border_wh ),
                            Some(0.15), Some(0.15)
                           );

        // draw names
        draw_names(&current_layer, &font_calibri, names_font_size, (names_offset_x, names_offset_y),
        couvert.receivers.iter().map(|r:&Receiver| (&r.nickname as &str, &r.group as &str))
        );

        // position sample address
        {
            let font_addresses = font_calibri_light.clone();
            current_layer.begin_text_section();
            current_layer.set_font(&font_addresses, address_font_size);
            current_layer.set_text_cursor(address_offset_x, address_offset_y);
            current_layer.set_line_height(address_font_size);
            current_layer.set_word_spacing(3000);
            current_layer.set_character_spacing(0);
            current_layer.set_text_rendering_mode(/*Fill, Stroke, FillStroke, Invisible, FillClip, StrokeClip, FillStrokeClip, Clip*/TextRenderingMode::Fill);

            for line in &(couvert.address) {
                current_layer.write_text(line.clone(), &font_addresses);
                current_layer.add_line_break();
            }

            current_layer.end_text_section();
        }

        // numbers in sidebadge
        let rolecount_dict : HashMap<Role, usize> = couvert.receivers.iter().fold(
            /*init:*/ HashMap::new(),
            /*f(map, item):*/ |mut map, &Receiver{role:item,..}| {
                map.insert(item, 1 + map.get(&item).unwrap_or(&0));
                return map;
            }
            );

        // position sample sidebadge
        let badge_spacing_y = Mm(15.0);
        draw_sidebadges(&current_layer, &font_calibri, badge_text_font_size,
                        (border_wh, border_wh), badge_spacing_y,
                        rolecount_dict);

    }

    return doc;
}

#[deprecated(note="use generate_couverts instead")]
fn couvert_doc(receivers: Vec<Receiver>, address: Vec<&str>) -> printpdf::PdfDocumentReference {
    use printpdf::*;

    // document config
    let document_title = "Versand";
    let address_font_size = 18;
    let names_font_size = 11;
    let badge_text_font_size = 11;
    let page_width = Mm(229.0);
    let page_height = Mm(162.0);
    let address_offset_x = Mm(120.0);
    let address_offset_y = Mm(65.0);
    let border_wh = Mm(12.0);
    let names_offset_x = border_wh + Mm(20.0);
    let names_offset_y = page_height - Mm(18.0);

    // create the document
    let (doc, page1, layer1) : (PdfDocumentReference, indices::PdfPageIndex, indices::PdfLayerIndex) = PdfDocument::new(document_title, page_width, page_height, /*initial_layer_name*/"Layer 1");
    // load a font
    let font_calibri = doc.add_external_font(std::fs::File::open("res/fonts/calibri.ttf").unwrap()).unwrap();
    let font_calibri_light = doc.add_external_font(std::fs::File::open("res/fonts/calibril.ttf").unwrap()).unwrap();


    for i in 0..=100 {

        // add new page
        println!("Generating page {}/100", i);
        let (next_page, layer1) = doc.add_page(page_width, page_height, "Page 2, Layer 1");

        // prepare usage variables
        let current_page = doc.get_page(next_page);
        let current_layer = current_page.get_layer(layer1);

        // place the logo first, so that it is in the background
        // original logo is at 300 dpi approx 16/0.15 = 106mm
        add_bitmap_to_layer(&current_layer,
                            Some(border_wh), Some(page_height - Mm(16.0) - border_wh ),
                            Some(0.15), Some(0.15)
                           );

        // draw names
        draw_names(&current_layer, &font_calibri, names_font_size, (names_offset_x, names_offset_y),
        receivers.iter().map(|r:&Receiver| (&r.nickname as &str, &r.group as &str))
        );

        // position sample address
        {
            let font_addresses = font_calibri_light.clone();
            current_layer.begin_text_section();
            current_layer.set_font(&font_addresses, address_font_size);
            current_layer.set_text_cursor(address_offset_x, address_offset_y);
            current_layer.set_line_height(address_font_size);
            current_layer.set_word_spacing(3000);
            current_layer.set_character_spacing(0);
            current_layer.set_text_rendering_mode(/*Fill, Stroke, FillStroke, Invisible, FillClip, StrokeClip, FillStrokeClip, Clip*/TextRenderingMode::Fill);

            for line in &address {
                current_layer.write_text(line.clone(), &font_addresses);
                current_layer.add_line_break();
            }

            current_layer.end_text_section();
        }

        // numbers in sidebadge
        let rolecount_dict : HashMap<Role, usize> = receivers.iter().fold(
            /*init:*/ HashMap::new(),
            /*f(map, item):*/ |mut map, &Receiver{role:item,..}| {
                map.insert(item, 1 + map.get(&item).unwrap_or(&0));
                return map;
            }
            );

        // position sample sidebadge
        let badge_spacing_y = Mm(15.0);
        draw_sidebadges(&current_layer, &font_calibri, badge_text_font_size,
                        (border_wh, border_wh), badge_spacing_y,
                        rolecount_dict);

    }

    return doc;
} 

fn draw_names<'a> (current_layer: &printpdf::PdfLayerReference,
               font: &printpdf::IndirectFontRef,
               font_size: i64,
               (start_x, start_y): (printpdf::Mm, printpdf::Mm),
               names_and_groups: impl Iterator<Item=(&'a str, &'a str)> + Clone){
    let line_distance_y = printpdf::Mm(5.0);

    let names_str = names_and_groups.clone()
        .map(|(name, _group)| name)
        .collect::<Vec<&str>>().join(", ");

    let groups_str = names_and_groups
        .map(|(_name, group)| group)
        .collect::<Vec<&str>>().join(", ");

    // position the names
    current_layer.use_text(
        names_str,
        font_size,
        start_x, start_y,
        &font);

    // position the group names
    current_layer.use_text(
        groups_str,
        font_size,
        start_x, start_y - line_distance_y,
        &font);

}

fn draw_sidebadges (current_layer: &printpdf::PdfLayerReference,
                   font: &printpdf::IndirectFontRef,
                   font_size: i64,
                   (start_x, start_y): (printpdf::Mm, printpdf::Mm),
                   badge_spacing_y: printpdf::Mm,
                   numbers: HashMap<Role,usize>) {
    
    let mut y = start_y;
    for (role, num) in numbers {
        let txt: String = role.value().for_num(num);
        let text = format!("{} {}", num, txt);
        draw_sidebadge(&current_layer, start_x, y,
                       &font, font_size, &text);
        y += badge_spacing_y;
    }
}

/// overwrites the fill color of the current layer and draws a badge at (origin_x, origin_y)
fn draw_sidebadge (current_layer: &printpdf::PdfLayerReference,
                   origin_x: printpdf::Mm,
                   origin_y: printpdf::Mm,
                   font: &printpdf::IndirectFontRef,
                   font_size: i64,
                   text: &str) {
    use printpdf::{Point, Line, Mm};

    let badge_height = 10.0;
    let badge_width = 30.0;
    let badge_dent_width = badge_width / 10.0;

    // point relative to lower left corner (pos_x, pos_y)
    let point = |posx: f64, posy: f64| -> Point {
        let printpdf::Mm(pos_x) = origin_x;
        let printpdf::Mm(pos_y) = origin_y;
        Point::new(Mm(pos_x + posx), Mm(pos_y + posy))
    };

    // The "false" determines if the next (following)
    // point is a bezier handle (for curves)
    // If you want holes, simply reorder the winding of the points to be
    // counterclockwise instead of clockwise.
    let points1 = vec![
        (point(0.0, badge_height), false),
        (point(badge_width, badge_height), false),
        (point(badge_width - badge_dent_width, badge_height/2.0), false),
        (point(badge_width, 0.0), false),
        (point(0.0, 0.0), false)
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
    let fill_color_black = printpdf::Color::Cmyk(printpdf::Cmyk::new(0.0, 0.0, 0.0, 1.0, None));
    current_layer.set_fill_color(fill_color_black);
    current_layer.add_shape(line1);

    // create text
    let fill_color_white = printpdf::Color::Cmyk(printpdf::Cmyk::new(0.0, 0.0, 0.0, 0.0, None));
    current_layer.set_fill_color(fill_color_white);
    current_layer.use_text(text, font_size, origin_x + Mm(2.5), origin_y + Mm(badge_height/2.0) - Mm(0.8), &font);
}

fn add_bitmap_to_layer(current_layer : &printpdf::PdfLayerReference, 
                       posx : Option<printpdf::Mm>,
                       posy : Option<printpdf::Mm>,
                       scalex : Option<f64>,
                       scaley : Option<f64>) {
    use printpdf::*;
    use image::bmp::BMPDecoder;
    let mut image_file = std::fs::File::open("res/images/logo.bmp").unwrap();
    let decoder = BMPDecoder::new(&mut image_file).unwrap();
    let image = Image::try_from(decoder).unwrap();
    // translate x, translate y, rotate, scale x, scale y, dpi
    image.add_to_layer(current_layer.clone(), posx, posy, None, scalex, scaley, None);
}

#[allow(dead_code)]
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

#[derive(Debug, Clone)]
pub struct Receiver {
    pub nickname: String,
    pub group: String,
    pub role: Role,
}

/// Used for the sidebadges
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Role {
    Leiter,
    Teilnehmer,
    Traegerkreis,
    Ehemalige,
}

impl Role {
    fn value(&self) -> impl pluralizable::Pluralizable {
        let t = |sin: &str, plu: &str| pluralizable::Text::new(sin, plu);
        match *self {
            Role::Leiter =>  t("Leiter", "Leiter"),
            Role::Teilnehmer => t("Teilnehmer", "Teilnehmer"),
            Role::Traegerkreis => t("Trägerkreis", "Trägerkreis"),
            Role::Ehemalige => t("Ehemaliger", "Ehemalige"),
        }
    }

    fn get_text_for_num(&self, num: usize) -> String {
        self.value().for_num(num)
    }

    fn values() -> Vec<Role> {
        return vec![Role::Leiter, Role::Teilnehmer, Role::Traegerkreis, Role::Ehemalige];
    }
}

