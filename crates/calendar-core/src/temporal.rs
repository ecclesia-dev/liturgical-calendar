use chrono::{Datelike, Duration, NaiveDate, Weekday};
use std::collections::BTreeMap;

use crate::computus::moveable_feasts;
use crate::types::*;

/// Season and week assignment for a date
#[derive(Debug, Clone, Copy)]
pub struct TemporalEntry {
    pub season: LiturgicalSeason,
    pub week: u8,
}

/// Build the temporal cycle for a given year.
/// Returns a map from date -> (season, week, optional special celebration).
pub fn build_temporal_cycle(year: i32) -> BTreeMap<NaiveDate, (TemporalEntry, Option<Celebration>)> {
    let mf = moveable_feasts(year);
    let prev_mf = moveable_feasts(year - 1);
    let mut map = BTreeMap::new();

    let jan1 = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let dec31 = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
    let _epiphany = NaiveDate::from_ymd_opt(year, 1, 6).unwrap();

    // We need to assign every day of the civil year a season and week.
    // The liturgical year starts on Advent 1 of the previous year, but
    // we only care about the civil year for output.

    let mut date = jan1;
    while date <= dec31 {
        let (entry, special) = classify_date(date, year, &mf, &prev_mf);
        map.insert(date, (entry, special));
        date += Duration::days(1);
    }

    map
}

fn classify_date(
    date: NaiveDate,
    year: i32,
    mf: &MoveableFeasts,
    _prev_mf: &MoveableFeasts,
) -> (TemporalEntry, Option<Celebration>) {
    let epiphany = NaiveDate::from_ymd_opt(year, 1, 6).unwrap();
    let christmas = NaiveDate::from_ymd_opt(year, 12, 25).unwrap();

    // Special fixed celebrations first
    let special = classify_special(date, year, mf);

    // Season assignment
    let entry = if date < epiphany {
        // Christmas season (from previous year's Christmas)
        TemporalEntry {
            season: LiturgicalSeason::Christmas,
            week: 1, // Octave of Christmas
        }
    } else if date >= epiphany && date < mf.septuagesima {
        // After Epiphany
        let weeks_after_epiph = ((date - epiphany).num_days() / 7) as u8;
        TemporalEntry {
            season: LiturgicalSeason::AfterEpiphany,
            week: weeks_after_epiph + 1,
        }
    } else if date >= mf.septuagesima && date < mf.ash_wednesday {
        // Septuagesima season
        let weeks = ((date - mf.septuagesima).num_days() / 7) as u8;
        TemporalEntry {
            season: LiturgicalSeason::Septuagesima,
            week: weeks + 1,
        }
    } else if date >= mf.ash_wednesday && date < mf.passion_sunday {
        // Lent
        let _days_since_ash = (date - mf.ash_wednesday).num_days();
        // Ash Wednesday to Saturday = partial week, then Sunday starts week 1
        let first_sunday_of_lent = mf.ash_wednesday + Duration::days(4); // Wed+4=Sun
        if date < first_sunday_of_lent {
            TemporalEntry {
                season: LiturgicalSeason::Lent,
                week: 0, // Before 1st Sunday
            }
        } else {
            let weeks = ((date - first_sunday_of_lent).num_days() / 7) as u8;
            TemporalEntry {
                season: LiturgicalSeason::Lent,
                week: weeks + 1,
            }
        }
    } else if date >= mf.passion_sunday && date < mf.palm_sunday {
        // Passiontide (Passion Sunday to Saturday before Palm Sunday)
        TemporalEntry {
            season: LiturgicalSeason::Passiontide,
            week: 1,
        }
    } else if date >= mf.palm_sunday && date < mf.easter {
        // Holy Week
        TemporalEntry {
            season: LiturgicalSeason::HolyWeek,
            week: 1,
        }
    } else if date >= mf.easter && date < mf.ascension {
        // Easter season
        let weeks = ((date - mf.easter).num_days() / 7) as u8;
        TemporalEntry {
            season: LiturgicalSeason::Easter,
            week: weeks + 1,
        }
    } else if date >= mf.ascension && date <= mf.pentecost {
        // Ascensiontide
        TemporalEntry {
            season: LiturgicalSeason::Ascensiontide,
            week: 1,
        }
    } else if date > mf.pentecost && date < mf.advent_1 {
        // After Pentecost
        let weeks = ((date - mf.pentecost).num_days() / 7) as u8;
        TemporalEntry {
            season: LiturgicalSeason::AfterPentecost,
            week: weeks + 1,
        }
    } else if date >= mf.advent_1 && date < christmas {
        // Advent
        let weeks = ((date - mf.advent_1).num_days() / 7) as u8;
        TemporalEntry {
            season: LiturgicalSeason::Advent,
            week: weeks + 1,
        }
    } else {
        // Christmas (Dec 25-31)
        TemporalEntry {
            season: LiturgicalSeason::Christmas,
            week: 1,
        }
    };

    (entry, special)
}

fn classify_special(
    date: NaiveDate,
    _year: i32,
    mf: &MoveableFeasts,
) -> Option<Celebration> {
    // Easter
    if date == mf.easter {
        return Some(Celebration::new(
            "easter-sunday",
            "Dominica Resurrectionis",
            "Easter Sunday",
            CelebrationRank::ClassI,
            CelebrationCategory::Solemnity,
            LiturgicalColor::White,
            1,
        ));
    }

    // Days within the Easter Octave
    if date > mf.easter && date < mf.easter + Duration::days(7) {
        return Some(Celebration::new(
            format!("easter-octave-{}", (date - mf.easter).num_days()),
            "Infra Octavam Paschae",
            format!("{} within the Octave of Easter", weekday_name(date.weekday())),
            CelebrationRank::ClassI,
            CelebrationCategory::WithinOctave,
            LiturgicalColor::White,
            1,
        ));
    }

    // Low Sunday (Octave Day of Easter)
    if date == mf.easter + Duration::days(7) {
        return Some(Celebration::new(
            "low-sunday",
            "Dominica in Albis",
            "Low Sunday (Octave Day of Easter)",
            CelebrationRank::ClassI,
            CelebrationCategory::OctaveDay,
            LiturgicalColor::White,
            1,
        ));
    }

    // Ash Wednesday
    if date == mf.ash_wednesday {
        return Some(Celebration::new(
            "ash-wednesday",
            "Feria IV Cinerum",
            "Ash Wednesday",
            CelebrationRank::ClassI,
            CelebrationCategory::Feria,
            LiturgicalColor::Violet,
            3,
        ));
    }

    // Palm Sunday
    if date == mf.palm_sunday {
        return Some(Celebration::new(
            "palm-sunday",
            "Dominica in Palmis",
            "Palm Sunday",
            CelebrationRank::ClassI,
            CelebrationCategory::Sunday,
            LiturgicalColor::Violet,
            2,
        ));
    }

    // Holy Thursday
    if date == mf.holy_thursday {
        return Some(Celebration::new(
            "holy-thursday",
            "Feria V in Cena Domini",
            "Holy Thursday",
            CelebrationRank::ClassI,
            CelebrationCategory::Solemnity,
            LiturgicalColor::White,
            1,
        ));
    }

    // Good Friday
    if date == mf.good_friday {
        return Some(Celebration::new(
            "good-friday",
            "Feria VI in Parasceve",
            "Good Friday",
            CelebrationRank::ClassI,
            CelebrationCategory::Solemnity,
            LiturgicalColor::Black,
            1,
        ));
    }

    // Holy Saturday
    if date == mf.holy_saturday {
        return Some(Celebration::new(
            "holy-saturday",
            "Sabbato Sancto",
            "Holy Saturday",
            CelebrationRank::ClassI,
            CelebrationCategory::Solemnity,
            LiturgicalColor::Violet,
            1,
        ));
    }

    // Ascension
    if date == mf.ascension {
        return Some(Celebration::new(
            "ascension",
            "In Ascensione Domini",
            "The Ascension of Our Lord",
            CelebrationRank::ClassI,
            CelebrationCategory::FeastOfLord,
            LiturgicalColor::White,
            1,
        ));
    }

    // Pentecost
    if date == mf.pentecost {
        return Some(Celebration::new(
            "pentecost",
            "Dominica Pentecostes",
            "Pentecost Sunday",
            CelebrationRank::ClassI,
            CelebrationCategory::Solemnity,
            LiturgicalColor::Red,
            1,
        ));
    }

    // Pentecost Octave days (Mon-Sat after Pentecost)
    if date > mf.pentecost && date < mf.pentecost + Duration::days(7) {
        return Some(Celebration::new(
            format!("pentecost-octave-{}", (date - mf.pentecost).num_days()),
            "Infra Octavam Pentecostes",
            format!("{} within the Octave of Pentecost", weekday_name(date.weekday())),
            CelebrationRank::ClassI,
            CelebrationCategory::WithinOctave,
            LiturgicalColor::Red,
            1,
        ));
    }

    // Corpus Christi
    if date == mf.corpus_christi {
        return Some(Celebration::new(
            "corpus-christi",
            "Ss.mi Corporis Christi",
            "Corpus Christi",
            CelebrationRank::ClassI,
            CelebrationCategory::FeastOfLord,
            LiturgicalColor::White,
            1,
        ));
    }

    // Sacred Heart
    if date == mf.sacred_heart {
        return Some(Celebration::new(
            "sacred-heart",
            "Ss.mi Cordis Jesu",
            "The Most Sacred Heart of Jesus",
            CelebrationRank::ClassI,
            CelebrationCategory::FeastOfLord,
            LiturgicalColor::White,
            4,
        ));
    }

    // Christ the King
    if date == mf.christ_the_king {
        return Some(Celebration::new(
            "christ-the-king",
            "D.N. Jesu Christi Regis",
            "Our Lord Jesus Christ the King",
            CelebrationRank::ClassI,
            CelebrationCategory::FeastOfLord,
            LiturgicalColor::White,
            4,
        ));
    }

    // Ember days
    if mf.ember_days.contains(&date) {
        return Some(Celebration::new(
            format!("ember-{}", date.format("%m-%d")),
            "Feria Quatuor Temporum",
            format!("Ember {}", weekday_name(date.weekday())),
            CelebrationRank::FeriaPrivileged,
            CelebrationCategory::EmberDay,
            LiturgicalColor::Violet,
            8,
        ));
    }

    // Rogation days
    if mf.rogation_days.contains(&date) {
        return Some(Celebration::new(
            format!("rogation-{}", date.format("%m-%d")),
            "Feria Rogationum",
            format!("Rogation {}", weekday_name(date.weekday())),
            CelebrationRank::ClassIV,
            CelebrationCategory::RogationDay,
            LiturgicalColor::Violet,
            11,
        ));
    }

    // Last Sunday after Pentecost (Sunday before Advent 1)
    if date.weekday() == Weekday::Sun
        && date >= mf.pentecost + Duration::days(7)
        && date < mf.advent_1
        && date + Duration::days(7) >= mf.advent_1
    {
        return Some(Celebration::new(
            "last-sunday-after-pentecost",
            "Dominica Ultima post Pentecosten",
            "Last Sunday after Pentecost",
            CelebrationRank::ClassI,
            CelebrationCategory::Sunday,
            LiturgicalColor::Green,
            2,
        ));
    }

    // Septuagesima, Sexagesima, Quinquagesima Sundays
    if date == mf.septuagesima {
        return Some(Celebration::sunday(LiturgicalSeason::Septuagesima, 1));
    }
    if date == mf.septuagesima + Duration::days(7) {
        return Some(Celebration::sunday(LiturgicalSeason::Septuagesima, 2));
    }
    if date == mf.septuagesima + Duration::days(14) {
        return Some(Celebration::sunday(LiturgicalSeason::Septuagesima, 3));
    }

    // Passion Sunday
    if date == mf.passion_sunday {
        return Some(Celebration::sunday(LiturgicalSeason::Passiontide, 1));
    }

    None
}

fn weekday_name(w: Weekday) -> &'static str {
    match w {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_days_assigned() {
        for year in 2020..=2030 {
            let cycle = build_temporal_cycle(year);
            let jan1 = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
            let dec31 = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
            let expected = (dec31 - jan1).num_days() + 1;
            assert_eq!(
                cycle.len() as i64, expected,
                "Year {} missing days: got {}, expected {}", year, cycle.len(), expected
            );
        }
    }

    #[test]
    fn test_ash_wednesday_2026_is_lent() {
        let cycle = build_temporal_cycle(2026);
        let ash_wed = NaiveDate::from_ymd_opt(2026, 2, 18).unwrap();
        let (entry, special) = &cycle[&ash_wed];
        assert_eq!(entry.season, LiturgicalSeason::Lent);
        assert!(special.is_some());
        assert_eq!(special.as_ref().unwrap().id, "ash-wednesday");
    }

    #[test]
    fn test_easter_2026_in_cycle() {
        let cycle = build_temporal_cycle(2026);
        let easter = NaiveDate::from_ymd_opt(2026, 4, 5).unwrap();
        let (entry, special) = &cycle[&easter];
        assert_eq!(entry.season, LiturgicalSeason::Easter);
        assert!(special.is_some());
        assert_eq!(special.as_ref().unwrap().id, "easter-sunday");
    }

    #[test]
    fn test_christmas_season_dec() {
        let cycle = build_temporal_cycle(2026);
        let dec25 = NaiveDate::from_ymd_opt(2026, 12, 25).unwrap();
        let (entry, _) = &cycle[&dec25];
        assert_eq!(entry.season, LiturgicalSeason::Christmas);
    }

    #[test]
    fn test_advent_2026() {
        let cycle = build_temporal_cycle(2026);
        // Advent 1 2026: Nov 29
        let advent1 = NaiveDate::from_ymd_opt(2026, 11, 29).unwrap();
        let (entry, _) = &cycle[&advent1];
        assert_eq!(entry.season, LiturgicalSeason::Advent);
        assert_eq!(entry.week, 1);
    }
}
