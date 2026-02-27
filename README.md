# Traditional Roman Liturgical Calendar

Data and API for the 1962 Roman Calendar (Extraordinary Form / Traditional Latin Mass), built in Rust.

## What It Provides

- **Moveable feasts** — Easter (computus), Septuagesima, Ash Wednesday, Ascension, Pentecost, Ember Days, and all dependent dates
- **Sanctoral cycle** — Fixed feasts, saints' days, and vigils for the full liturgical year
- **Canonical hours** — Season and rank data needed for Divine Office / Breviary integration
- **.ics export** — Pre-built iCalendar files (2024–2050) for import into Apple Calendar, Google Calendar, and other clients
- **Precedence resolution** — 1962 rubrical precedence tables with support for 1955 and pre-1955 variants

## Usage

Import the `.ics` file into your calendar application:

1. Download `traditional-roman-calendar.ics`
2. Open it with your calendar app, or import via URL/file
3. All traditional feasts, seasons, and ranks appear as calendar events

For the REST API (forthcoming), see `SPEC.md` for endpoint documentation.

## License

- **Code:** MIT OR Apache-2.0 (at your option)
- **Liturgical data** (derived from Divinum Officium): GPL v3
- **Public-domain liturgical texts:** CC0 1.0 Universal

See `LICENSE` for full details.

## Credits

Built by Thomas. DevOps by Albert. Security audit by Athanasius. Reviewed by Bellarmine (liturgical accuracy) and Pius (content alignment).
