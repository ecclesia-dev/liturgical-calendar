# Traditional Roman Liturgical Calendar API — Technical Specification

**Version:** 0.1.0-draft
**Date:** 2026-02-18
**Status:** Design Phase

---

## 1. Overview

A REST API that computes the Traditional Roman Liturgical Calendar for any given date or year. The primary rubrical system is the **1962 Missale Romanum** (the "extraordinary form" / TLM), with support for **1955 (simplified Holy Week)** and **pre-1955** rubrics as alternate modes.

This is the foundational service for a suite of Catholic software tools (daily missal apps, breviary integrations, parish bulletin generators, etc.).

---

## 2. Research Summary

### 2.1 Existing Open-Source Projects

| Project | Language | Calendar Type | Notes |
|---|---|---|---|
| **Divinum Officium** | Perl + data files | 1960/1955/pre-1955 | The gold standard for traditional calendar computation. Massive corpus of Latin/vernacular texts in structured text files. Calendar logic in Perl is complex but battle-tested. Data files (`web/www/horas/`, `web/www/missa/`) are the most comprehensive open-source source for traditional propers. |
| **Romcal** | TypeScript | Novus Ordo (1969+) | Well-architected JS library, but implements post-Vatican II calendar only. Useful as an architectural reference but not for traditional rubrics. |
| **LiturgicalCalendarAPI** | PHP | Novus Ordo | PHP API with OpenAPI spec. Again, post-1969 calendar only. Good reference for API design patterns. |
| **Proprium de Tempore / Sanctis** texts | N/A | Data | Various digitized texts from pre-1962 missals available at sacred-texts.com, Catholic encyclopedias, and the Divinum Officium data corpus. |

### 2.2 Key Insight

**No existing open-source project provides a clean, modern API for the traditional (1962) calendar.** Divinum Officium has the logic and data, but it's a monolithic Perl web app, not an API. This project fills a genuine gap.

### 2.3 Language Decision: **Rust**

| Criterion | Rust | Go | TypeScript |
|---|---|---|---|
| Performance | ★★★ | ★★★ | ★★ |
| Type safety (complex rules) | ★★★ | ★★ | ★★ |
| Deployment (single binary) | ★★★ | ★★★ | ★ |
| Ecosystem (date/time) | ★★★ (chrono) | ★★★ | ★★★ |
| Long-term maintainability | ★★★ | ★★★ | ★★ |
| Community for this domain | ★★ | ★★ | ★★★ |

**Recommendation: Rust** with `axum` web framework.

**Rationale:**
- Calendar computation is pure logic with complex precedence rules — Rust's type system (enums, pattern matching) maps naturally to liturgical ranks, classes, and precedence tables
- Single-binary deployment is ideal for self-hosting by parishes/individuals
- The `chrono` crate handles date computation (Easter/computus) elegantly
- Performance means a single $5 VPS can serve the entire traditional Catholic community
- Correctness matters enormously (people pray from this) — Rust's compiler catches bugs

**Alternative considered:** Go is a fine second choice if the team prefers it. TypeScript is acceptable for prototyping but less ideal for a long-lived reference implementation.

### 2.4 Data Sources

1. **Divinum Officium data files** (GPL): The primary source for texts, rubrics, and calendar rules. These structured text files can be parsed and converted to JSON/TOML data.
2. **1962 Missale Romanum** (Typis Polyglottis Vaticanis): The authoritative source for rubrical rules.
3. **Rubricæ Generales Missalis Romani (1960)**: The codified rubrical norms.
4. **Canon Law / Decrees of the SRC**: For precedence tables and specific feast classifications.

---

## 3. Domain Model

### 3.1 Rubrical System

```rust
enum RubricalSystem {
    Rubrics1962,    // Default. Post-1960 general rubrics, 1962 Missal
    Rubrics1955,    // Pius XII simplified Holy Week (1955-1961)
    PrePius,        // Pre-1955 (Divino Afflatu 1911 through 1954)
}
```

### 3.2 Liturgical Day

```rust
struct LiturgicalDay {
    date: NaiveDate,
    season: LiturgicalSeason,
    week: u8,                        // Week within season
    day_of_week: Weekday,

    // Primary celebration
    celebration: Celebration,

    // Commemorations (in order of precedence)
    commemorations: Vec<Commemoration>,

    // Liturgical color
    color: LiturgicalColor,

    // Readings for Mass
    readings: Readings,

    // Optional: Matins readings, Office hymn, etc. (future expansion)
}
```

### 3.3 Celebration

```rust
struct Celebration {
    id: String,                       // e.g., "nativity-of-lord", "st-thomas-aquinas"
    title: String,                    // "In Nativitate Domini"
    title_vernacular: Option<String>, // "The Nativity of Our Lord"
    rank: CelebrationRank,
    category: CelebrationCategory,
    saint: Option<Saint>,
    proper_texts: Option<ProperTexts>,
}

/// 1962 ranking system (4 classes replaced the old "double/semidouble" system)
enum CelebrationRank {
    ClassI,       // Highest: Easter, Christmas, etc.
    ClassII,      // Major feasts: Apostles, Doctors
    ClassIII,     // Most saints' feasts
    ClassIV,      // Simple commemorations
    Feria,        // Weekday
    FeriaPrivileged,  // Privileged ferias (Advent, Lent, Ember Days)
}

enum CelebrationCategory {
    Solemnity,    // Mapped for API clarity
    Feast,
    Memorial,
    OptionalMemorial,
    Feria,
    Vigil,
    WithinOctave,
    OctaveDay,
    RogationDay,
    EmberDay,
}

enum LiturgicalSeason {
    Advent,
    Christmas,      // Christmas to Septuagesima
    Septuagesima,   // Pre-Lent (Septuagesima, Sexagesima, Quinquagesima)
    Lent,           // Ash Wednesday to Passion Sunday
    Passiontide,    // Passion Sunday to Holy Thursday
    HolyWeek,       // Palm Sunday to Holy Saturday
    SacredTriduum,  // Holy Thursday evening to Easter Vigil
    Easter,         // Easter to Ascension
    Ascensiontide,  // Ascension to Pentecost
    AfterPentecost, // Pentecost to Advent
    // Note: "Time after Epiphany" is part of Christmas in 1962
}

enum LiturgicalColor {
    White,
    Red,
    Green,
    Violet,
    Rose,    // Gaudete & Laetare
    Black,   // Requiem Masses, Good Friday
    Gold,    // Permitted as substitute for White/Red/Green
}
```

### 3.4 Readings

```rust
struct Readings {
    epistle: ReadingReference,
    gospel: ReadingReference,
    // Traditional Mass has Introit, Gradual, etc. but those are propers, not "readings"
    // Include them under proper_texts
}

struct ReadingReference {
    reference: String,      // e.g., "Rom 13:11-14"
    text_latin: Option<String>,
    text_vernacular: Option<String>,
}

struct ProperTexts {
    introit: Option<TextPair>,
    collect: Option<TextPair>,
    epistle: Option<TextPair>,
    gradual: Option<TextPair>,     // or Tract, or Alleluia
    sequence: Option<TextPair>,    // Dies Irae, Veni Sancte Spiritus, etc.
    gospel: Option<TextPair>,
    offertory: Option<TextPair>,
    secret: Option<TextPair>,
    communion: Option<TextPair>,
    postcommunion: Option<TextPair>,
    last_gospel: Option<TextPair>, // Usually John 1:1-14 unless proper
}

struct TextPair {
    latin: String,
    vernacular: Option<String>,  // Configurable language
}
```

### 3.5 Saint

```rust
struct Saint {
    id: String,
    name: String,
    titles: Vec<String>,         // "Bishop", "Doctor of the Church", "Virgin", "Martyr"
    feast_day: MonthDay,         // Fixed date (month/day)
    biography_short: Option<String>,
}
```

---

## 4. Calendar Computation Engine

### 4.1 Core Algorithm

The calendar engine resolves each day of the year through this pipeline:

```
1. Compute moveable feast dates for the year
   ├── Easter (Computus algorithm)
   ├── Septuagesima (Easter - 63 days)
   ├── Ash Wednesday (Easter - 46 days)
   ├── Ascension (Easter + 39 days)
   ├── Pentecost (Easter + 49 days)
   ├── Corpus Christi (Easter + 60 days)
   ├── Sacred Heart (Friday after Corpus Christi octave)
   ├── Christ the King (last Sunday of October)
   ├── Ember Days (4 sets per year)
   └── Rogation Days (Mon-Wed before Ascension)

2. Build the Temporal Cycle (de Tempore)
   - Assign season/week to every day of the year
   - Place all moveable feasts/ferias

3. Build the Sanctoral Cycle (de Sanctis)
   - Load all fixed feasts from the calendar data
   - Filter by rubrical system (some feasts differ 1962 vs pre-1955)

4. Resolve Precedence (the hard part)
   - For each day, determine which celebration wins
   - Apply the Table of Precedence (1960 rubrics, Title XI)
   - Handle translations (transferred feasts)
   - Determine commemorations
   - Handle occurring/concurring (Vespers)
```

### 4.2 Precedence Table (1962 Rubrics, simplified)

| Priority | Examples |
|---|---|
| 1 | Easter Triduum, Easter, Christmas, Pentecost |
| 2 | Sundays of Class I (Advent I, Lent I, Passion, Palm) |
| 3 | Days within Octave of Christmas; Ash Wednesday; Holy Week ferias |
| 4 | Solemnities of Class I (Immaculate Conception, All Saints, etc.) |
| 5 | Feasts of the Lord, Class I |
| 6 | Sundays of Class II |
| 7 | Feasts of Class II |
| 8 | Privileged ferias (Advent, Lent, Ember Days) |
| 9 | Feasts of Class III |
| 10 | Ferias of Advent (Dec 17-23) |
| 11 | Ordinary ferias, Class IV commemorations |

### 4.3 Octaves (varies by rubrical system)

- **1962**: Only Easter and Christmas octaves survive
- **Pre-1955**: Many more octaves (Epiphany, Ascension, Corpus Christi, Sacred Heart, Assumption, All Saints, etc.)

### 4.4 Vigils (varies by rubrical system)

- **1962**: Christmas, Pentecost, Assumption, John the Baptist, Sts. Peter & Paul, St. Lawrence (simplified)
- **Pre-1955**: Many more vigils with fasting obligations

---

## 5. REST API Design

### 5.1 Base URL

```
https://api.ecclesiadev.com/v1 (forthcoming)
```

### 5.2 Endpoints

#### `GET /v1/calendar/{date}`

Returns the liturgical day for a specific date.

**Path Parameters:**
- `date` — ISO 8601 date (`YYYY-MM-DD`)

**Query Parameters:**
- `rubrics` — `1962` (default), `1955`, `pre1955`
- `lang` — Vernacular language code: `en` (default), `fr`, `de`, `es`, `pt`, `it`
- `include` — Comma-separated: `propers`, `readings`, `saints`, `all` (default: basic info only)
- `diocese` — Optional diocese code for local proper feasts (e.g., `rome`, `paris`)

**Response:**
```json
{
  "date": "2026-01-28",
  "rubrics": "1962",
  "season": "after_epiphany",
  "season_week": 3,
  "day_of_week": "wednesday",
  "celebration": {
    "id": "st-thomas-aquinas",
    "title": "S. Thomæ de Aquino",
    "title_vernacular": "St. Thomas Aquinas",
    "rank": "class_iii",
    "category": "feast",
    "color": "white",
    "saint": {
      "id": "thomas-aquinas",
      "name": "St. Thomas Aquinas",
      "titles": ["Confessor", "Doctor of the Church"],
      "feast_day": "01-28"
    }
  },
  "commemorations": [
    {
      "id": "st-peter-nolasco",
      "title": "S. Petri Nolasci",
      "title_vernacular": "St. Peter Nolasco",
      "rank": "class_iii"
    }
  ],
  "color": "white",
  "readings": {
    "epistle": { "reference": "Sap 7:7-14" },
    "gospel": { "reference": "Matt 5:13-19" }
  }
}
```

#### `GET /v1/calendar/{year}`

Returns the full calendar for a year.

**Path Parameters:**
- `year` — 4-digit year (`YYYY`)

**Query Parameters:** Same as above.

**Response:** Array of `LiturgicalDay` objects (365/366 items). Supports streaming (`Accept: application/x-ndjson`).

#### `GET /v1/calendar/{year}/{month}`

Returns all days in a given month.

#### `GET /v1/calendar/today`

Shorthand for today's date. Requires `tz` query parameter (e.g., `tz=America/Chicago`).

#### `GET /v1/propers/{celebration_id}`

Returns the full proper texts for a given celebration.

**Query Parameters:**
- `lang` — Language code
- `rubrics` — Rubrical system

**Response:**
```json
{
  "id": "st-thomas-aquinas",
  "introit": {
    "latin": "In médio Ecclésiæ apéruit os ejus...",
    "vernacular": "In the midst of the Church he opened his mouth..."
  },
  "collect": { "latin": "...", "vernacular": "..." },
  "epistle": { "reference": "Sap 7:7-14", "latin": "...", "vernacular": "..." },
  "gradual": { "latin": "...", "vernacular": "..." },
  "gospel": { "reference": "Matt 5:13-19", "latin": "...", "vernacular": "..." },
  "offertory": { "latin": "...", "vernacular": "..." },
  "secret": { "latin": "...", "vernacular": "..." },
  "communion": { "latin": "...", "vernacular": "..." },
  "postcommunion": { "latin": "...", "vernacular": "..." }
}
```

#### `GET /v1/saints/{saint_id}`

Returns saint information.

#### `GET /v1/saints?month={MM}&day={DD}`

Returns saints for a given month/day.

#### `GET /v1/seasons/{year}`

Returns season boundaries for a given year (start/end dates for each liturgical season).

#### `GET /v1/moveable-feasts/{year}`

Returns computed dates of all moveable feasts for a year.

### 5.3 Response Conventions

- All responses: `Content-Type: application/json`
- Dates: ISO 8601 (`YYYY-MM-DD`)
- Enums: lowercase snake_case strings
- Latin text always included; vernacular included when `lang` specified and translation available
- Pagination: Not needed (max response is 366 days)
- Errors: Standard HTTP codes with `{ "error": "...", "message": "..." }`

### 5.4 Rate Limiting & Caching

- Calendar data is deterministic — aggressive HTTP caching (`Cache-Control: public, max-age=86400`)
- Full-year endpoints: `ETag` based on year+rubrics hash
- No authentication required for public instance
- Rate limit: 100 req/min per IP (configurable)

---

## 6. Architecture

```
┌─────────────────────────────────────────────┐
│                  axum HTTP                   │
│              (REST endpoints)                │
├─────────────────────────────────────────────┤
│              Calendar Service                │
│   ┌──────────┐  ┌──────────┐  ┌──────────┐ │
│   │ Temporal  │  │Sanctoral │  │Precedence│ │
│   │  Cycle    │  │  Cycle   │  │ Resolver │ │
│   └──────────┘  └──────────┘  └──────────┘ │
├─────────────────────────────────────────────┤
│                Computus Engine               │
│          (Easter + moveable feasts)          │
├─────────────────────────────────────────────┤
│               Data Layer                     │
│   ┌──────────┐  ┌──────────┐  ┌──────────┐ │
│   │  Saints   │  │ Propers  │  │ Readings │ │
│   │  (TOML)   │  │  (TOML)  │  │  (TOML)  │ │
│   └──────────┘  └──────────┘  └──────────┘ │
└─────────────────────────────────────────────┘
```

### 6.1 Key Design Decisions

1. **No database.** All data is embedded in the binary from TOML/JSON files at compile time (`include_str!` or `rust-embed`). The calendar is fully deterministic — no state needed.

2. **TOML for data files.** More readable than JSON for the liturgical text corpus. Structured enough for parsing. Easy for contributors (priests, scholars) to edit.

3. **Rubrical system as a strategy pattern.** Each rubrical variant (1962, 1955, pre-1955) implements a `RubricalSystem` trait that provides:
   - Precedence table
   - Octave rules
   - Vigil rules
   - Holy Week ordo
   - Feast classifications (some feasts change rank between systems)

4. **Compute on request, cache aggressively.** A full year computation takes <10ms. Cache at the HTTP layer. No need for pre-computation.

5. **Propers as a separate data concern.** Calendar computation (what feast is today?) is separate from text retrieval (what is the Introit?). The `/calendar` endpoint returns structure; `/propers` returns texts. The `include=propers` parameter joins them for convenience.

### 6.2 Crate Structure

```
liturgical-calendar/
├── Cargo.toml
├── crates/
│   ├── calendar-core/       # Pure computation, no I/O
│   │   ├── src/
│   │   │   ├── computus.rs      # Easter algorithm
│   │   │   ├── temporal.rs      # Temporal cycle
│   │   │   ├── sanctoral.rs     # Sanctoral cycle
│   │   │   ├── precedence.rs    # Resolution logic
│   │   │   ├── rubrics/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── r1962.rs
│   │   │   │   ├── r1955.rs
│   │   │   │   └── pre1955.rs
│   │   │   └── types.rs         # All domain types
│   │   └── Cargo.toml
│   ├── calendar-data/       # TOML data files + parsing
│   │   ├── src/
│   │   ├── data/
│   │   │   ├── sanctoral/       # One file per month or per saint
│   │   │   ├── temporal/        # Proper texts for Sundays/ferias
│   │   │   ├── commons/         # Common of Martyrs, Virgins, etc.
│   │   │   └── readings/        # Scripture references
│   │   └── Cargo.toml
│   └── calendar-api/       # axum web server
│       ├── src/
│       │   ├── main.rs
│       │   ├── routes.rs
│       │   ├── handlers.rs
│       │   └── responses.rs     # JSON serialization
│       └── Cargo.toml
├── data/                    # Source TOML files (symlinked or copied into calendar-data)
├── tests/
│   ├── known_dates.rs       # Test against known liturgical calendars
│   └── regression.rs
└── README.md
```

---

## 7. Data Encoding

### 7.1 Sanctoral Data (example TOML)

```toml
# data/sanctoral/01-january.toml

[[feasts]]
id = "circumcision"
date = "01-01"
title_la = "In Circumcisione Domini et Octava Nativitatis"
title_en = "The Circumcision of Our Lord and Octave Day of Christmas"
rank_1962 = "class_i"
rank_pre1955 = "double_ii_class"
color = "white"
category = "feast_of_lord"
proper = "circumcision"  # references propers file

[[feasts]]
id = "holy-name-of-jesus"
date = "sunday_between_01-02_and_01-05"  # Moveable within range
fallback_date = "01-02"                    # If no Sunday exists
title_la = "Ss.mi Nominis Jesu"
title_en = "The Most Holy Name of Jesus"
rank_1962 = "class_ii"
color = "white"
category = "feast_of_lord"
proper = "holy-name"
```

### 7.2 Propers Data (example TOML)

```toml
# data/propers/sanctoral/st-thomas-aquinas.toml

[introit]
latin = "In médio Ecclésiæ apéruit os ejus: et implévit eum Dóminus spíritu sapiéntiæ et intelléctus: stolam glóriæ índuit eum."
english = "In the midst of the Church he opened his mouth: and the Lord filled him with the spirit of wisdom and understanding: He clothed him with a robe of glory."
reference = "Eccli 15:5"

[collect]
latin = "Deus, qui Ecclésiam tuam beáti Thomæ Confessóris tui mira eruditióne clarifícas..."
english = "O God, Who dost make Thy Church illustrious by the wondrous learning of blessed Thomas, Thy Confessor..."

[epistle]
reference = "Sap 7:7-14"
latin = "Optávi, et datus est mihi sensus..."
english = "I wished, and understanding was given to me..."

[gospel]
reference = "Matt 5:13-19"
latin = "In illo témpore: Dixit Jesus discípulis suis: Vos estis sal terræ..."
english = "At that time, Jesus said to His disciples: You are the salt of the earth..."
```

---

## 8. Testing Strategy

### 8.1 Known-Date Tests

Validate against published Ordo calendars (FSSP, ICKSP, diocesan ordos):

```rust
#[test]
fn easter_2026() {
    assert_eq!(computus(2026), NaiveDate::from_ymd(2026, 4, 5));
}

#[test]
fn ash_wednesday_2026() {
    let cal = Calendar::new(2026, Rubrics1962);
    let day = cal.get(NaiveDate::from_ymd(2026, 2, 18));
    assert_eq!(day.celebration.id, "ash-wednesday");
    assert_eq!(day.color, LiturgicalColor::Violet);
    assert_eq!(day.rank, CelebrationRank::FeriaPrivileged);
}

#[test]
fn transferred_feast() {
    // When a Class III feast falls on a Class I Sunday, it's commemorated
    // Verify the precedence resolver handles this correctly
}
```

### 8.2 Cross-Validation

- Compare output against Divinum Officium for every day of multiple years
- Automated comparison script parsing DO's web output

### 8.3 Property-Based Tests

- Every day must have exactly one primary celebration
- Every day must have a color
- Easter must always fall between March 22 and April 25
- Lent must always be 40 days (excluding Sundays) before Easter

---

## 9. Build Plan

### Phase 1: Core Engine (Weeks 1-4)
- [ ] Computus algorithm + moveable feast computation
- [ ] Temporal cycle builder (all seasons, weeks, ferias)
- [ ] Domain types (all enums, structs)
- [ ] Basic sanctoral cycle (major feasts only: ~50 key dates)
- [ ] Precedence resolver (1962 rules)
- [ ] Known-date test suite (50+ dates)

### Phase 2: Complete Sanctoral + API (Weeks 5-8)
- [ ] Full sanctoral calendar (~300 feasts for 1962)
- [ ] axum REST API (all endpoints except `/propers`)
- [ ] JSON serialization
- [ ] HTTP caching headers
- [ ] OpenAPI specification (auto-generated via `utoipa`)
- [ ] Docker image

### Phase 3: Propers & Texts (Weeks 9-14)
- [ ] Parse/convert Divinum Officium data files to TOML
- [ ] Propers endpoint
- [ ] Latin + English texts for all Sundays/major feasts
- [ ] Commons (Common of Martyrs, Virgins, Confessors, etc.)
- [ ] Readings references

### Phase 4: Alternate Rubrics & Polish (Weeks 15-18)
- [ ] 1955 rubrical variant
- [ ] Pre-1955 rubrical variant (octaves, additional vigils, old ranking)
- [ ] Cross-validation against Divinum Officium (automated)
- [ ] Additional languages (start with French, Spanish)
- [ ] Diocese-specific propers framework
- [ ] Public deployment + documentation site

### Phase 5: Ecosystem (Ongoing)
- [ ] `calendar-core` published as standalone crate on crates.io
- [ ] Client SDKs (TypeScript, Swift, Kotlin)
- [ ] iCal/webcal feed endpoint (`/v1/calendar/{year}.ics`)
- [ ] RSS/Atom feed for daily celebrations
- [ ] Webhook support (daily notifications)
- [ ] Breviary/Office integration

---

## 10. Deployment

- **Binary:** Single static binary, ~10-20MB with embedded data
- **Docker:** `FROM scratch` image with just the binary
- **Public instance:** Fly.io or similar (low-cost, global edge)
- **Self-hosted:** Download binary, run it. No dependencies.
- **Configuration:** Environment variables only (`PORT`, `DEFAULT_LANG`, `LOG_LEVEL`)

---

## 11. License

**Dual license:**
- Code: **MIT OR Apache-2.0** (standard Rust ecosystem)
- Liturgical data derived from Divinum Officium: **GPL v3** (respecting upstream)
- Original text data (public domain liturgical texts): **CC0**

---

## 12. Open Questions

1. **Ordo for religious orders?** Dominicans, Benedictines, etc. have their own calendars. Support as plugins?
2. **Votive Masses?** Not tied to the calendar per se, but frequently requested.
3. **Pre-1911 rubrics?** Some communities use even older forms. Scope creep risk.
4. **Vernacular translations:** How many languages at launch? English is essential; Latin is always present.
5. **Lectionary vs. Missal readings:** The traditional missal has readings embedded in the propers. The "readings" field should match the Missal, not the modern lectionary.

---

## Appendix A: Computus (Easter Algorithm)

The anonymous Gregorian Easter algorithm:

```rust
pub fn computus(year: i32) -> NaiveDate {
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
    NaiveDate::from_ymd_opt(year, month as u32, day as u32).unwrap()
}
```

## Appendix B: Key Dependencies

```toml
[dependencies]
axum = "0.8"
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.6", features = ["cors", "compression-gzip"] }
utoipa = { version = "5", features = ["axum_extras"] }
tracing = "0.1"
tracing-subscriber = "0.3"
rust-embed = "8"
```
