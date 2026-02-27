use chrono::{NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

/// Rubrical system selector
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RubricalSystem {
    Rubrics1962,
    Rubrics1955,
    PrePius,
}

impl Default for RubricalSystem {
    fn default() -> Self {
        Self::Rubrics1962
    }
}

/// Liturgical seasons in the traditional Roman calendar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiturgicalSeason {
    Advent,
    Christmas,
    AfterEpiphany,
    Septuagesima,
    Lent,
    Passiontide,
    HolyWeek,
    Easter,
    Ascensiontide,
    AfterPentecost,
}

/// 1962 ranking system
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CelebrationRank {
    /// Highest rank
    ClassI,
    ClassII,
    ClassIII,
    ClassIV,
    Feria,
    FeriaPrivileged,
}

impl CelebrationRank {
    /// Returns a numeric precedence (lower = higher rank)
    pub fn precedence_value(&self) -> u8 {
        match self {
            Self::ClassI => 1,
            Self::ClassII => 2,
            Self::ClassIII => 3,
            Self::FeriaPrivileged => 4,
            Self::ClassIV => 5,
            Self::Feria => 6,
        }
    }
}

/// Category of celebration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CelebrationCategory {
    FeastOfLord,
    Solemnity,
    Feast,
    Memorial,
    OptionalMemorial,
    Feria,
    Vigil,
    WithinOctave,
    OctaveDay,
    RogationDay,
    EmberDay,
    Sunday,
}

/// Liturgical colors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiturgicalColor {
    White,
    Red,
    Green,
    Violet,
    Rose,
    Black,
    Gold,
}

/// A celebration (feast, feria, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Celebration {
    pub id: String,
    pub title: String,
    pub title_vernacular: Option<String>,
    pub rank: CelebrationRank,
    pub category: CelebrationCategory,
    pub color: LiturgicalColor,
    /// Precedence number in the 1962 table (1-11, lower wins)
    pub precedence: u8,
}

impl Celebration {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        title_en: impl Into<String>,
        rank: CelebrationRank,
        category: CelebrationCategory,
        color: LiturgicalColor,
        precedence: u8,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            title_vernacular: Some(title_en.into()),
            rank,
            category,
            color,
            precedence,
        }
    }

    pub fn feria(season: LiturgicalSeason, week: u8, day: Weekday) -> Self {
        let (color, rank, precedence) = match season {
            LiturgicalSeason::Advent => (LiturgicalColor::Violet, CelebrationRank::FeriaPrivileged, 8),
            LiturgicalSeason::Lent | LiturgicalSeason::Passiontide => {
                (LiturgicalColor::Violet, CelebrationRank::FeriaPrivileged, 8)
            }
            LiturgicalSeason::HolyWeek => (LiturgicalColor::Violet, CelebrationRank::ClassI, 3),
            LiturgicalSeason::Christmas | LiturgicalSeason::AfterEpiphany => {
                (LiturgicalColor::White, CelebrationRank::Feria, 11)
            }
            LiturgicalSeason::Septuagesima => (LiturgicalColor::Violet, CelebrationRank::Feria, 11),
            LiturgicalSeason::Easter | LiturgicalSeason::Ascensiontide => {
                (LiturgicalColor::White, CelebrationRank::Feria, 11)
            }
            LiturgicalSeason::AfterPentecost => (LiturgicalColor::Green, CelebrationRank::Feria, 11),
        };
        let day_name = format!("{:?}", day);
        let id = format!("feria-{}-week-{}-{}", season_id(season), week, day_name.to_lowercase());
        let title = format!("Feria {} of {} Week {}", day_name, season_name(season), week);
        Self {
            id,
            title: title.clone(),
            title_vernacular: Some(title),
            rank,
            category: CelebrationCategory::Feria,
            color,
            precedence,
        }
    }

    pub fn sunday(season: LiturgicalSeason, week: u8) -> Self {
        let (color, rank, precedence) = match season {
            LiturgicalSeason::Advent => {
                if week == 1 {
                    (LiturgicalColor::Violet, CelebrationRank::ClassI, 2)
                } else if week == 3 {
                    // Gaudete Sunday
                    (LiturgicalColor::Rose, CelebrationRank::ClassI, 6)
                } else {
                    (LiturgicalColor::Violet, CelebrationRank::ClassI, 6)
                }
            }
            LiturgicalSeason::Christmas | LiturgicalSeason::AfterEpiphany => {
                (LiturgicalColor::White, CelebrationRank::ClassII, 6)
            }
            LiturgicalSeason::Septuagesima => {
                (LiturgicalColor::Violet, CelebrationRank::ClassII, 6)
            }
            LiturgicalSeason::Lent => {
                if week == 1 {
                    (LiturgicalColor::Violet, CelebrationRank::ClassI, 2)
                } else if week == 4 {
                    // Laetare Sunday
                    (LiturgicalColor::Rose, CelebrationRank::ClassI, 6)
                } else {
                    (LiturgicalColor::Violet, CelebrationRank::ClassI, 6)
                }
            }
            LiturgicalSeason::Passiontide => {
                // Passion Sunday = week 1 of Passiontide
                (LiturgicalColor::Violet, CelebrationRank::ClassI, 2)
            }
            LiturgicalSeason::HolyWeek => {
                // Palm Sunday
                (LiturgicalColor::Violet, CelebrationRank::ClassI, 2)
            }
            LiturgicalSeason::Easter => {
                if week == 1 {
                    // Easter Sunday itself handled separately
                    (LiturgicalColor::White, CelebrationRank::ClassI, 1)
                } else {
                    (LiturgicalColor::White, CelebrationRank::ClassII, 6)
                }
            }
            LiturgicalSeason::Ascensiontide => {
                (LiturgicalColor::White, CelebrationRank::ClassII, 6)
            }
            LiturgicalSeason::AfterPentecost => {
                (LiturgicalColor::Green, CelebrationRank::ClassII, 6)
            }
        };
        let id = format!("sunday-{}-{}", season_id(season), week);
        let title = format!("{} Sunday of {}", ordinal(week), season_name(season));
        Self {
            id,
            title: title.clone(),
            title_vernacular: Some(title),
            rank,
            category: CelebrationCategory::Sunday,
            color,
            precedence,
        }
    }
}

/// A complete liturgical day
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiturgicalDay {
    pub date: NaiveDate,
    pub season: LiturgicalSeason,
    pub week: u8,
    pub day_of_week: String,
    pub celebration: Celebration,
    pub commemorations: Vec<Celebration>,
    pub color: LiturgicalColor,
    /// Optional reading references (Epistle, Gospel, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readings: Option<Readings>,
    /// Optional special notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Scripture reading references for a liturgical day
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Readings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epistle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gospel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_testament: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradual: Option<String>,
}

/// Moveable feast dates for a given year
#[derive(Debug, Clone)]
pub struct MoveableFeasts {
    pub easter: NaiveDate,
    pub septuagesima: NaiveDate,
    pub ash_wednesday: NaiveDate,
    pub passion_sunday: NaiveDate,
    pub palm_sunday: NaiveDate,
    pub holy_thursday: NaiveDate,
    pub good_friday: NaiveDate,
    pub holy_saturday: NaiveDate,
    pub ascension: NaiveDate,
    pub pentecost: NaiveDate,
    pub corpus_christi: NaiveDate,
    pub sacred_heart: NaiveDate,
    pub christ_the_king: NaiveDate,
    pub advent_1: NaiveDate,
    pub ember_days: Vec<NaiveDate>,
    pub rogation_days: Vec<NaiveDate>,
}

// Helper functions

fn season_id(s: LiturgicalSeason) -> &'static str {
    match s {
        LiturgicalSeason::Advent => "advent",
        LiturgicalSeason::Christmas => "christmas",
        LiturgicalSeason::AfterEpiphany => "after-epiphany",
        LiturgicalSeason::Septuagesima => "septuagesima",
        LiturgicalSeason::Lent => "lent",
        LiturgicalSeason::Passiontide => "passiontide",
        LiturgicalSeason::HolyWeek => "holy-week",
        LiturgicalSeason::Easter => "easter",
        LiturgicalSeason::Ascensiontide => "ascensiontide",
        LiturgicalSeason::AfterPentecost => "after-pentecost",
    }
}

fn season_name(s: LiturgicalSeason) -> &'static str {
    match s {
        LiturgicalSeason::Advent => "Advent",
        LiturgicalSeason::Christmas => "Christmas",
        LiturgicalSeason::AfterEpiphany => "the Time after Epiphany",
        LiturgicalSeason::Septuagesima => "Septuagesima",
        LiturgicalSeason::Lent => "Lent",
        LiturgicalSeason::Passiontide => "Passiontide",
        LiturgicalSeason::HolyWeek => "Holy Week",
        LiturgicalSeason::Easter => "Easter",
        LiturgicalSeason::Ascensiontide => "Ascensiontide",
        LiturgicalSeason::AfterPentecost => "the Time after Pentecost",
    }
}

fn ordinal(n: u8) -> String {
    match n {
        1 => "1st".to_string(),
        2 => "2nd".to_string(),
        3 => "3rd".to_string(),
        _ => format!("{}th", n),
    }
}
