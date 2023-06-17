use genpdf::{Document, fonts, PaperSize, elements::{Text, Break, TableLayout}, Margins, SimplePageDecorator, style::{Style, StyledString}};

fn write_text(doc: &mut Document, text: &str, text_style: Style) {
    for text_line in text.split("\n").collect::<Vec<&str>>().iter() {
        doc.push(Text::new(StyledString::new(*text_line, text_style)));
    }
}

fn main() {
    let font_family = fonts::from_files("./fonts", "LiberationSans", None).unwrap();
    let mut doc = Document::new(font_family);
    doc.set_paper_size(PaperSize::A4);
    let mut page_decorator = SimplePageDecorator::new();
    page_decorator.set_margins(Margins::all(20.0));
    doc.set_page_decorator(page_decorator);

    write_text(&mut doc, "Max Sendemann\nSendestraße 101\n45678 Sendestadt\n\n\n\n\n\n\n\n", Style::new());
    write_text(&mut doc, "Mustermann Consulting\nMusterstraße 10\n44444 Musterstadt\n", Style::new());
    write_text(&mut doc, "RECHNUNG\n", Style::new().bold());
    write_text(&mut doc, "Sehr geehrter Kunde,\n\nwie vereinbart berechne ich für meine Leistung wie folgt:", Style::new());

    let mut table = TableLayout::new(vec![4, 1]);
    table.set_cell_decorator(FramedCellDecorator::new(true, true, true));


    doc.render_to_file("rechnung.pdf").unwrap();
}