use chrono::{Datelike, NaiveDate, Weekday};
use std::collections::BTreeMap;

use crate::computus::moveable_feasts;
use crate::precedence::resolve_precedence;
use crate::sanctoral::build_sanctoral_cycle;
use crate::temporal::build_temporal_cycle;
use crate::types::*;

/// The main calendar for a given year under the 1962 rubrics.
pub struct Calendar {
    year: i32,
    days: BTreeMap<NaiveDate, LiturgicalDay>,
}

impl Calendar {
    /// Build the complete calendar for a given year.
    pub fn new(year: i32) -> Self {
        let temporal = build_temporal_cycle(year);
        let sanctoral = build_sanctoral_cycle(year);
        let _mf = moveable_feasts(year);

        let mut days = BTreeMap::new();

        for (date, (entry, special_celebration)) in &temporal {
            // Build the temporal celebration for this day
            let temporal_celeb = if let Some(special) = special_celebration {
                special.clone()
            } else if date.weekday() == Weekday::Sun {
                Celebration::sunday(entry.season, entry.week)
            } else {
                Celebration::feria(entry.season, entry.week, date.weekday())
            };

            // Get sanctoral celebrations for this date
            let sanctoral_celebs = sanctoral.get(date).cloned().unwrap_or_default();

            // Resolve precedence
            let (winner, commemorations) = resolve_precedence(&temporal_celeb, &sanctoral_celebs);

            let readings = crate::readings::get_readings(&winner.id);
            let notes = crate::readings::get_notes(&winner.id);

            let day = LiturgicalDay {
                date: *date,
                season: entry.season,
                week: entry.week,
                day_of_week: format!("{:?}", date.weekday()),
                celebration: winner.clone(),
                commemorations,
                color: winner.color,
                readings,
                notes,
            };

            days.insert(*date, day);
        }

        Self { year, days }
    }

    /// Get the liturgical day for a specific date.
    pub fn get(&self, date: NaiveDate) -> Option<&LiturgicalDay> {
        self.days.get(&date)
    }

    /// Get all days in the calendar.
    pub fn days(&self) -> &BTreeMap<NaiveDate, LiturgicalDay> {
        &self.days
    }

    /// Get the year.
    pub fn year(&self) -> i32 {
        self.year
    }

    /// Get the moveable feasts for this year.
    pub fn moveable_feasts(&self) -> MoveableFeasts {
        moveable_feasts(self.year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_has_all_days() {
        let cal = Calendar::new(2026);
        let _jan1 = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let _dec31 = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        assert_eq!(cal.days().len(), 365);
    }

    #[test]
    fn test_calendar_leap_year() {
        let cal = Calendar::new(2024);
        assert_eq!(cal.days().len(), 366);
    }

    #[test]
    fn test_easter_sunday_wins() {
        let cal = Calendar::new(2026);
        let easter = NaiveDate::from_ymd_opt(2026, 4, 5).unwrap();
        let day = cal.get(easter).unwrap();
        assert_eq!(day.celebration.id, "easter-sunday");
        assert_eq!(day.color, LiturgicalColor::White);
    }

    #[test]
    fn test_christmas_day() {
        let cal = Calendar::new(2026);
        let dec25 = NaiveDate::from_ymd_opt(2026, 12, 25).unwrap();
        let day = cal.get(dec25).unwrap();
        assert_eq!(day.celebration.id, "christmas");
        assert_eq!(day.season, LiturgicalSeason::Christmas);
    }

    #[test]
    fn test_ash_wednesday() {
        let cal = Calendar::new(2026);
        let date = NaiveDate::from_ymd_opt(2026, 2, 18).unwrap();
        let day = cal.get(date).unwrap();
        assert_eq!(day.celebration.id, "ash-wednesday");
        assert_eq!(day.color, LiturgicalColor::Violet);
    }

    #[test]
    fn test_all_days_have_color() {
        let cal = Calendar::new(2026);
        for (_, day) in cal.days() {
            // Color should be set
            let _ = day.color;
        }
    }

    #[test]
    fn test_all_days_have_one_celebration() {
        let cal = Calendar::new(2026);
        for (_, day) in cal.days() {
            assert!(!day.celebration.id.is_empty());
        }
    }
}
