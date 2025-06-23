use std::fmt;
use std::time::Duration;

pub struct FormattedDuration(Duration);

impl FormattedDuration {
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }

    fn item<T>(f: &mut fmt::Formatter, name: &str, value: T) -> fmt::Result
    where
        T: std::fmt::Display + std::cmp::PartialEq + Default,
    {
        if value != T::default() {
            write!(f, "{value}{name}")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for FormattedDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let secs = self.0.as_secs();
        let nanos = self.0.subsec_nanos();

        if secs == 0 && nanos == 0 {
            f.write_str("0s")?;
            return Ok(());
        }

        let years = secs / 31_557_600; // 365.25d
        let ydays = secs % 31_557_600;
        let months = ydays / 2_630_016; // 30.44d
        let mdays = ydays % 2_630_016;
        let days = mdays / 86400;
        let day_secs = mdays % 86400;
        let hours = day_secs / 3600;
        let minutes = day_secs % 3600 / 60;
        let seconds = day_secs % 60;

        Self::item(f, "y", years)?;
        Self::item(f, "mo", months)?;
        Self::item(f, "d", days)?;
        Self::item(f, "h", hours as u32)?;
        Self::item(f, "m", minutes as u32)?;
        Self::item(f, "s", seconds as u32)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatted_second() {
        let d = FormattedDuration::new(Duration::from_secs(2));
        assert_eq!("2s", d.to_string());
    }

    #[test]
    fn formatted_minutes_and_secs() {
        let d = FormattedDuration::new(Duration::from_secs(62));
        assert_eq!("1m2s", d.to_string());
    }

    #[test]
    fn formatted_empty() {
        let d = FormattedDuration::new(Duration::from_secs(0));
        assert_eq!("0s", d.to_string());
    }

    #[test]
    fn formatted_hour() {
        let d = FormattedDuration::new(Duration::from_secs(3600));
        assert_eq!("1h", d.to_string());
    }
}
