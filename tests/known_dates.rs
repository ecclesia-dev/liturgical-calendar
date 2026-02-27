use calendar_core::*;
use chrono::{Datelike, NaiveDate};

fn d(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

fn cal(year: i32) -> Calendar {
    Calendar::new(year)
}

// ============================================================
// Easter dates (Computus verification)
// ============================================================

#[test]
fn easter_2024() { assert_eq!(computus::easter(2024), d(2024, 3, 31)); }
#[test]
fn easter_2025() { assert_eq!(computus::easter(2025), d(2025, 4, 20)); }
#[test]
fn easter_2026() { assert_eq!(computus::easter(2026), d(2026, 4, 5)); }
#[test]
fn easter_2027() { assert_eq!(computus::easter(2027), d(2027, 3, 28)); }
#[test]
fn easter_2028() { assert_eq!(computus::easter(2028), d(2028, 4, 16)); }

// ============================================================
// 2026 Calendar - Major Fixed Feasts
// ============================================================

#[test]
fn circumcision_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 1, 1)).unwrap();
    assert_eq!(day.celebration.id, "circumcision");
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
    assert_eq!(day.color, LiturgicalColor::White);
    assert_eq!(day.season, LiturgicalSeason::Christmas);
}

#[test]
fn epiphany_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 1, 6)).unwrap();
    assert_eq!(day.celebration.id, "epiphany");
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
    assert_eq!(day.color, LiturgicalColor::White);
}

#[test]
fn st_thomas_aquinas_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 1, 28)).unwrap();
    assert_eq!(day.celebration.id, "st-thomas-aquinas");
    assert_eq!(day.celebration.rank, CelebrationRank::ClassIII);
}

#[test]
fn purification_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 2, 2)).unwrap();
    assert_eq!(day.celebration.id, "purification-bvm");
    assert_eq!(day.color, LiturgicalColor::White);
}

#[test]
fn ash_wednesday_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 2, 18)).unwrap();
    assert_eq!(day.celebration.id, "ash-wednesday");
    assert_eq!(day.color, LiturgicalColor::Violet);
    assert_eq!(day.season, LiturgicalSeason::Lent);
}

#[test]
fn st_joseph_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 3, 19)).unwrap();
    assert_eq!(day.celebration.id, "st-joseph");
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
    assert_eq!(day.color, LiturgicalColor::White);
}

#[test]
fn annunciation_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 3, 25)).unwrap();
    assert_eq!(day.celebration.id, "annunciation");
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
    assert_eq!(day.color, LiturgicalColor::White);
}

#[test]
fn palm_sunday_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 3, 29)).unwrap();
    assert_eq!(day.celebration.id, "palm-sunday");
    assert_eq!(day.color, LiturgicalColor::Violet);
    assert_eq!(day.season, LiturgicalSeason::HolyWeek);
}

#[test]
fn holy_thursday_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 4, 2)).unwrap();
    assert_eq!(day.celebration.id, "holy-thursday");
    assert_eq!(day.color, LiturgicalColor::White);
}

#[test]
fn good_friday_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 4, 3)).unwrap();
    assert_eq!(day.celebration.id, "good-friday");
    assert_eq!(day.color, LiturgicalColor::Black);
}

#[test]
fn holy_saturday_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 4, 4)).unwrap();
    assert_eq!(day.celebration.id, "holy-saturday");
    assert_eq!(day.color, LiturgicalColor::Violet);
}

#[test]
fn easter_sunday_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 4, 5)).unwrap();
    assert_eq!(day.celebration.id, "easter-sunday");
    assert_eq!(day.color, LiturgicalColor::White);
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
    assert_eq!(day.celebration.precedence, 1);
}

#[test]
fn easter_monday_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 4, 6)).unwrap();
    assert_eq!(day.celebration.id, "easter-octave-1");
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
    assert_eq!(day.season, LiturgicalSeason::Easter);
}

#[test]
fn low_sunday_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 4, 12)).unwrap();
    assert_eq!(day.celebration.id, "low-sunday");
}

#[test]
fn st_mark_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 4, 25)).unwrap();
    assert_eq!(day.celebration.id, "st-mark");
}

#[test]
fn st_joseph_worker_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 5, 1)).unwrap();
    assert_eq!(day.celebration.id, "st-joseph-worker");
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
}

#[test]
fn ascension_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 5, 14)).unwrap();
    assert_eq!(day.celebration.id, "ascension");
    assert_eq!(day.color, LiturgicalColor::White);
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
}

#[test]
fn pentecost_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 5, 24)).unwrap();
    assert_eq!(day.celebration.id, "pentecost");
    assert_eq!(day.color, LiturgicalColor::Red);
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
}

#[test]
fn pentecost_monday_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 5, 25)).unwrap();
    assert_eq!(day.celebration.id, "pentecost-octave-1");
}

#[test]
fn corpus_christi_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 6, 4)).unwrap();
    assert_eq!(day.celebration.id, "corpus-christi");
    assert_eq!(day.color, LiturgicalColor::White);
}

#[test]
fn sacred_heart_2026() {
    let c = cal(2026);
    // Sacred Heart = Corpus Christi (Jun 4) + 8 = Jun 12
    let day = c.get(d(2026, 6, 12)).unwrap();
    assert_eq!(day.celebration.id, "sacred-heart");
    assert_eq!(day.color, LiturgicalColor::White);
}

#[test]
fn nativity_john_baptist_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 6, 24)).unwrap();
    assert_eq!(day.celebration.id, "nativity-of-st-john-baptist");
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
}

#[test]
fn ss_peter_paul_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 6, 29)).unwrap();
    assert_eq!(day.celebration.id, "ss-peter-paul");
    assert_eq!(day.color, LiturgicalColor::Red);
}

#[test]
fn transfiguration_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 8, 6)).unwrap();
    assert_eq!(day.celebration.id, "transfiguration");
    assert_eq!(day.color, LiturgicalColor::White);
}

#[test]
fn assumption_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 8, 15)).unwrap();
    assert_eq!(day.celebration.id, "assumption-bvm");
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
    assert_eq!(day.color, LiturgicalColor::White);
}

#[test]
fn st_michael_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 9, 29)).unwrap();
    assert_eq!(day.celebration.id, "st-michael");
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
}

#[test]
fn holy_rosary_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 10, 7)).unwrap();
    assert_eq!(day.celebration.id, "holy-rosary");
}

#[test]
fn christ_the_king_2026() {
    let c = cal(2026);
    // Last Sunday of October 2026 = Oct 25
    let day = c.get(d(2026, 10, 25)).unwrap();
    assert_eq!(day.celebration.id, "christ-the-king");
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
}

#[test]
fn all_saints_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 11, 1)).unwrap();
    assert_eq!(day.celebration.id, "all-saints");
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
    assert_eq!(day.color, LiturgicalColor::White);
}

#[test]
fn all_souls_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 11, 2)).unwrap();
    assert_eq!(day.celebration.id, "all-souls");
    assert_eq!(day.color, LiturgicalColor::Black);
}

#[test]
fn immaculate_conception_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 12, 8)).unwrap();
    assert_eq!(day.celebration.id, "immaculate-conception");
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
    assert_eq!(day.color, LiturgicalColor::White);
}

#[test]
fn christmas_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 12, 25)).unwrap();
    assert_eq!(day.celebration.id, "christmas");
    assert_eq!(day.celebration.rank, CelebrationRank::ClassI);
    assert_eq!(day.color, LiturgicalColor::White);
    assert_eq!(day.celebration.precedence, 1);
}

#[test]
fn st_stephen_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 12, 26)).unwrap();
    assert_eq!(day.celebration.id, "st-stephen");
    assert_eq!(day.color, LiturgicalColor::Red);
}

#[test]
fn st_john_evangelist_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 12, 27)).unwrap();
    assert_eq!(day.celebration.id, "st-john-evangelist");
}

#[test]
fn holy_innocents_2026() {
    let c = cal(2026);
    let day = c.get(d(2026, 12, 28)).unwrap();
    assert_eq!(day.celebration.id, "holy-innocents");
    assert_eq!(day.color, LiturgicalColor::Red);
}

// ============================================================
// Season assignments
// ============================================================

#[test]
fn season_jan_7_is_after_epiphany() {
    let c = cal(2026);
    let day = c.get(d(2026, 1, 7)).unwrap();
    assert_eq!(day.season, LiturgicalSeason::AfterEpiphany);
}

#[test]
fn season_feb_1_is_septuagesima() {
    let c = cal(2026);
    // Septuagesima 2026 = Feb 1
    let day = c.get(d(2026, 2, 1)).unwrap();
    assert_eq!(day.season, LiturgicalSeason::Septuagesima);
}

#[test]
fn season_march_22_is_passiontide() {
    let c = cal(2026);
    // Passion Sunday 2026 = Mar 22
    let day = c.get(d(2026, 3, 22)).unwrap();
    assert_eq!(day.season, LiturgicalSeason::Passiontide);
}

#[test]
fn season_march_30_is_holy_week() {
    let c = cal(2026);
    let day = c.get(d(2026, 3, 30)).unwrap();
    assert_eq!(day.season, LiturgicalSeason::HolyWeek);
}

#[test]
fn season_may_15_is_ascensiontide() {
    let c = cal(2026);
    let day = c.get(d(2026, 5, 15)).unwrap();
    assert_eq!(day.season, LiturgicalSeason::Ascensiontide);
}

#[test]
fn season_june_15_is_after_pentecost() {
    let c = cal(2026);
    let day = c.get(d(2026, 6, 15)).unwrap();
    assert_eq!(day.season, LiturgicalSeason::AfterPentecost);
}

#[test]
fn season_dec_1_is_advent() {
    let c = cal(2026);
    let day = c.get(d(2026, 12, 1)).unwrap();
    assert_eq!(day.season, LiturgicalSeason::Advent);
}

// ============================================================
// Liturgical colors for seasons
// ============================================================

#[test]
fn green_sundays_after_pentecost() {
    let c = cal(2026);
    // A Sunday after Pentecost that doesn't conflict with a feast
    // 7th Sunday after Pentecost 2026: Jul 12
    let day = c.get(d(2026, 7, 12)).unwrap();
    if day.celebration.category == CelebrationCategory::Sunday {
        assert_eq!(day.color, LiturgicalColor::Green);
    }
}

#[test]
fn violet_lent_feria() {
    let c = cal(2026);
    // A Thursday in Lent (Feb 19, day after Ash Wednesday)
    let day = c.get(d(2026, 2, 19)).unwrap();
    assert_eq!(day.color, LiturgicalColor::Violet);
}

// ============================================================
// Cross-year verification (2024, 2025, 2028)
// ============================================================

#[test]
fn easter_2024_calendar() {
    let c = cal(2024);
    let day = c.get(d(2024, 3, 31)).unwrap();
    assert_eq!(day.celebration.id, "easter-sunday");
}

#[test]
fn christmas_2024() {
    let c = cal(2024);
    let day = c.get(d(2024, 12, 25)).unwrap();
    assert_eq!(day.celebration.id, "christmas");
}

#[test]
fn ash_wednesday_2025() {
    let c = cal(2025);
    // Easter 2025 = Apr 20, Ash Wed = Apr 20 - 46 = Mar 5
    let day = c.get(d(2025, 3, 5)).unwrap();
    assert_eq!(day.celebration.id, "ash-wednesday");
}

#[test]
fn pentecost_2025() {
    let c = cal(2025);
    // Easter 2025 = Apr 20, Pentecost = Jun 8
    let day = c.get(d(2025, 6, 8)).unwrap();
    assert_eq!(day.celebration.id, "pentecost");
}

#[test]
fn easter_2028_calendar() {
    let c = cal(2028);
    let day = c.get(d(2028, 4, 16)).unwrap();
    assert_eq!(day.celebration.id, "easter-sunday");
}

// ============================================================
// Property tests
// ============================================================

#[test]
fn every_day_has_celebration_all_years() {
    for year in 2020..=2030 {
        let c = cal(year);
        for (date, day) in c.days() {
            assert!(!day.celebration.id.is_empty(),
                "Date {} in year {} has empty celebration", date, year);
        }
    }
}

#[test]
fn every_sunday_has_sunday_or_feast() {
    let c = cal(2026);
    for (date, day) in c.days() {
        if date.weekday() == chrono::Weekday::Sun {
            // Either the celebration is a Sunday, or a feast that outranks it
            let is_sunday = day.celebration.category == CelebrationCategory::Sunday;
            let is_high_feast = day.celebration.precedence <= 5;
            assert!(is_sunday || is_high_feast,
                "Sunday {} has neither Sunday nor high feast: {:?}", date, day.celebration);
        }
    }
}

#[test]
fn easter_never_has_commemorations_from_sanctoral() {
    for year in 2020..=2030 {
        let c = cal(year);
        let easter = computus::easter(year);
        let day = c.get(easter).unwrap();
        assert_eq!(day.celebration.id, "easter-sunday");
        // No sanctoral commemorations should override Easter
        assert_eq!(day.celebration.precedence, 1);
    }
}
