use crate::types::*;

/// Resolve precedence between temporal and sanctoral celebrations for a given day.
/// Returns (winner, commemorations).
///
/// The 1962 precedence table (simplified):
/// 1. Easter Triduum, Easter, Christmas, Pentecost (precedence 1)
/// 2. Sundays of Class I (precedence 2)
/// 3. Octave days/within octave of Class I (precedence 3)
/// 4. Solemnities of Class I (precedence 4)
/// 5. Feasts of the Lord, Class I (precedence 5) -- actually shares with Class II Lord feasts
/// 6. Sundays of Class II (precedence 6)
/// 7. Feasts of Class II (precedence 7)
/// 8. Privileged ferias (precedence 8)
/// 9. Feasts of Class III (precedence 9)
/// 10. Ferias of Advent Dec 17-23 (precedence 10)
/// 11. Ordinary ferias, Class IV (precedence 11)
pub fn resolve_precedence(
    temporal_celebration: &Celebration,
    sanctoral_celebrations: &[Celebration],
) -> (Celebration, Vec<Celebration>) {
    let mut all: Vec<&Celebration> = Vec::new();
    all.push(temporal_celebration);
    for c in sanctoral_celebrations {
        all.push(c);
    }

    // Sort by precedence number (lower wins), then by rank
    all.sort_by(|a, b| {
        a.precedence.cmp(&b.precedence)
            .then(a.rank.precedence_value().cmp(&b.rank.precedence_value()))
    });

    let winner = all[0].clone();
    let mut commemorations = Vec::new();

    // Commemorations: lower-ranked celebrations that are at least Class III
    // or privileged ferias can be commemorated.
    // In 1962 rubrics, Class IV feasts are commemorated; Class III feasts
    // impeded by a higher celebration are commemorated.
    for c in &all[1..] {
        match c.rank {
            CelebrationRank::ClassI => {
                // Class I feasts impeded are transferred, not commemorated.
                // For now we skip transfer logic and just commemorate.
                commemorations.push((*c).clone());
            }
            CelebrationRank::ClassII
            | CelebrationRank::ClassIII
            | CelebrationRank::FeriaPrivileged => {
                commemorations.push((*c).clone());
            }
            CelebrationRank::ClassIV => {
                commemorations.push((*c).clone());
            }
            CelebrationRank::Feria => {
                // Ordinary ferias are not commemorated
            }
        }
    }

    (winner, commemorations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_i_beats_class_iii() {
        let temporal = Celebration::new(
            "easter-sunday", "Dominica Resurrectionis", "Easter Sunday",
            CelebrationRank::ClassI, CelebrationCategory::Solemnity,
            LiturgicalColor::White, 1,
        );
        let sanctoral = Celebration::new(
            "some-saint", "S. Alicujus", "Some Saint",
            CelebrationRank::ClassIII, CelebrationCategory::Feast,
            LiturgicalColor::White, 9,
        );
        let (winner, comms) = resolve_precedence(&temporal, &[sanctoral.clone()]);
        assert_eq!(winner.id, "easter-sunday");
        assert_eq!(comms.len(), 1);
        assert_eq!(comms[0].id, "some-saint");
    }

    #[test]
    fn test_sanctoral_class_i_beats_sunday_class_ii() {
        let temporal = Celebration::sunday(LiturgicalSeason::AfterPentecost, 5);
        let sanctoral = Celebration::new(
            "all-saints", "Omnium Sanctorum", "All Saints",
            CelebrationRank::ClassI, CelebrationCategory::Solemnity,
            LiturgicalColor::White, 4,
        );
        let (winner, comms) = resolve_precedence(&temporal, &[sanctoral]);
        assert_eq!(winner.id, "all-saints");
        // The Sunday is commemorated
        assert!(comms.iter().any(|c| c.category == CelebrationCategory::Sunday));
    }

    #[test]
    fn test_advent_sunday_beats_class_iii_saint() {
        let temporal = Celebration::sunday(LiturgicalSeason::Advent, 2);
        let sanctoral = Celebration::new(
            "st-ambrose", "S. Ambrosii", "St. Ambrose",
            CelebrationRank::ClassIII, CelebrationCategory::Feast,
            LiturgicalColor::White, 9,
        );
        let (winner, comms) = resolve_precedence(&temporal, &[sanctoral.clone()]);
        // Advent Sunday Class I (prec 6 for week 2) vs Class III (prec 9)
        assert_eq!(winner.category, CelebrationCategory::Sunday);
        assert_eq!(comms.len(), 1);
    }

    #[test]
    fn test_privileged_feria_beats_class_iv() {
        let temporal = Celebration::new(
            "lent-feria", "Feria", "Lenten Feria",
            CelebrationRank::FeriaPrivileged, CelebrationCategory::Feria,
            LiturgicalColor::Violet, 8,
        );
        let sanctoral = Celebration::new(
            "minor-saint", "S. Minoris", "Minor Saint",
            CelebrationRank::ClassIV, CelebrationCategory::Memorial,
            LiturgicalColor::White, 11,
        );
        let (winner, comms) = resolve_precedence(&temporal, &[sanctoral]);
        assert_eq!(winner.id, "lent-feria");
        assert_eq!(comms.len(), 1);
    }

    #[test]
    fn test_no_commemorations_for_ordinary_feria() {
        let sanctoral = Celebration::new(
            "st-someone", "S. Alicujus", "St. Someone",
            CelebrationRank::ClassIII, CelebrationCategory::Feast,
            LiturgicalColor::White, 9,
        );
        let temporal = Celebration::feria(LiturgicalSeason::AfterPentecost, 5, chrono::Weekday::Tue);
        let (winner, comms) = resolve_precedence(&temporal, &[sanctoral]);
        assert_eq!(winner.id, "st-someone");
        // Ordinary feria is NOT commemorated
        assert!(comms.is_empty());
    }
}
