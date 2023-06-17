use genpdf::{Document, fonts, PaperSize, elements::{Text, Paragraph, TableLayout, LinearLayout, FrameCellDecorator}, Margins, SimplePageDecorator, style::{Style, StyledString}, error::Error, Element};

fn write_text(doc: &mut Document, text: &str, text_style: Style) {
    for text_line in text.split("\n").collect::<Vec<&str>>().iter() {
        doc.push(Text::new(StyledString::new(*text_line, text_style)));
    }
}

fn add_table_row_with_text(table: &mut TableLayout, text_for_columns: Vec<&str>, style_for_columns: Vec<Style>, shall_linear_layout_be_padded: bool) -> Result<(), Error> {
    assert_eq!(text_for_columns.len(), style_for_columns.len());
    let mut table_row = table.row();
    for i in 0..text_for_columns.len() {
        let mut linear_layout = LinearLayout::vertical();
        for text_line_for_column in text_for_columns[i].split("\n").collect::<Vec<&str>>().iter() {
            linear_layout.push(Paragraph::new(StyledString::new(*text_line_for_column, style_for_columns[i])));
        }
        if shall_linear_layout_be_padded {
            table_row.push_element(linear_layout.padded(1));
        } else {
            table_row.push_element(linear_layout);
        }

    }
    table_row.push()
}

fn main() {
    let font_family = fonts::from_files("./fonts", "LiberationSerif", None).unwrap();
    let mut doc = Document::new(font_family);
    doc.set_paper_size(PaperSize::A4);
    let mut page_decorator = SimplePageDecorator::new();
    page_decorator.set_margins(Margins::all(20.0));
    doc.set_page_decorator(page_decorator);

    let mut header_table = TableLayout::new(vec![1, 2, 2]);
    header_table.set_cell_decorator(FrameCellDecorator::new(false, false, false));

    add_table_row_with_text(&mut header_table, vec!["Max Sendemann", "", "das heutige Datum"], vec![Style::new(); 3], false).unwrap();
    add_table_row_with_text(&mut header_table, vec!["Sendestraße 101\n45678 Sendestadt\n\n\n\n\n\n\n\n", "", "\n\n\n\nRechnungsnummer: 00,00,0001\nKundennummer: PG 01212\nSteuernummer: 5646545646\nEmail: max.sendemann@gmail.com"], vec![Style::new(); 3], false).unwrap();
    add_table_row_with_text(&mut header_table, vec!["Mustermann Consulting\nMusterstraße 10\n44444 Musterstadt\n\n", "", "\n\n\n\n\n\n"],vec![Style::new(); 3], false).unwrap();

    doc.push(header_table);

    write_text(&mut doc, "RECHNUNG\n", Style::new().bold());
    write_text(&mut doc, "Sehr geehrter Kunde,\n\nwie vereinbart berechne ich für meine Leistung wie folgt:\n", Style::new());

    let mut product_table = TableLayout::new(vec![4, 1]);
    product_table.set_cell_decorator(FrameCellDecorator::new(true, true, false));

    add_table_row_with_text(&mut product_table, vec!["Softwareleistung", "Preis "], vec![Style::new(); 2], true).unwrap();
    add_table_row_with_text(&mut product_table, vec!["Wetterapp (iOS und Android)\nAktueller Stundensatz: 17 Stunden a 50 €", "510  €"], vec![Style::new(); 2], true).unwrap();
    add_table_row_with_text(&mut product_table, vec!["Telegram Bot (Linux)\nAktueller Stundensatz: 1 Stunden a 50 €", "30   €"], vec![Style::new(); 2], true).unwrap();
    add_table_row_with_text(&mut product_table, vec!["PasswordCardCreator (Linux)\nAktueller Stundensatz: 19 Stunden a 50 €", "950  €"], vec![Style::new(); 2], true).unwrap();
    add_table_row_with_text(&mut product_table, vec!["Rechnungsbetrag", "1490 €"], vec![Style::new(); 2], true).unwrap();

    doc.push(product_table);

    write_text(&mut doc, "\nDer Betrag ist bitte bis zum 21.04.2023 auf das, unten stehende, Konto zu überweisen.\n\nHochachtungsvoll\n\n\nMax Sendemann", Style::new());
    write_text(&mut doc, "\n\nBankverbindung:\nMUSTER-BANK West\nIBAN: DE54 4654 5674 5656 5468 45\nBIC: AASDFKL3SAD", Style::new());

    doc.render_to_file("rechnung.pdf").unwrap();
}