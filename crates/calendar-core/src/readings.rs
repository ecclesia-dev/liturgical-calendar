use crate::types::Readings;

/// Get the scripture readings for a celebration by its ID.
/// These are references from the 1962 Missale Romanum.
pub fn get_readings(celebration_id: &str) -> Option<Readings> {
    let r = match celebration_id {
        // Christmas & Epiphany cycle
        "christmas" => Readings {
            epistle: Some("Titus 2:11-15".into()),
            gospel: Some("Luke 2:1-14".into()),
            old_testament: None,
            gradual: Some("Ps 97:3-4, 2".into()),
        },
        "circumcision" => Readings {
            epistle: Some("Titus 2:11-15".into()),
            gospel: Some("Luke 2:21".into()),
            old_testament: None,
            gradual: Some("Ps 97:3-4".into()),
        },
        "epiphany" => Readings {
            epistle: Some("Isaias 60:1-6".into()),
            gospel: Some("Matt 2:1-12".into()),
            old_testament: None,
            gradual: Some("Ps 71:10-11".into()),
        },
        "holy-family" => Readings {
            epistle: Some("Col 3:12-17".into()),
            gospel: Some("Luke 2:42-52".into()),
            old_testament: None,
            gradual: Some("Ps 26:4".into()),
        },
        "holy-name-of-jesus" => Readings {
            epistle: Some("Acts 4:8-12".into()),
            gospel: Some("Luke 2:21".into()),
            old_testament: None,
            gradual: Some("Ps 105:47".into()),
        },

        // Lent & Passiontide
        "ash-wednesday" => Readings {
            epistle: Some("Joel 2:12-19".into()),
            gospel: Some("Matt 6:16-21".into()),
            old_testament: None,
            gradual: Some("Ps 56:2".into()),
        },
        "palm-sunday" => Readings {
            epistle: Some("Phil 2:5-11".into()),
            gospel: Some("Matt 26:36–27:60 (Passion)".into()),
            old_testament: None,
            gradual: Some("Ps 21:2-3, 22".into()),
        },

        // Sacred Triduum
        "holy-thursday" => Readings {
            epistle: Some("1 Cor 11:20-32".into()),
            gospel: Some("John 13:1-15".into()),
            old_testament: None,
            gradual: Some("Phil 2:8-9".into()),
        },
        "good-friday" => Readings {
            epistle: Some("Osee 6:1-6; Exod 12:1-11".into()),
            gospel: Some("John 18:1–19:42 (Passion)".into()),
            old_testament: Some("Isaias 52:13–53:12".into()),
            gradual: Some("Hab 3:2-3".into()),
        },
        "holy-saturday" => Readings {
            epistle: Some("Col 3:1-4".into()),
            gospel: Some("Matt 28:1-7".into()),
            old_testament: Some("Gen 1:1–2:2 (and Prophecies)".into()),
            gradual: None,
        },

        // Easter cycle
        "easter-sunday" => Readings {
            epistle: Some("1 Cor 5:7-8".into()),
            gospel: Some("Mark 16:1-7".into()),
            old_testament: None,
            gradual: Some("Ps 117:24, 1 (Sequence: Victimae Paschali Laudes)".into()),
        },
        "low-sunday" => Readings {
            epistle: Some("1 John 5:4-10".into()),
            gospel: Some("John 20:19-31".into()),
            old_testament: None,
            gradual: Some("Matt 28:7; John 20:26".into()),
        },
        "ascension" => Readings {
            epistle: Some("Acts 1:1-11".into()),
            gospel: Some("Mark 16:14-20".into()),
            old_testament: None,
            gradual: Some("Ps 46:6; Ps 67:18-19".into()),
        },
        "pentecost" => Readings {
            epistle: Some("Acts 2:1-11".into()),
            gospel: Some("John 14:23-31".into()),
            old_testament: None,
            gradual: Some("Ps 103:30 (Sequence: Veni Sancte Spiritus)".into()),
        },

        // Corpus Christi & Sacred Heart
        "corpus-christi" => Readings {
            epistle: Some("1 Cor 11:23-29".into()),
            gospel: Some("John 6:56-59".into()),
            old_testament: None,
            gradual: Some("Ps 144:15-16 (Sequence: Lauda Sion)".into()),
        },
        "sacred-heart" => Readings {
            epistle: Some("Eph 3:8-19".into()),
            gospel: Some("John 19:31-37".into()),
            old_testament: None,
            gradual: Some("Ps 24:8-9".into()),
        },

        // Christ the King
        "christ-the-king" => Readings {
            epistle: Some("Col 1:12-20".into()),
            gospel: Some("John 18:33-37".into()),
            old_testament: None,
            gradual: Some("Ps 71:8, 11".into()),
        },

        // Major sanctoral feasts
        "purification-bvm" => Readings {
            epistle: Some("Mal 3:1-4".into()),
            gospel: Some("Luke 2:22-32".into()),
            old_testament: None,
            gradual: Some("Ps 47:10-11, 9".into()),
        },
        "st-joseph" => Readings {
            epistle: Some("Ecclus 45:1-6".into()),
            gospel: Some("Matt 1:18-21".into()),
            old_testament: None,
            gradual: Some("Ps 20:4-5".into()),
        },
        "annunciation" => Readings {
            epistle: Some("Isaias 7:10-15".into()),
            gospel: Some("Luke 1:26-38".into()),
            old_testament: None,
            gradual: Some("Ps 44:3, 5".into()),
        },
        "nativity-of-st-john-baptist" => Readings {
            epistle: Some("Isaias 49:1-3, 5-7".into()),
            gospel: Some("Luke 1:57-68".into()),
            old_testament: None,
            gradual: Some("Jer 1:5, 9".into()),
        },
        "ss-peter-paul" => Readings {
            epistle: Some("Acts 12:1-11".into()),
            gospel: Some("Matt 16:13-19".into()),
            old_testament: None,
            gradual: Some("Ps 44:17-18".into()),
        },
        "transfiguration" => Readings {
            epistle: Some("2 Peter 1:16-19".into()),
            gospel: Some("Matt 17:1-9".into()),
            old_testament: None,
            gradual: Some("Ps 44:3".into()),
        },
        "assumption-bvm" => Readings {
            epistle: Some("Judith 13:22-25; 15:10".into()),
            gospel: Some("Luke 1:41-50".into()),
            old_testament: None,
            gradual: Some("Ps 44:10, 12, 16".into()),
        },
        "exaltation-holy-cross" => Readings {
            epistle: Some("Phil 2:5-11".into()),
            gospel: Some("John 12:31-36".into()),
            old_testament: None,
            gradual: Some("Phil 2:8-9".into()),
        },
        "st-michael" => Readings {
            epistle: Some("Apoc 1:1-5".into()),
            gospel: Some("Matt 18:1-10".into()),
            old_testament: None,
            gradual: Some("Ps 102:20".into()),
        },
        "all-saints" => Readings {
            epistle: Some("Apoc 7:2-12".into()),
            gospel: Some("Matt 5:1-12".into()),
            old_testament: None,
            gradual: Some("Ps 33:10-11".into()),
        },
        "all-souls" => Readings {
            epistle: Some("1 Cor 15:51-57".into()),
            gospel: Some("John 5:25-29".into()),
            old_testament: None,
            gradual: None,
        },
        "immaculate-conception" => Readings {
            epistle: Some("Prov 8:22-35".into()),
            gospel: Some("Luke 1:26-28".into()),
            old_testament: None,
            gradual: Some("Judith 15:10; 13:23".into()),
        },

        // Dec 25 octave
        "st-stephen" => Readings {
            epistle: Some("Acts 6:8-10; 7:54-59".into()),
            gospel: Some("Matt 23:34-39".into()),
            old_testament: None,
            gradual: Some("Ps 118:23, 86, 23".into()),
        },
        "st-john-evangelist" => Readings {
            epistle: Some("Ecclus 15:1-6".into()),
            gospel: Some("John 21:19-24".into()),
            old_testament: None,
            gradual: Some("Ps 91:13-14".into()),
        },
        "holy-innocents" => Readings {
            epistle: Some("Apoc 14:1-5".into()),
            gospel: Some("Matt 2:13-18".into()),
            old_testament: None,
            gradual: Some("Ps 123:7-8".into()),
        },

        _ => return None,
    };
    Some(r)
}

/// Get special notes for a celebration.
pub fn get_notes(celebration_id: &str) -> Option<String> {
    match celebration_id {
        "ash-wednesday" => Some("Blessing and imposition of ashes. Fast and abstinence.".into()),
        "palm-sunday" => Some("Blessing of palms and procession before Mass.".into()),
        "holy-thursday" => Some("Mass of the Lord's Supper. Mandatum. Stripping of the altars. Repository.".into()),
        "good-friday" => Some("Solemn liturgical action. Veneration of the Cross. Fast and abstinence. No Mass celebrated.".into()),
        "holy-saturday" => Some("Easter Vigil: Blessing of the new fire, Paschal candle, baptismal water. First Mass of Easter.".into()),
        "easter-sunday" => Some("Solemnity of solemnities. Sequence: Victimae Paschali Laudes.".into()),
        "pentecost" => Some("Sequence: Veni Sancte Spiritus.".into()),
        "corpus-christi" => Some("Sequence: Lauda Sion Salvatorem. Procession of the Blessed Sacrament.".into()),
        "purification-bvm" => Some("Candlemas. Blessing of candles and procession.".into()),
        "all-souls" => Some("Commemoration of All the Faithful Departed. Three Masses permitted for each priest.".into()),
        "christmas" => Some("Solemnity of the Nativity. Three Masses: Midnight, Dawn, Day.".into()),
        "circumcision" => Some("Octave Day of Christmas. Holy Day of Obligation.".into()),
        "epiphany" => Some("Holy Day of Obligation. Blessing of water, chalk, and incense.".into()),
        "assumption-bvm" => Some("Holy Day of Obligation.".into()),
        "all-saints" => Some("Holy Day of Obligation.".into()),
        "immaculate-conception" => Some("Holy Day of Obligation.".into()),
        "ascension" => Some("Holy Day of Obligation.".into()),
        "st-joseph" => Some("Holy Day of Obligation in many countries.".into()),
        "ss-peter-paul" => Some("Holy Day of Obligation in many countries.".into()),
        _ => None,
    }
}
