//#[macro_use] extern crate clap;

use genpdf::{Document, fonts, PaperSize, elements::{Text, Paragraph, TableLayout, LinearLayout, FrameCellDecorator}, Margins, SimplePageDecorator, style::{Style, StyledString}, error::Error, Element};
use clap::Parser;

mod parsing_arguments;

use crate::parsing_arguments::TerminalArguments;

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
    let args = TerminalArguments::parse();
    assert!(&args.bill_date < &args.payment_date);
    println!("{:?}", args);

    let font_family = fonts::from_files("/Users/programmerskill/Desktop/rechnungsgenerator/fonts", "LiberationSerif", None).unwrap();
    let mut doc = Document::new(font_family);
    doc.set_paper_size(PaperSize::A4);
    let mut page_decorator = SimplePageDecorator::new();
    page_decorator.set_margins(Margins::all(20.0));
    doc.set_page_decorator(page_decorator);

    let mut header_table = TableLayout::new(vec![2, 1, 2]);
    header_table.set_cell_decorator(FrameCellDecorator::new(false, false, false));

    add_table_row_with_text(&mut header_table, vec![&args.sender_name, "", &args.bill_date.format("%d.%m.%Y").to_string()], vec![Style::new(); 3], false).unwrap();
    add_table_row_with_text(&mut header_table, vec![format!("{}\n{}\n\n\n\n\n\n\n\n", &args.sender_address, &args.sender_zip_code_and_city).as_str(), "", format!("\n\n\n\nRechnungsnummer: {}\nKundennummer: {}\nSteuernummer: {}\nEmail: {}", &args.bill_id, &args.customer_id, &args.sender_tax_number.unwrap_or(String::new()), &args.sender_email_address.unwrap_or(String::new())).as_str()], vec![Style::new(); 3], false).unwrap();
    add_table_row_with_text(&mut header_table, vec![format!("{}\n{}\n{}\n\n", &args.recipient_name, &args.recipient_address, &args.recipient_zip_code_and_city).as_str(), "", "\n\n\n\n\n\n"],vec![Style::new(); 3], false).unwrap();

    doc.push(header_table);

    write_text(&mut doc, "RECHNUNG\n", Style::new().bold());
    write_text(&mut doc, "Sehr geehrter Kunde,\n\nwie vereinbart berechne ich für meine Leistung wie folgt:\n", Style::new());

    let mut product_table = TableLayout::new(vec![4, 1]);
    product_table.set_cell_decorator(FrameCellDecorator::new(true, true, false));

    add_table_row_with_text(&mut product_table, vec!["Softwareleistung", "Preis "], vec![Style::new(); 2], true).unwrap();
    //add_table_row_with_text(&mut product_table, vec!["Wetterapp (iOS und Android)\nAktueller Stundensatz: 17 Stunden a 50 €", "510  €"], vec![Style::new(); 2], true).unwrap();
    //add_table_row_with_text(&mut product_table, vec!["Telegram Bot (Linux)\nAktueller Stundensatz: 1 Stunden a 50 €", "30   €"], vec![Style::new(); 2], true).unwrap();
    //add_table_row_with_text(&mut product_table, vec!["PasswordCardCreator (Linux)\nAktueller Stundensatz: 19 Stunden a 50 €", "950  €"], vec![Style::new(); 2], true).unwrap();
    /*for product in &args.products {
        add_table_row_with_text(&mut product_table, vec![product.description.as_str(), product.price.as_str()], vec![Style::new(); 2], true).unwrap();
    }*/
    add_table_row_with_text(&mut product_table, vec!["Rechnungsbetrag", "0 €"], vec![Style::new(); 2], true).unwrap();

    doc.push(product_table);

    write_text(&mut doc, format!("\nDer Betrag ist bitte bis zum {} auf das, unten stehende, Konto zu überweisen.\n\nHochachtungsvoll\n\n\n{}", &args.payment_date.format("%d.%m.%Y").to_string(), &args.sender_name).as_str(), Style::new());
    write_text(&mut doc, format!("\n\nBankverbindung:\n{}\nIBAN: {}\nBIC: {}", &args.sender_bank, &args.sender_iban, &args.sender_bic).as_str(), Style::new());

    doc.render_to_file("rechnung.pdf").unwrap();
}