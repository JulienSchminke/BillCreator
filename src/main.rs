use genpdf::{Document, fonts, PaperSize, elements::{Text, TableLayout, LinearLayout, FrameCellDecorator}, Margins, SimplePageDecorator, style::{Style, StyledString}, error::Error, Element};

fn write_text(doc: &mut Document, text: &str, text_style: Style) {
    for text_line in text.split("\n").collect::<Vec<&str>>().iter() {
        doc.push(Text::new(StyledString::new(*text_line, text_style)));
    }
}

fn add_table_row_with_text(table: &mut TableLayout, text_for_columns: Vec<&str>, style_for_columns: Vec<Style>) -> Result<(), Error> {
    assert_eq!(text_for_columns.len(), style_for_columns.len());
    let mut table_row = table.row();
    for i in 0..text_for_columns.len() {
        let mut linear_layout = LinearLayout::vertical();
        for text_line_for_column in text_for_columns[i].split("\n").collect::<Vec<&str>>().iter() {
            linear_layout.push(Text::new(StyledString::new(*text_line_for_column, style_for_columns[i])));
        }
        table_row.push_element(linear_layout.padded(1));
    }
    table_row.push()
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
    write_text(&mut doc, "Sehr geehrter Kunde,\n\nwie vereinbart berechne ich für meine Leistung wie folgt:\n", Style::new());

    let mut table = TableLayout::new(vec![4, 1]);
    table.set_cell_decorator(FrameCellDecorator::new(true, true, true));

    add_table_row_with_text(&mut table, vec!["Softwareleistung"                                                     , "Preis "], vec![Style::new(); 2]).unwrap();
    add_table_row_with_text(&mut table, vec!["Wetterapp (iOS und Android)\nAktueller Stundensatz: 17 Stunden a 50 €", "510  €"], vec![Style::new(); 2]).unwrap();
    add_table_row_with_text(&mut table, vec!["Telegram Bot (Linux)\nAktueller Stundensatz: 1 Stunden a 50 €"        , "30   €"], vec![Style::new(); 2]).unwrap();
    add_table_row_with_text(&mut table, vec!["PasswordCardCreator (Linux)\nAktueller Stundensatz: 19 Stunden a 50 €", "950  €"], vec![Style::new(); 2]).unwrap();
    add_table_row_with_text(&mut table, vec!["Rechnungsbetrag"                                                      , "1490 €"], vec![Style::new(); 2]).unwrap();

    doc.push(table);

    write_text(&mut doc, "Der Betrag ist bitte bis zum 21.04.2023 auf das, unten stehende, Konto zu überweisen.\n\nHochachtungsvoll\n\n\nMax Sendemann", Style::new());

    doc.render_to_file("rechnung.pdf").unwrap();
}