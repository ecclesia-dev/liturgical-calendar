use chrono::{Datelike, Duration, NaiveDate, Weekday};
use std::collections::BTreeMap;

use crate::types::*;

/// A fixed feast definition
#[derive(Debug, Clone)]
pub struct FixedFeast {
    pub month: u32,
    pub day: u32,
    pub celebration: Celebration,
}

/// Build the sanctoral cycle for a given year.
/// Returns a map from date -> Vec<Celebration> (multiple feasts can fall on same day;
/// precedence resolver picks the winner).
pub fn build_sanctoral_cycle(year: i32) -> BTreeMap<NaiveDate, Vec<Celebration>> {
    let feasts = major_feasts();
    let mut map: BTreeMap<NaiveDate, Vec<Celebration>> = BTreeMap::new();

    for feast in feasts {
        if let Some(date) = NaiveDate::from_ymd_opt(year, feast.month, feast.day) {
            map.entry(date).or_default().push(feast.celebration);
        }
    }

    // Holy Name of Jesus: Sunday between Jan 2-5, or Jan 2 if no Sunday
    let holy_name_date = find_sunday_between(year, 1, 2, 1, 5)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year, 1, 2).unwrap());
    map.entry(holy_name_date).or_default().push(Celebration::new(
        "holy-name-of-jesus",
        "Ss.mi Nominis Jesu",
        "The Most Holy Name of Jesus",
        CelebrationRank::ClassII,
        CelebrationCategory::FeastOfLord,
        LiturgicalColor::White,
        5,
    ));

    // Holy Family: Sunday within Octave of Christmas (Jan 1-5 range in civil year)
    // Actually in the liturgical year it's the Sunday within the Octave, which is
    // between Dec 26 and Jan 1. If no Sunday, it's Dec 30.
    // For the civil year we handle the Jan portion:
    // Check if there's a Sunday Dec 26-31 of the *previous* conceptual Christmas,
    // but since we're building for a civil year, the Holy Family within
    // the Octave at start of year = Sunday between Dec 26 prev year and Jan 1.
    // If Christmas Dec 25 is Sunday, Holy Family = Dec 30.
    // We'll place it if it lands in our year.
    let dec25_prev = NaiveDate::from_ymd_opt(year - 1, 12, 25).unwrap();
    let holy_family_date = if dec25_prev.weekday() == Weekday::Sun {
        NaiveDate::from_ymd_opt(year - 1, 12, 30).unwrap()
    } else {
        // Find Sunday between Dec 26 and Jan 1
        let mut d = NaiveDate::from_ymd_opt(year - 1, 12, 26).unwrap();
        let end = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let mut found = None;
        while d <= end {
            if d.weekday() == Weekday::Sun {
                found = Some(d);
                break;
            }
            d += Duration::days(1);
        }
        found.unwrap_or(NaiveDate::from_ymd_opt(year - 1, 12, 30).unwrap())
    };
    if holy_family_date.year() == year {
        map.entry(holy_family_date).or_default().push(Celebration::new(
            "holy-family",
            "Sanctae Familiae",
            "The Holy Family",
            CelebrationRank::ClassI,
            CelebrationCategory::FeastOfLord,
            LiturgicalColor::White,
            4,
        ));
    }

    map
}

fn find_sunday_between(year: i32, m1: u32, d1: u32, m2: u32, d2: u32) -> Option<NaiveDate> {
    let start = NaiveDate::from_ymd_opt(year, m1, d1)?;
    let end = NaiveDate::from_ymd_opt(year, m2, d2)?;
    let mut d = start;
    while d <= end {
        if d.weekday() == Weekday::Sun {
            return Some(d);
        }
        d += Duration::days(1);
    }
    None
}

/// ~50 major fixed feasts for the 1962 calendar
fn major_feasts() -> Vec<FixedFeast> {
    vec![
        // January
        fixed(1, 1, "circumcision", "In Circumcisione Domini", "Circumcision of Our Lord", CelebrationRank::ClassI, CelebrationCategory::FeastOfLord, LiturgicalColor::White, 4),
        fixed(1, 6, "epiphany", "In Epiphania Domini", "The Epiphany of Our Lord", CelebrationRank::ClassI, CelebrationCategory::FeastOfLord, LiturgicalColor::White, 4),
        fixed(1, 25, "conversion-of-st-paul", "Conversio S. Pauli", "Conversion of St. Paul", CelebrationRank::ClassIII, CelebrationCategory::Feast, LiturgicalColor::White, 9),
        fixed(1, 28, "st-thomas-aquinas", "S. Thomae de Aquino", "St. Thomas Aquinas", CelebrationRank::ClassIII, CelebrationCategory::Feast, LiturgicalColor::White, 9),

        // February
        fixed(2, 2, "purification-bvm", "In Purificatione B.M.V.", "Purification of the BVM (Candlemas)", CelebrationRank::ClassII, CelebrationCategory::FeastOfLord, LiturgicalColor::White, 5),
        fixed(2, 22, "chair-of-st-peter", "Cathedra S. Petri", "Chair of St. Peter", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::White, 7),
        fixed(2, 24, "st-matthias", "S. Matthiae", "St. Matthias, Apostle", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::Red, 7),

        // March
        fixed(3, 7, "st-perpetua-felicity", "Ss. Perpetuae et Felicitatis", "Sts. Perpetua and Felicity, Martyrs", CelebrationRank::ClassIII, CelebrationCategory::Feast, LiturgicalColor::Red, 9),
        fixed(3, 12, "st-gregory-great", "S. Gregorii I Papae", "St. Gregory the Great, Pope and Doctor", CelebrationRank::ClassIII, CelebrationCategory::Feast, LiturgicalColor::White, 9),
        fixed(3, 17, "st-patrick", "S. Patricii", "St. Patrick, Bishop and Confessor", CelebrationRank::ClassIII, CelebrationCategory::Feast, LiturgicalColor::White, 9),
        fixed(3, 19, "st-joseph", "S. Joseph Sponsi B.M.V.", "St. Joseph, Spouse of the BVM", CelebrationRank::ClassI, CelebrationCategory::Solemnity, LiturgicalColor::White, 4),
        fixed(3, 25, "annunciation", "In Annuntiatione B.M.V.", "The Annunciation of the BVM", CelebrationRank::ClassI, CelebrationCategory::FeastOfLord, LiturgicalColor::White, 4),

        // April
        fixed(4, 2, "st-francis-of-paola", "S. Francisci de Paula", "St. Francis of Paola, Confessor", CelebrationRank::ClassIII, CelebrationCategory::Feast, LiturgicalColor::White, 9),
        fixed(4, 25, "st-mark", "S. Marci", "St. Mark, Evangelist", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::Red, 7),

        // May
        fixed(5, 1, "st-joseph-worker", "S. Joseph Opificis", "St. Joseph the Worker", CelebrationRank::ClassI, CelebrationCategory::Solemnity, LiturgicalColor::White, 4),
        fixed(5, 3, "finding-holy-cross", "Inventio S. Crucis", "Finding of the Holy Cross", CelebrationRank::ClassIII, CelebrationCategory::Feast, LiturgicalColor::Red, 9),
        fixed(5, 11, "ss-philip-james", "Ss. Philippi et Jacobi", "Sts. Philip and James, Apostles", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::Red, 7),
        fixed(5, 31, "queenship-of-mary", "B.M.V. Reginae", "Queenship of the BVM", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::White, 7),

        // June
        fixed(6, 24, "nativity-of-st-john-baptist", "In Nativitate S. Joannis Baptistae", "Nativity of St. John the Baptist", CelebrationRank::ClassI, CelebrationCategory::Solemnity, LiturgicalColor::White, 4),
        fixed(6, 29, "ss-peter-paul", "Ss. Petri et Pauli", "Sts. Peter and Paul, Apostles", CelebrationRank::ClassI, CelebrationCategory::Solemnity, LiturgicalColor::Red, 4),

        // July
        fixed(7, 2, "visitation-bvm", "Visitatio B.M.V.", "Visitation of the BVM", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::White, 7),
        fixed(7, 25, "st-james-greater", "S. Jacobi Majoris", "St. James the Greater, Apostle", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::Red, 7),
        fixed(7, 26, "st-anne", "S. Annae Matris B.M.V.", "St. Anne, Mother of the BVM", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::White, 7),

        // August
        fixed(8, 6, "transfiguration", "In Transfiguratione Domini", "The Transfiguration of Our Lord", CelebrationRank::ClassII, CelebrationCategory::FeastOfLord, LiturgicalColor::White, 5),
        fixed(8, 10, "st-lawrence", "S. Laurentii", "St. Lawrence, Martyr", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::Red, 7),
        fixed(8, 15, "assumption-bvm", "In Assumptione B.M.V.", "The Assumption of the BVM", CelebrationRank::ClassI, CelebrationCategory::Solemnity, LiturgicalColor::White, 4),
        fixed(8, 22, "immaculate-heart-of-mary", "Immaculati Cordis B.M.V.", "Immaculate Heart of Mary", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::White, 7),
        fixed(8, 24, "st-bartholomew", "S. Bartholomaei", "St. Bartholomew, Apostle", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::Red, 7),
        fixed(8, 28, "st-augustine", "S. Augustini", "St. Augustine, Bishop and Doctor", CelebrationRank::ClassIII, CelebrationCategory::Feast, LiturgicalColor::White, 9),
        fixed(8, 29, "beheading-john-baptist", "In Decollatione S. Joannis Baptistae", "Beheading of St. John the Baptist", CelebrationRank::ClassIII, CelebrationCategory::Feast, LiturgicalColor::Red, 9),

        // September
        fixed(9, 8, "nativity-bvm", "In Nativitate B.M.V.", "Nativity of the BVM", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::White, 7),
        fixed(9, 14, "exaltation-holy-cross", "In Exaltatione S. Crucis", "Exaltation of the Holy Cross", CelebrationRank::ClassII, CelebrationCategory::FeastOfLord, LiturgicalColor::Red, 5),
        fixed(9, 15, "seven-sorrows-bvm", "Septem Dolorum B.M.V.", "Seven Sorrows of the BVM", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::White, 7),
        fixed(9, 21, "st-matthew", "S. Matthaei", "St. Matthew, Apostle and Evangelist", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::Red, 7),
        fixed(9, 29, "st-michael", "Dedicatio S. Michaelis Archangeli", "St. Michael the Archangel", CelebrationRank::ClassI, CelebrationCategory::Solemnity, LiturgicalColor::White, 4),

        // October
        fixed(10, 7, "holy-rosary", "B.M.V. a Rosario", "Our Lady of the Rosary", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::White, 7),
        fixed(10, 11, "divine-motherhood-bvm", "Maternitatis B.M.V.", "Divine Motherhood of the BVM", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::White, 7),
        fixed(10, 18, "st-luke", "S. Lucae", "St. Luke, Evangelist", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::Red, 7),
        fixed(10, 28, "ss-simon-jude", "Ss. Simonis et Judae", "Sts. Simon and Jude, Apostles", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::Red, 7),

        // November
        fixed(11, 1, "all-saints", "Omnium Sanctorum", "All Saints", CelebrationRank::ClassI, CelebrationCategory::Solemnity, LiturgicalColor::White, 4),
        fixed(11, 2, "all-souls", "In Commemoratione Omnium Fidelium Defunctorum", "All Souls Day", CelebrationRank::ClassI, CelebrationCategory::Solemnity, LiturgicalColor::Black, 4),
        fixed(11, 9, "dedication-lateran", "Dedicatio Archibasilicae Ss.mi Salvatoris", "Dedication of the Lateran Basilica", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::White, 7),
        fixed(11, 21, "presentation-bvm", "Praesentatio B.M.V.", "Presentation of the BVM", CelebrationRank::ClassIII, CelebrationCategory::Feast, LiturgicalColor::White, 9),
        fixed(11, 30, "st-andrew", "S. Andreae", "St. Andrew, Apostle", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::Red, 7),

        // November (more)
        fixed(11, 11, "st-martin-of-tours", "S. Martini Episcopi", "St. Martin of Tours, Bishop", CelebrationRank::ClassIII, CelebrationCategory::Feast, LiturgicalColor::White, 9),
        fixed(11, 22, "st-cecilia", "S. Caeciliae", "St. Cecilia, Virgin and Martyr", CelebrationRank::ClassIII, CelebrationCategory::Feast, LiturgicalColor::Red, 9),
        fixed(11, 25, "st-catherine-of-alexandria", "S. Catharinae", "St. Catherine of Alexandria, Virgin and Martyr", CelebrationRank::ClassIII, CelebrationCategory::Feast, LiturgicalColor::Red, 9),

        // December
        fixed(12, 8, "immaculate-conception", "In Conceptione Immaculata B.M.V.", "Immaculate Conception of the BVM", CelebrationRank::ClassI, CelebrationCategory::Solemnity, LiturgicalColor::White, 4),
        fixed(12, 21, "st-thomas-apostle", "S. Thomae Apostoli", "St. Thomas, Apostle", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::Red, 7),
        fixed(12, 25, "christmas", "In Nativitate Domini", "The Nativity of Our Lord", CelebrationRank::ClassI, CelebrationCategory::Solemnity, LiturgicalColor::White, 1),
        fixed(12, 26, "st-stephen", "S. Stephani Protomartyris", "St. Stephen, Protomartyr", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::Red, 5),
        fixed(12, 27, "st-john-evangelist", "S. Joannis Apostoli et Evangelistae", "St. John, Apostle and Evangelist", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::White, 5),
        fixed(12, 28, "holy-innocents", "Ss. Innocentium", "Holy Innocents", CelebrationRank::ClassII, CelebrationCategory::Feast, LiturgicalColor::Red, 5),
        fixed(12, 31, "st-sylvester", "S. Silvestri I", "St. Sylvester I, Pope", CelebrationRank::ClassIII, CelebrationCategory::Feast, LiturgicalColor::White, 9),
    ]
}

fn fixed(
    month: u32,
    day: u32,
    id: &str,
    title: &str,
    title_en: &str,
    rank: CelebrationRank,
    category: CelebrationCategory,
    color: LiturgicalColor,
    precedence: u8,
) -> FixedFeast {
    FixedFeast {
        month,
        day,
        celebration: Celebration::new(id, title, title_en, rank, category, color, precedence),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanctoral_has_christmas() {
        let cycle = build_sanctoral_cycle(2026);
        let dec25 = NaiveDate::from_ymd_opt(2026, 12, 25).unwrap();
        assert!(cycle.contains_key(&dec25));
        assert!(cycle[&dec25].iter().any(|c| c.id == "christmas"));
    }

    #[test]
    fn test_sanctoral_has_all_saints() {
        let cycle = build_sanctoral_cycle(2026);
        let nov1 = NaiveDate::from_ymd_opt(2026, 11, 1).unwrap();
        assert!(cycle.contains_key(&nov1));
        assert!(cycle[&nov1].iter().any(|c| c.id == "all-saints"));
    }

    #[test]
    fn test_major_feast_count() {
        let feasts = major_feasts();
        assert!(feasts.len() >= 45, "Expected at least 45 major feasts, got {}", feasts.len());
    }

    #[test]
    fn test_epiphany_class_i() {
        let cycle = build_sanctoral_cycle(2026);
        let jan6 = NaiveDate::from_ymd_opt(2026, 1, 6).unwrap();
        let epiph = cycle[&jan6].iter().find(|c| c.id == "epiphany").unwrap();
        assert_eq!(epiph.rank, CelebrationRank::ClassI);
    }
}
