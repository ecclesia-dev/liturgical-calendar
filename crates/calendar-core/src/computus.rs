use chrono::NaiveDate;

/// Computes the date of Easter Sunday for a given year using the Anonymous
/// Gregorian algorithm (Meeus/Jones/Butcher). Valid for all years in the
/// Gregorian calendar.
pub fn easter(year: i32) -> NaiveDate {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = (h + l - 7 * m + 114) % 31 + 1;
    NaiveDate::from_ymd_opt(year, month as u32, day as u32)
        .expect("Easter computation produced invalid date")
}

use crate::types::MoveableFeasts;
use chrono::{Datelike, Duration};

/// Compute all moveable feasts for a given year.
pub fn moveable_feasts(year: i32) -> MoveableFeasts {
    let easter_date = easter(year);

    let septuagesima = easter_date - Duration::days(63);
    let ash_wednesday = easter_date - Duration::days(46);
    let passion_sunday = easter_date - Duration::days(14);
    let palm_sunday = easter_date - Duration::days(7);
    let holy_thursday = easter_date - Duration::days(3);
    let good_friday = easter_date - Duration::days(2);
    let holy_saturday = easter_date - Duration::days(1);
    let ascension = easter_date + Duration::days(39);
    let pentecost = easter_date + Duration::days(49);
    let corpus_christi = easter_date + Duration::days(60);
    // Sacred Heart: Friday after Corpus Christi octave = Corpus Christi + 8 days = Friday
    // Corpus Christi is Thursday (Easter+60). +8 = Friday after octave
    let sacred_heart = corpus_christi + Duration::days(8);

    // Christ the King: Last Sunday of October
    let oct31 = NaiveDate::from_ymd_opt(year, 10, 31).unwrap();
    let days_from_sunday = oct31.weekday().num_days_from_sunday();
    let christ_the_king = oct31 - Duration::days(days_from_sunday as i64);

    // Advent 1: Sunday nearest to Nov 30 (St. Andrew), i.e., the Sunday
    // falling on or between Nov 27 and Dec 3
    let nov30 = NaiveDate::from_ymd_opt(year, 11, 30).unwrap();
    let days_from_sun = nov30.weekday().num_days_from_sunday();
    let advent_1 = if days_from_sun <= 3 {
        nov30 - Duration::days(days_from_sun as i64)
    } else {
        nov30 + Duration::days((7 - days_from_sun) as i64)
    };

    // Ember days:
    // 1. After Ash Wednesday (Wed, Fri, Sat of 1st week of Lent)
    let _lent_ember_wed = ash_wednesday + Duration::days(4); // Wednesday after Ash Wednesday? No:
    // Ember days of Lent = Wed, Fri, Sat after 1st Sunday of Lent
    let first_sunday_of_lent = ash_wednesday + Duration::days(4); // Ash Wed is Wed, +4 = Sun
    let lent_ember = vec![
        first_sunday_of_lent + Duration::days(3), // Wed
        first_sunday_of_lent + Duration::days(5), // Fri
        first_sunday_of_lent + Duration::days(6), // Sat
    ];
    // 2. After Pentecost (Wed, Fri, Sat of the week after Pentecost = Whit Ember Days)
    let pent_ember = vec![
        pentecost + Duration::days(3),
        pentecost + Duration::days(5),
        pentecost + Duration::days(6),
    ];
    // 3. After Holy Cross (September): Wed, Fri, Sat after Sept 14 (or the 3rd week of Sept)
    // In 1962 rubrics: Wed, Fri, Sat after the 3rd Sunday of September
    let sept1 = NaiveDate::from_ymd_opt(year, 9, 1).unwrap();
    let days_to_sun = (7 - sept1.weekday().num_days_from_sunday()) % 7;
    let first_sunday_sept = sept1 + Duration::days(days_to_sun as i64);
    let third_sunday_sept = first_sunday_sept + Duration::days(14);
    let sept_ember = vec![
        third_sunday_sept + Duration::days(3),
        third_sunday_sept + Duration::days(5),
        third_sunday_sept + Duration::days(6),
    ];
    // 4. Advent: Wed, Fri, Sat after 3rd Sunday of Advent (Gaudete)
    let advent_3 = advent_1 + Duration::days(14);
    let advent_ember = vec![
        advent_3 + Duration::days(3),
        advent_3 + Duration::days(5),
        advent_3 + Duration::days(6),
    ];

    let mut ember_days = Vec::new();
    ember_days.extend(lent_ember);
    ember_days.extend(pent_ember);
    ember_days.extend(sept_ember);
    ember_days.extend(advent_ember);

    // Rogation days: Mon, Tue, Wed before Ascension
    let rogation_days = vec![
        ascension - Duration::days(3),
        ascension - Duration::days(2),
        ascension - Duration::days(1),
    ];

    MoveableFeasts {
        easter: easter_date,
        septuagesima,
        ash_wednesday,
        passion_sunday,
        palm_sunday,
        holy_thursday,
        good_friday,
        holy_saturday,
        ascension,
        pentecost,
        corpus_christi,
        sacred_heart,
        christ_the_king,
        advent_1,
        ember_days,
        rogation_days,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Weekday;

    #[test]
    fn test_easter_known_dates() {
        // Verified against published Easter dates
        let cases = vec![
            (2000, 4, 23),
            (2001, 4, 15),
            (2005, 3, 27),
            (2008, 3, 23),
            (2010, 4, 4),
            (2015, 4, 5),
            (2016, 3, 27),
            (2017, 4, 16),
            (2018, 4, 1),
            (2019, 4, 21),
            (2020, 4, 12),
            (2021, 4, 4),
            (2022, 4, 17),
            (2023, 4, 9),
            (2024, 3, 31),
            (2025, 4, 20),
            (2026, 4, 5),
            (2027, 3, 28),
            (2028, 4, 16),
            (2029, 4, 1),
            (2030, 4, 21),
            (2038, 4, 25), // Latest possible
            (1818, 3, 22), // Earliest possible in 19th-20th century range
        ];
        for (year, month, day) in cases {
            let expected = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            assert_eq!(easter(year), expected, "Easter {} failed", year);
        }
    }

    #[test]
    fn test_easter_range() {
        // Easter must always fall between March 22 and April 25
        for year in 1900..=2100 {
            let e = easter(year);
            let mar22 = NaiveDate::from_ymd_opt(year, 3, 22).unwrap();
            let apr25 = NaiveDate::from_ymd_opt(year, 4, 25).unwrap();
            assert!(e >= mar22 && e <= apr25, "Easter {} = {} out of range", year, e);
        }
    }

    #[test]
    fn test_easter_always_sunday() {
        for year in 1900..=2100 {
            assert_eq!(easter(year).weekday(), Weekday::Sun, "Easter {} not Sunday", year);
        }
    }

    #[test]
    fn test_moveable_feasts_2026() {
        let mf = moveable_feasts(2026);
        assert_eq!(mf.easter, NaiveDate::from_ymd_opt(2026, 4, 5).unwrap());
        assert_eq!(mf.ash_wednesday, NaiveDate::from_ymd_opt(2026, 2, 18).unwrap());
        assert_eq!(mf.pentecost, NaiveDate::from_ymd_opt(2026, 5, 24).unwrap());
        assert_eq!(mf.ascension, NaiveDate::from_ymd_opt(2026, 5, 14).unwrap());
        assert_eq!(mf.corpus_christi, NaiveDate::from_ymd_opt(2026, 6, 4).unwrap());
        assert_eq!(mf.palm_sunday, NaiveDate::from_ymd_opt(2026, 3, 29).unwrap());
        assert_eq!(mf.septuagesima, NaiveDate::from_ymd_opt(2026, 2, 1).unwrap());
    }

    #[test]
    fn test_christ_the_king_is_last_sunday_october() {
        for year in 2020..=2030 {
            let mf = moveable_feasts(year);
            assert_eq!(mf.christ_the_king.weekday(), Weekday::Sun);
            assert_eq!(mf.christ_the_king.month(), 10);
            // Must be >= Oct 25
            assert!(mf.christ_the_king.day() >= 25);
        }
    }

    #[test]
    fn test_advent_1_range() {
        for year in 2020..=2030 {
            let mf = moveable_feasts(year);
            assert_eq!(mf.advent_1.weekday(), Weekday::Sun);
            // Advent 1 falls Nov 27 - Dec 3
            let (m, d) = (mf.advent_1.month(), mf.advent_1.day());
            assert!(
                (m == 11 && d >= 27) || (m == 12 && d <= 3),
                "Advent 1 {} = {} out of range", year, mf.advent_1
            );
        }
    }

    #[test]
    fn test_rogation_days_before_ascension() {
        let mf = moveable_feasts(2026);
        assert_eq!(mf.rogation_days.len(), 3);
        assert_eq!(mf.rogation_days[0].weekday(), Weekday::Mon);
        assert_eq!(mf.rogation_days[1].weekday(), Weekday::Tue);
        assert_eq!(mf.rogation_days[2].weekday(), Weekday::Wed);
        assert_eq!(mf.rogation_days[2] + Duration::days(1), mf.ascension);
    }
}
