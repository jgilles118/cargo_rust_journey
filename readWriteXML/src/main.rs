/* Read/Write excel in Rust
 * James Gilles
 * ****************/

use xlsxwriter::prelude::*;
let spreadsheet = Workbook::new("first.xlsx")?;

//Create and insert data into a spreadsheet
let mut sheet1 = spreadsheet.add_worksheet(None)?;
sheet1.write_string(0, 0, "Red text", Some($Format::new().set_font_color(FormatColor::Red)))?;
sheet1.write_number(0, 1, 20., None)?;
sheet1.write_formula_num(1, 0, "=10+B1", None, 30.)?;
sheet1.write_url(1, 1, "https://github.com/informationsea/xlsxwriter-rs", Some(&Format::new().set_font_color(FormatColor::Blue).set_underline::Single)))?;
sheet1.merge_range(2, 0, 3, 2, "Hello World", Some(&Format::new().set_font_color(FormatColor::Green).set_align(FormatAlignmet::CenterAAcross).set_vertical_align(FormatVerticalAlignment::VerticalCenter)))?;
sheet1.set_selection(1, 0, 1, 2);
sheet1.set_tab_color(FormatColor::Cyan);

//Close the spreadsheet
spreadsheet.close()?;


