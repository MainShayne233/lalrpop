use ascii_canvas::AsciiView;
use std::cmp;
use super::*;

#[derive(Debug)]
pub struct Wrap {
    items: Vec<Box<Content>>
}

impl Wrap {
    pub fn new(items: Vec<Box<Content>>) -> Self {
        let mut wrap_items = vec![];
        for item in items {
            item.into_wrap_items(&mut wrap_items);
        }
        Wrap { items: wrap_items }
    }
}

impl Content for Wrap {
    fn min_width(&self) -> usize {
        self.items.iter()
                  .map(|c| c.min_width())
                  .max()
                  .unwrap()
    }

    fn emit(&self, view: &mut AsciiView) {
        let columns = view.columns();
        let mut row = 0;    // current row
        let mut height = 1; // max height of anything in this row
        let mut column = 0; // current column in this row

        for item in &self.items {
            let len = item.min_width();

            println!("column={} len={} columns={}",
                     column, len, columns);

            // If we don't have enough space for this content,
            // then move to the next line.
            if column + len > columns {
                column = 0;
                row += height;
                height = 1;
            }

            assert!(column + len <= columns);

            let (c_row, c_column) = item.emit_at(view, row, column);
            assert!(c_column >= column);
            column = c_column + 2;
            height = cmp::max(c_row - row + 1, height);

            println!("item={:?} c_row={} c_column={} column={} height={}",
                     item, c_row, c_column, column, height);
        }
    }

    fn into_wrap_items(self: Box<Self>, wrap_items: &mut Vec<Box<Content>>) {
        wrap_items.extend(self.items); // `items` are already subdivided
    }
}
