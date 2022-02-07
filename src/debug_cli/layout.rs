pub trait Print {
    fn print(&self) -> Vec<String>;
}

struct Widget<'a> {
    item: Box<dyn Print + 'a>,
    col: usize,
}

pub struct Layout<'a> {
    widgets: Vec<Widget<'a>>,
}

impl<'a> Layout<'a> {
    pub fn new() -> Layout<'a> {
        Layout {
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Print + 'a>, col: usize) {
        self.widgets.push(Widget { item: widget, col })
    }

    pub fn draw(&self) {
        // Print each of the widgets and store with col
        let widgets = self
            .widgets
            .iter()
            .map(|w| (w.item.print(), w.col))
            .collect::<Vec<(Vec<String>, usize)>>();

        // Work out the max number of lines needed
        let get_max_size = |column: usize| -> usize {
            widgets
                .iter()
                .filter_map(|(lines, col)| {
                    if *col == column {
                        Some(lines.len())
                    } else {
                        None
                    }
                })
                .sum()
        };

        let col_0_size = get_max_size(0);
        let col_1_size = get_max_size(1);
        let max_lines = std::cmp::max(col_0_size, col_1_size);

        // Create a buffer with that many lines in it. Each line is 80 chars wide
        let buffer_line = " ".repeat(80);
        let mut buffer = vec![buffer_line; max_lines];

        // Iterate through all the column 0 widgets and print them into the buffer.
        // Work out the max width as we go
        let mut col_0_width = 0;
        let mut index = 0;
        for (data, col) in widgets.iter() {
            if *col == 0 {
                for line in data.iter() {
                    col_0_width = std::cmp::max(col_0_width, line.len());
                    buffer[index].replace_range(..line.len(), line);
                    index += 1;
                }
            }
        }

        // Print out the second row into the buffer using the col_0 width as an offset.
        index = 0;
        for (data, col) in widgets.iter() {
            if *col == 1 {
                for line in data.iter() {
                    buffer[index].replace_range(col_0_width..(col_0_width + line.len()), line);
                    index += 1;
                }
            }
        }

        // Actually print out the buffer
        buffer.iter().for_each(|l| println!("{}", l));
    }
}
