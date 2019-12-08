/// Used to align a widget vertical or horizontal.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Stretch
    }
}

impl Alignment {
    /// Calculates the position (x or y) of the widget depending on the available measure, the goal measure
    /// margin and alignment.
    pub fn align_position(
        self,
        available_measure: f64,
        measure: f64,
        margin_start: f64,
        margin_end: f64,
    ) -> f64 {
        match self {
            Alignment::End => available_measure - measure - margin_end,
            Alignment::Center => (available_measure - measure) / 2.0,
            _ => margin_start,
        }
    }

    /// Calculates the measure (measure or height) of the widget depending on the available measure, the goal measure
    /// margin and horizontal alignment.
    pub fn align_measure(
        self,
        available_measure: f64,
        measure: f64,
        margin_start: f64,
        margin_end: f64,
    ) -> f64 {
        match self {
            Alignment::Stretch => available_measure - margin_start - margin_end,
            _ => measure,
        }
    }
}

impl From<&str> for Alignment {
    fn from(t: &str) -> Self {
        match t {
            "End" | "end" => Alignment::End,
            "Center" | "center" => Alignment::Center,
            "Start" | "start" => Alignment::Start,
            _ => Alignment::Stretch,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_align_position() {
        let available_measure = 100.0;
        let measure = 50.0;

        let alignment = Alignment::Stretch;
        assert_eq!(
            alignment.align_position(available_measure, measure, 0.0, 0.0),
            0.0
        );

        let alignment = Alignment::Center;
        assert_eq!(
            alignment.align_position(available_measure, measure, 0.0, 0.0),
            25.0
        );

        let alignment = Alignment::Start;
        assert_eq!(
            alignment.align_position(available_measure, measure, 0.0, 0.0),
            0.0
        );

        let alignment = Alignment::End;
        assert_eq!(
            alignment.align_position(available_measure, measure, 0.0, 0.0),
            50.0
        );
    }

    #[test]
    fn test_align_measure() {
        let available_measure = 100.0;
        let measure = 50.0;

        let alignment = Alignment::Stretch;
        assert_eq!(
            alignment.align_measure(available_measure, measure, 0.0, 0.0),
            available_measure
        );

        let alignment = Alignment::Center;
        assert_eq!(
            alignment.align_measure(available_measure, measure, 0.0, 0.0),
            measure
        );

        let alignment = Alignment::Start;
        assert_eq!(
            alignment.align_measure(available_measure, measure, 0.0, 0.0),
            measure
        );

        let alignment = Alignment::End;
        assert_eq!(
            alignment.align_measure(available_measure, measure, 0.0, 0.0),
            measure
        );
    }

    #[test]
    fn test_into() {
        let alignment: Alignment = "start".into();
        assert_eq!(alignment, Alignment::Start);

        let alignment: Alignment = "start".into();
        assert_eq!(alignment, Alignment::Start);

        let alignment: Alignment = "Center".into();
        assert_eq!(alignment, Alignment::Center);

        let alignment: Alignment = "center".into();
        assert_eq!(alignment, Alignment::Center);

        let alignment: Alignment = "end".into();
        assert_eq!(alignment, Alignment::End);

        let alignment: Alignment = "end".into();
        assert_eq!(alignment, Alignment::End);

        let alignment: Alignment = "Stretch".into();
        assert_eq!(alignment, Alignment::Stretch);

        let alignment: Alignment = "stretch".into();
        assert_eq!(alignment, Alignment::Stretch);

        let alignment: Alignment = "other".into();
        assert_eq!(alignment, Alignment::Stretch);
    }
}
