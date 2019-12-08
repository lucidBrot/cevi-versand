use std::slice::{Iter, IterMut};

/// Used to build a row, specifying additional details.
#[derive(Default)]
pub struct RowBuilder {
    height: RowHeight,
    min_height: f64,
    max_height: f64,
}

impl RowBuilder {
    /// Creates a new `RowBuilder` with default values.
    pub fn new() -> Self {
        RowBuilder::default()
    }

    /// Inserts a new height.
    pub fn height(mut self, height: RowHeight) -> Self {
        self.height = height;
        self
    }

    /// Inserts a new min height.
    pub fn min_height(mut self, min_height: f64) -> Self {
        self.min_height = min_height;
        self
    }

    /// Inserts a new max height.
    pub fn max_height(mut self, max_height: f64) -> Self {
        self.max_height = max_height;
        self
    }

    /// Builds the row.
    pub fn build(self) -> Row {
        Row {
            height: self.height,
            min_height: self.min_height,
            max_height: self.max_height,
            current_height: 0.0,
        }
    }
}

/// Used to define the row of the `Grid`.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Row {
    pub height: RowHeight,
    pub min_height: f64,
    pub max_height: f64,
    current_height: f64,
}

impl Row {
    /// Creates a new `RowBuilder` object with default values.
    pub fn create() -> RowBuilder {
        RowBuilder::new()
    }

    /// Gets the row height.
    pub fn height(&self) -> RowHeight {
        self.height
    }

    /// Gets the current height.
    pub fn current_height(&self) -> f64 {
        self.current_height
    }

    /// Sets the current height.
    pub fn set_current_height(&mut self, height: f64) {
        self.current_height = if self.min_height == 0.0 && self.max_height == 0.0 && height > 0.0 {
            height
        } else if height < self.min_height && self.min_height > 0.0 {
            self.min_height
        } else if height > self.max_height && self.max_height > 0.0 {
            self.max_height
        } else {
            height
        };
    }
}

impl From<&str> for Row {
    fn from(t: &str) -> Self {
        match t {
            "Auto" | "auto" => Row::create().height(RowHeight::Auto).build(),
            _ => Row::create().height(RowHeight::Stretch).build(),
        }
    }
}

impl From<f64> for Row {
    fn from(t: f64) -> Self {
        Row::create().height(RowHeight::Height(t)).build()
    }
}

/// Used to define the height of a grid row.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RowHeight {
    /// Row is measured by the highest child.
    Auto,

    /// Column expands to the rest available height.
    Stretch,

    /// Defines a fixed size for the row.
    Height(f64),
}

impl Default for RowHeight {
    fn default() -> Self {
        RowHeight::Stretch
    }
}

#[derive(Default)]
pub struct RowsBuilder {
    row_definitions: Vec<Row>,
}

/// Used to build a rows, specifying additional details.
impl RowsBuilder {
    /// Creates a new `RowsBuilder` with default values.
    pub fn new() -> Self {
        RowsBuilder::default()
    }

    /// Inserts a new row.
    pub fn row<R: Into<Row>>(mut self, row: R) -> Self {
        self.row_definitions.push(row.into());
        self
    }

    /// Inserts a list of rows.
    pub fn rows<R: Into<Row> + Clone>(mut self, rows: &[R]) -> Self {
        for row in rows.to_vec() {
            self.row_definitions.push(row.into());
        }
        self
    }

    /// Inserts the given row as often as given.
    pub fn repeat<R: Into<Row> + Copy>(mut self, row: R, count: usize) -> Self {
        for _ in 0..count {
            self.row_definitions.push(row.into())
        }
        self
    }

    /// Builds the rows.
    pub fn build(self) -> Rows {
        Rows(self.row_definitions)
    }
}

/// Helper struct used inside of the row Property.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rows(Vec<Row>);

impl Rows {
    /// Creates a new `RowsBuilder` object with default values.
    pub fn create() -> RowsBuilder {
        RowsBuilder::new()
    }

    /// Returns the number of elements in the rows list, also referred to as its 'length'.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Is the row empty?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a reference to an row.
    pub fn get(&self, row: usize) -> Option<&Row> {
        self.0.get(row)
    }

    /// Returns a mutable reference to an row.
    pub fn get_mut(&mut self, row: usize) -> Option<&mut Row> {
        self.0.get_mut(row)
    }

    /// Returns an iterator over the slice.
    pub fn iter(&self) -> Iter<Row> {
        self.0.iter()
    }

    /// Returns a mutable iterator over the slice.
    pub fn iter_mut(&mut self) -> IterMut<Row> {
        self.0.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height() {
        let height = RowHeight::Height(64.0);

        let builder = RowBuilder::new();
        let row = builder.height(height).build();

        assert_eq!(row.height, height);
    }

    #[test]
    fn test_min_height() {
        let min_height = 64.0;

        let builder = RowBuilder::new();
        let row = builder.min_height(min_height).build();

        assert_eq!(row.min_height, min_height);
    }

    #[test]
    fn test_max_height() {
        let max_height = 64.0;

        let builder = RowBuilder::new();
        let row = builder.max_height(max_height).build();

        assert_eq!(row.max_height, max_height);
    }

    #[test]
    fn test_set_current_height() {
        let out_one_height = 10.0;
        let out_two_height = 66.0;
        let in_height = 33.0;
        let min_height = 14.0;
        let max_height = 64.0;

        let builder = RowBuilder::new();
        let mut row = builder
            .min_height(min_height)
            .max_height(max_height)
            .build();

        row.set_current_height(out_one_height);
        assert_eq!(row.current_height(), min_height);

        row.set_current_height(out_two_height);
        assert_eq!(row.current_height(), max_height);

        row.set_current_height(in_height);
        assert_eq!(row.current_height(), in_height);
    }

    #[test]
    fn test_row() {
        let builder = RowsBuilder::new();
        let rows = builder.build();

        assert_eq!(rows.len(), 0);

        let builder = RowsBuilder::new();
        let rows = builder
            .row(Row::create().build())
            .row(Row::create().build())
            .build();

        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn test_row_height_into() {
        let row: Row = "Auto".into();
        assert_eq!(row.height(), RowHeight::Auto);

        let row: Row = "auto".into();
        assert_eq!(row.height(), RowHeight::Auto);

        let row: Row = "Stretch".into();
        assert_eq!(row.height(), RowHeight::Stretch);

        let row: Row = "stretch".into();
        assert_eq!(row.height(), RowHeight::Stretch);

        let row: Row = "*".into();
        assert_eq!(row.height(), RowHeight::Stretch);

        let row: Row = "other".into();
        assert_eq!(row.height(), RowHeight::Stretch);

        let row: Row = 64.0.into();
        assert_eq!(row.height(), RowHeight::Height(64.0));
    }
}
