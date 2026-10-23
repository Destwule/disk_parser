pub trait Row {
    fn to_row(&self) -> Vec<String>;
}

pub trait Table {
    type Entry: Row;

    fn entries_widths(table_headings: &[&str], entries: &[Self::Entry]) -> Vec<usize> {
        let mut widths: Vec<usize> = table_headings.iter().map(|h| h.len()).collect();
        for entry in entries {
            for (i, value) in entry.to_row().iter().enumerate() {
                widths[i] = widths[i].max(value.len());
            }
        }
        widths
    }

    fn display_table(table_headings: &[&str], entries: &Vec<Self::Entry>) {
        let widths = Self::entries_widths(table_headings, entries);

        for (i, heading) in table_headings.iter().enumerate() {
            print!("{heading:<w$} ", w = widths[i]);
        }
        println!();

        for entry in entries {
            for (i, value) in entry.to_row().iter().enumerate() {
                print!("{value:<w$} ", w = widths[i]);
            }
            println!();
        }
    }
}