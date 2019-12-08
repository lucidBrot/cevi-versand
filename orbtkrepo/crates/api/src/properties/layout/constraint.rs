use std::f64;

/// Used to build a constraint, specifying additional details.
#[derive(Default)]
pub struct ConstraintBuilder {
    width: f64,
    height: f64,
    min_width: f64,
    min_height: f64,
    max_width: f64,
    max_height: f64,
}

/// Used to build a constraint, specifying additional details.
impl ConstraintBuilder {
    /// Creates a new `ConstraintBuilder` with default values.
    pub fn new() -> Self {
        ConstraintBuilder::default()
    }

    /// Inserts a new width.
    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    /// Inserts a new height.
    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    /// Inserts a new size.
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Inserts a new min_width.
    pub fn min_width(mut self, min_width: f64) -> Self {
        self.min_width = min_width;
        self
    }

    /// Inserts a new min_height.
    pub fn min_height(mut self, min_height: f64) -> Self {
        self.min_height = min_height;
        self
    }

    /// Inserts a new min_size.
    pub fn min_size(mut self, min_width: f64, min_height: f64) -> Self {
        self.min_width = min_width;
        self.min_height = min_height;
        self
    }

    /// Inserts a new max_width.
    pub fn max_width(mut self, max_width: f64) -> Self {
        self.max_width = max_width;
        self
    }

    /// Inserts a new max_height.
    pub fn max_height(mut self, max_height: f64) -> Self {
        self.max_height = max_height;
        self
    }

    /// Inserts a new min_size.
    pub fn max_size(mut self, max_width: f64, max_height: f64) -> Self {
        self.max_width = max_width;
        self.max_height = max_height;
        self
    }

    /// Builds the constraint.
    pub fn build(self) -> Constraint {
        Constraint {
            width: self.width,
            height: self.height,
            min_width: self.min_width,
            min_height: self.min_height,
            max_width: self.max_width,
            max_height: self.max_height,
        }
    }
}

/// `Constraint` describes a box constraint.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Constraint {
    width: f64,
    height: f64,
    min_width: f64,
    min_height: f64,
    max_width: f64,
    max_height: f64,
}

impl Default for Constraint {
    fn default() -> Self {
        Constraint {
            width: 0.0,
            height: 0.0,
            min_width: 0.0,
            min_height: 0.0,
            max_width: f64::MAX,
            max_height: f64::MAX,
        }
    }
}

impl Constraint {
    /// Returns a constraint builder.
    pub fn create() -> ConstraintBuilder {
        ConstraintBuilder::new()
    }

    // Gets width.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Sets width.
    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    /// Gets height.
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Sets height.
    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    /// Gets the size.
    pub fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    /// Sets the size.
    pub fn set_size(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    /// Gets min_width.
    pub fn min_width(&self) -> f64 {
        self.min_width
    }

    /// Sets min_width.
    pub fn set_min_width(&mut self, min_width: f64) {
        self.min_width = min_width;
    }

    /// Gets min_height.
    pub fn min_height(&self) -> f64 {
        self.min_height
    }

    /// Sets min_height.
    pub fn set_min_height(&mut self, min_height: f64) {
        self.min_height = min_height;
    }

    /// Gets the min_size.
    pub fn min_size(&self) -> (f64, f64) {
        (self.min_width, self.min_height)
    }

    /// Sets the min size.
    pub fn set_min_size(&mut self, min_width: f64, min_height: f64) {
        self.min_width = min_width;
        self.min_height = min_height;
    }

    /// Gets max_width.
    pub fn max_width(&self) -> f64 {
        self.max_width
    }

    /// Sets max_width.
    pub fn set_max_width(&mut self, max_width: f64) {
        self.max_width = max_width;
    }

    /// Gets max_height.
    pub fn max_height(&self) -> f64 {
        self.max_height
    }

    /// Sets max_height.
    pub fn set_max_height(&mut self, max_height: f64) {
        self.max_height = max_height;
    }

    /// Gets the max_size.
    pub fn max_size(&self) -> (f64, f64) {
        (self.max_width, self.max_height)
    }

    /// Sets the max size.
    pub fn set_max_size(&mut self, max_width: f64, max_height: f64) {
        self.max_width = max_width;
        self.max_height = max_height;
    }

    /// Adjust the given `size` to match the constraint.
    pub fn perform(&self, size: (f64, f64)) -> (f64, f64) {
        let size = {
            let width = if self.width > 0.0 { self.width } else { size.0 };
            let height = if self.height > 0.0 {
                self.height
            } else {
                size.1
            };

            (width, height)
        };

        (
            constrain(size.0, self.min_width, self.max_width, self.width),
            constrain(size.1, self.min_height, self.max_height, self.height),
        )
    }
}

// Check constraint for the given
fn constrain(val: f64, min: f64, max: f64, size: f64) -> f64 {
    if min == 0.0 && max == 0.0 && size > 0.0 {
        size
    } else if val < min && min > 0.0 {
        min
    } else if val > max && max > 0.0 {
        max
    } else {
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_width() {
        let width = 12.0;

        let constraint = Constraint::create().width(width).build();

        assert_eq!(constraint.width(), width);
    }

    #[test]
    fn test_builder_height() {
        let height = 12.0;

        let constraint = Constraint::create().height(height).build();

        assert_eq!(constraint.height(), height);
    }

    #[test]
    fn test_builder_min_width() {
        let width = 12.0;

        let constraint = Constraint::create().min_width(width).build();

        assert_eq!(constraint.min_width(), width);
    }

    #[test]
    fn test_builder_min_height() {
        let height = 12.0;

        let constraint = Constraint::create().min_height(height).build();

        assert_eq!(constraint.min_height(), height);
    }

    #[test]
    fn test_builder_max_width() {
        let width = 12.0;

        let constraint = Constraint::create().max_width(width).build();

        assert_eq!(constraint.max_width(), width);
    }

    #[test]
    fn test_builder_max_height() {
        let height = 12.0;

        let constraint = Constraint::create().max_height(height).build();

        assert_eq!(constraint.max_height(), height);
    }

    #[test]
    fn test_set_width() {
        let width = 12.0;

        let mut constraint = Constraint::default();
        constraint.set_width(width);
        assert_eq!(constraint.width(), width);
    }

    #[test]
    fn test_set_height() {
        let height = 12.0;

        let mut constraint = Constraint::default();
        constraint.set_height(height);

        assert_eq!(constraint.height(), height);
    }

    #[test]
    fn test_set_size() {
        let width = 12.0;
        let height = 14.0;

        let mut constraint = Constraint::default();
        constraint.set_size(width, height);

        assert_eq!(constraint.size(), (width, height));
    }

    #[test]
    fn test_set_min_width() {
        let min_width = 12.0;

        let mut constraint = Constraint::default();
        constraint.set_min_width(min_width);

        assert_eq!(constraint.min_width(), min_width);
    }

    #[test]
    fn test_set_min_height() {
        let min_height = 12.0;

        let mut constraint = Constraint::default();
        constraint.set_min_height(min_height);

        assert_eq!(constraint.min_height(), min_height);
    }

    #[test]
    fn test_set_min_size() {
        let min_width = 12.0;
        let min_height = 14.0;

        let mut constraint = Constraint::default();
        constraint.set_min_size(min_width, min_height);

        assert_eq!(constraint.min_size(), (min_width, min_height));
    }

    #[test]
    fn test_set_max_width() {
        let max_width = 12.0;

        let mut constraint = Constraint::default();
        constraint.set_max_width(max_width);

        assert_eq!(constraint.max_width(), max_width);
    }

    #[test]
    fn test_set_max_height() {
        let max_height = 12.0;

        let mut constraint = Constraint::default();
        constraint.set_max_height(max_height);

        assert_eq!(constraint.max_height(), max_height);
    }

    #[test]
    fn test_set_max_size() {
        let max_width = 12.0;
        let max_height = 14.0;

        let mut constraint = Constraint::default();
        constraint.set_max_size(max_width, max_height);

        assert_eq!(constraint.max_size(), (max_width, max_height));
    }

    #[test]
    fn test_perform() {
        let mut constraint = Constraint::default();

        constraint.set_min_width(10.0);
        constraint.set_min_height(10.0);
        constraint.set_max_width(50.0);
        constraint.set_max_height(60.0);
        constraint.set_width(0.0);
        constraint.set_height(0.0);

        assert_eq!(constraint.perform((10.0, 59.0)), (10.0, 59.0));
        assert_eq!(constraint.perform((5.0, 40.0)), (10.0, 40.0));
        assert_eq!(constraint.perform((10.0, 70.0)), (10.0, 60.0));
    }
}
