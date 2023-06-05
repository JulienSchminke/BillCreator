use std::fs::File;
use std::io::BufWriter;
use printpdf::{PdfDocument, Mm, BuiltinFont, PdfLayerReference, IndirectFontRef, Point, Line};
use chrono::offset::Local;

const SENDER_CITY: &str = "Herne";


fn add_current_date(layer: PdfLayerReference, font: &IndirectFontRef) -> PdfLayerReference {
    let mut current_date = Local::now().date_naive().format(", den %d.%m.%Y").to_string();
    current_date.insert_str(0, SENDER_CITY);
    layer.begin_text_section();
    layer.set_text_cursor(Mm(155.0), Mm(288.0));
    layer.write_text(current_date, font);
    layer.end_text_section();
    layer
}

fn write_line(layer: PdfLayerReference, font: &IndirectFontRef, line: &str) -> PdfLayerReference {
    layer.write_text(line, font);
    layer.add_line_break();
    layer
}

fn write_text(mut layer: PdfLayerReference, font: &IndirectFontRef, full_text: &str) -> PdfLayerReference {
    for line in full_text.split("\n").collect::<Vec<&str>>().iter() {
        layer = write_line(layer, font, *line);
    }
    layer
}

fn create_table(layer: PdfLayerReference, font: &IndirectFontRef, text: Vec<Vec<&str>>, column_edge_ordinates: Vec<Mm>, top_border: Mm, bottom_border: Mm) -> PdfLayerReference {
    for text_row in text {
        assert_eq!(text_row.len(), column_edge_ordinates.len()-1);
    }
    assert_eq!(top_border > bottom_border, true);

    //let left_border  = column_edge_ordinates.iter().min().unwrap();
    //let right_border = column_edge_ordinates.iter().max().unwrap();
    //let  left_line  = Line { points: vec![(Point::new(Mm(20.0 ), Mm(142.0)), false), (Point::new(Mm(20.0 ), Mm(190.0)), false)], is_closed: false, has_fill: false, has_stroke: true, is_clipping_path: false, };
    //let middle_line = Line { points: vec![(Point::new(Mm(160.0), Mm(142.0)), false), (Point::new(Mm(160.0), Mm(190.0)), false)], is_closed: false, has_fill: false, has_stroke: true, is_clipping_path: false, };
    //let  right_line = Line { points: vec![(Point::new(Mm(190.0), Mm(142.0)), false), (Point::new(Mm(190.0), Mm(190.0)), false)], is_closed: false, has_fill: false, has_stroke: true, is_clipping_path: false, };
    let mut lines = Vec::new();
    //vertical lines
    for vertical_line_x_ordinate in column_edge_ordinates {
        lines.push(Line {
            points: vec![(Point::new(vertical_line_x_ordinate, top_border), false), (Point::new(vertical_line_x_ordinate, bottom_border), false)],
            is_closed: false,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        });
    }

    let mut horizontal_y_ordinates = Vec::new();
    for row in text {

    }

    layer
}

fn main() {
    let (document, page_index, layer_index) = PdfDocument::new("Rechnung", Mm(210.0), Mm(297.0), "Ebene");
    let font = document.add_builtin_font(BuiltinFont::TimesRoman).unwrap();
    let subject_line_font  = document.add_builtin_font(BuiltinFont::TimesBold).unwrap();

    let mut current_layer = document.get_page(page_index).get_layer(layer_index);

    //sender section
    current_layer.begin_text_section();
    current_layer.set_font(&font, 12.0);
    current_layer.set_text_cursor(Mm(20.0), Mm(288.0));
    current_layer.set_line_height(15.0);
    current_layer = write_text(current_layer, &font, "Max Sendemann\nSendestraße 101\n45678 Sendestadt");

    //recipient section
    for _ in 0..7 { current_layer.add_line_break(); }
    current_layer = write_text(current_layer, &font, "Mustermann Consulting\nMusterstraße 10\n44444 Musterstadt\n");

    //subject line
    current_layer.set_font(&subject_line_font, 12.0);
    current_layer = write_text(current_layer, &subject_line_font, "RECHNUNG\n");
    current_layer.set_font(&font, 12.0);
    current_layer = write_text(current_layer, &font, "Sehr geehrter Kunde,\n\nwie vereinbart berechne ich für meine Leistung wie folgt:");
    current_layer.end_text_section();

    //City and date
    current_layer = add_current_date(current_layer, &font);

    //Table
    //vertical lines
    let  left_line  = Line { points: vec![(Point::new(Mm(20.0 ), Mm(142.0)), false), (Point::new(Mm(20.0 ), Mm(190.0)), false)], is_closed: false, has_fill: false, has_stroke: true, is_clipping_path: false, };
    let middle_line = Line { points: vec![(Point::new(Mm(160.0), Mm(142.0)), false), (Point::new(Mm(160.0), Mm(190.0)), false)], is_closed: false, has_fill: false, has_stroke: true, is_clipping_path: false, };
    let  right_line = Line { points: vec![(Point::new(Mm(190.0), Mm(142.0)), false), (Point::new(Mm(190.0), Mm(190.0)), false)], is_closed: false, has_fill: false, has_stroke: true, is_clipping_path: false, };
    //horizontal lines
    let first  = Line { points: vec![(Point::new(Mm(20.0 ), Mm(142.0)), false), (Point::new(Mm(190.0), Mm(142.0)), false)], is_closed: false, has_fill: false, has_stroke: true, is_clipping_path: false, };
    let second = Line { points: vec![(Point::new(Mm(20.0 ), Mm(148.0)), false), (Point::new(Mm(190.0), Mm(148.0)), false)], is_closed: false, has_fill: false, has_stroke: true, is_clipping_path: false, };
    let third  = Line { points: vec![(Point::new(Mm(20.0 ), Mm(160.0)), false), (Point::new(Mm(190.0), Mm(160.0)), false)], is_closed: false, has_fill: false, has_stroke: true, is_clipping_path: false, };
    let fourth = Line { points: vec![(Point::new(Mm(20.0 ), Mm(172.0)), false), (Point::new(Mm(190.0), Mm(172.0)), false)], is_closed: false, has_fill: false, has_stroke: true, is_clipping_path: false, };
    let fifth  = Line { points: vec![(Point::new(Mm(20.0 ), Mm(184.0)), false), (Point::new(Mm(190.0), Mm(184.0)), false)], is_closed: false, has_fill: false, has_stroke: true, is_clipping_path: false, };
    let sixth  = Line { points: vec![(Point::new(Mm(20.0 ), Mm(190.0)), false), (Point::new(Mm(190.0), Mm(190.0)), false)], is_closed: false, has_fill: false, has_stroke: true, is_clipping_path: false, };

    current_layer.add_shape(left_line);
    current_layer.add_shape(middle_line);
    current_layer.add_shape(right_line);
    current_layer.add_shape(first);
    current_layer.add_shape(second);
    current_layer.add_shape(third);
    current_layer.add_shape(fourth);
    current_layer.add_shape(fifth);
    current_layer.add_shape(sixth);

    //content
    current_layer.use_text("Softwareleistung", 12.0, Mm(22.0 ), Mm(186.0), &font);
    current_layer.use_text("Preis"           , 12.0, Mm(162.0), Mm(186.0), &font);

    current_layer.use_text("Wetterapp (iOS und Android)", 12.0, Mm(22.0), Mm(180.0), &font);
    current_layer.use_text("Aktueller Stundensatz: 17 Stunden a 50 €", 12.0, Mm(22.0), Mm(174.0), &font);

    current_layer.use_text("Telegram Bot (Linux)", 12.0, Mm(22.0), Mm(168.0), &font);
    current_layer.use_text("Aktueller Stundensatz: 1 Stunden a 50 €", 12.0, Mm(22.0), Mm(162.0), &font);

    current_layer.use_text("PasswordCardCreator (Linux)", 12.0, Mm(22.0), Mm(156.0), &font);
    current_layer.use_text("Aktueller Stundensatz: 19 Stunden a 50 €", 12.0, Mm(22.0), Mm(150.0), &font);

    current_layer.use_text("Rechnungsbetrag", 12.0, Mm(22.0), Mm(144.0), &font);

    current_layer.use_text("510 €", 12.0, Mm(162.0), Mm(180.0), &font);
    current_layer.use_text("30 €", 12.0, Mm(162.0), Mm(168.0), &font);
    current_layer.use_text("950 €", 12.0, Mm(162.0), Mm(156.0), &font);
    current_layer.use_text("1490 €", 12.0, Mm(162.0), Mm(144.0), &font);

    current_layer.begin_text_section();
    current_layer.set_text_cursor(Mm(20.0), Mm(130.0));
    current_layer = write_text(current_layer, &font, "Der Betrag ist bitte bis zum 21.04.2023 auf das, unten stehende, Konto zu überweisen.\n\nHochachtungsvoll\n\n\nJulien Schminke");
    current_layer.end_text_section();

    current_layer.begin_text_section();
    current_layer.set_text_cursor(Mm(20.0), Mm(50.0));
    current_layer = write_text(current_layer, &font, "Bankverbindung:\nMUSTER-BANK West\nIBAN: DE54 4654 5674 5656 5468 45\nBIC: AASDFKL3SAD");
    current_layer.end_text_section();

    current_layer.begin_text_section();
    current_layer.set_text_cursor(Mm(120.0), Mm(260.0));
    current_layer = write_text(current_layer, &font, "Rechnungsnummer: 00,00,0001\nKundennummer: PG 01212\nSteuernummer: 5646545646\nEmail: julien.email@gmail.com");
    current_layer.end_text_section();

    document.save(&mut BufWriter::new(File::create("rechnung.pdf").unwrap())).unwrap()
}
