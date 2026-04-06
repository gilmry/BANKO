/// Pure domain fuzzy matching — no external dependencies.
/// Classic Levenshtein distance (edit distance) via dynamic programming.
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut prev = vec![0usize; n + 1];
    let mut curr = vec![0usize; n + 1];

    for (j, val) in prev.iter_mut().enumerate().take(n + 1) {
        *val = j;
    }

    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[n]
}

/// Compute similarity score 0-100 from Levenshtein distance.
pub fn similarity_score(a: &str, b: &str) -> u8 {
    if a.is_empty() && b.is_empty() {
        return 100;
    }
    let max_len = a.len().max(b.len());
    if max_len == 0 {
        return 100;
    }
    let dist = levenshtein_distance(a, b);
    ((1.0 - (dist as f64 / max_len as f64)) * 100.0).round() as u8
}

/// Normalize a name for comparison: lowercase, remove diacritics, trim, collapse whitespace.
pub fn normalize_name(name: &str) -> String {
    let lower = name.to_lowercase();
    let mut result = String::with_capacity(lower.len());

    for c in lower.chars() {
        let replacement = match c {
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => 'a',
            'æ' => {
                result.push('a');
                'e'
            }
            'ç' => 'c',
            'è' | 'é' | 'ê' | 'ë' => 'e',
            'ì' | 'í' | 'î' | 'ï' => 'i',
            'ð' => 'd',
            'ñ' => 'n',
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' => 'o',
            'ø' => 'o',
            'ù' | 'ú' | 'û' | 'ü' => 'u',
            'ý' | 'ÿ' => 'y',
            'þ' => 't',
            'ß' => {
                result.push('s');
                's'
            }
            _ => c,
        };
        result.push(replacement);
    }

    // Collapse whitespace
    let parts: Vec<&str> = result.split_whitespace().collect();
    parts.join(" ")
}

/// Compare two names with threshold. Returns Some(score) if >= threshold, None otherwise.
pub fn compare_names(name1: &str, name2: &str, threshold: u8) -> Option<u8> {
    let n1 = normalize_name(name1);
    let n2 = normalize_name(name2);

    // Direct comparison
    let direct_score = similarity_score(&n1, &n2);
    if direct_score >= threshold {
        return Some(direct_score);
    }

    // Try word-order permutation: sort words and compare
    let mut words1: Vec<&str> = n1.split_whitespace().collect();
    let mut words2: Vec<&str> = n2.split_whitespace().collect();
    words1.sort();
    words2.sort();
    let sorted1 = words1.join(" ");
    let sorted2 = words2.join(" ");
    let sorted_score = similarity_score(&sorted1, &sorted2);
    if sorted_score >= threshold {
        return Some(sorted_score);
    }

    None
}

/// Screen a name against a list of entries (name + aliases). Returns match details.
pub fn screen_name_against_entries(
    name: &str,
    entries: &[(String, Vec<String>)], // (full_name, aliases)
    threshold: u8,
) -> Vec<(usize, String, u8)> {
    // Returns: Vec<(entry_index, matched_name, score)>
    let mut matches = Vec::new();

    // Short names (< 3 chars) require exact match
    let effective_threshold = if normalize_name(name).len() < 3 {
        100
    } else {
        threshold
    };

    for (idx, (full_name, aliases)) in entries.iter().enumerate() {
        // Check main name
        if let Some(score) = compare_names(name, full_name, effective_threshold) {
            matches.push((idx, full_name.clone(), score));
            continue;
        }

        // Check aliases
        for alias in aliases {
            if let Some(score) = compare_names(name, alias, effective_threshold) {
                matches.push((idx, alias.clone(), score));
                break;
            }
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Levenshtein distance ---

    #[test]
    fn test_levenshtein_identical() {
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
    }

    #[test]
    fn test_levenshtein_empty() {
        assert_eq!(levenshtein_distance("", "hello"), 5);
        assert_eq!(levenshtein_distance("hello", ""), 5);
        assert_eq!(levenshtein_distance("", ""), 0);
    }

    #[test]
    fn test_levenshtein_one_edit() {
        assert_eq!(levenshtein_distance("cat", "bat"), 1); // substitution
        assert_eq!(levenshtein_distance("cat", "cats"), 1); // insertion
        assert_eq!(levenshtein_distance("cats", "cat"), 1); // deletion
    }

    #[test]
    fn test_levenshtein_multiple_edits() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    }

    // --- Similarity score ---

    #[test]
    fn test_similarity_identical() {
        assert_eq!(similarity_score("jean alaoui", "jean alaoui"), 100);
    }

    #[test]
    fn test_similarity_one_typo() {
        // "jean alaoui" vs "jean alaouie" — 1 char diff over 12 chars
        let score = similarity_score("jean alaoui", "jean alaouie");
        assert!(score > 85, "Score was {score}");
    }

    #[test]
    fn test_similarity_empty() {
        assert_eq!(similarity_score("", ""), 100);
        assert_eq!(similarity_score("abc", ""), 0);
    }

    // --- Name normalization ---

    #[test]
    fn test_normalize_lowercase() {
        assert_eq!(normalize_name("JEAN ALAOUI"), "jean alaoui");
    }

    #[test]
    fn test_normalize_accents() {
        assert_eq!(normalize_name("José García"), "jose garcia");
        assert_eq!(normalize_name("François Müller"), "francois muller");
        assert_eq!(normalize_name("Ñoño"), "nono");
    }

    #[test]
    fn test_normalize_whitespace() {
        assert_eq!(normalize_name("  jean   alaoui  "), "jean alaoui");
    }

    #[test]
    fn test_normalize_german_ss() {
        assert_eq!(normalize_name("Straße"), "strasse");
    }

    // --- Compare names ---

    #[test]
    fn test_compare_exact_match() {
        assert_eq!(compare_names("Jean Alaoui", "jean alaoui", 80), Some(100));
    }

    #[test]
    fn test_compare_typo() {
        let result = compare_names("Jean Alaoui", "Jean Alaouie", 80);
        assert!(result.is_some());
        assert!(result.unwrap() > 80);
    }

    #[test]
    fn test_compare_case_insensitive() {
        assert_eq!(compare_names("JEAN ALAOUI", "jean alaoui", 80), Some(100));
    }

    #[test]
    fn test_compare_word_order() {
        let result = compare_names("Jean Alaoui", "Alaoui Jean", 80);
        assert!(result.is_some());
    }

    #[test]
    fn test_compare_no_match() {
        let result = compare_names("Jean Alaoui", "Mohammed Ben Ali", 80);
        assert!(result.is_none());
    }

    #[test]
    fn test_compare_short_name_requires_exact() {
        // "Al" vs "Ali" — short name, requires exact
        let result = compare_names("Al", "Ali", 80);
        assert!(result.is_none());
    }

    // --- Screen against entries ---

    #[test]
    fn test_screen_finds_match() {
        let entries = vec![
            (
                "Mohammed Ben Ali".to_string(),
                vec!["M. Ben Ali".to_string()],
            ),
            ("Jean Alaoui".to_string(), vec!["J. Alaoui".to_string()]),
        ];
        let matches = screen_name_against_entries("Jean Alaouie", &entries, 80);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].0, 1); // index of "Jean Alaoui"
        assert!(matches[0].2 > 80);
    }

    #[test]
    fn test_screen_finds_alias_match() {
        let entries = vec![(
            "Full Name Person".to_string(),
            vec!["Jean Alaoui".to_string()],
        )];
        let matches = screen_name_against_entries("Jean Alaouie", &entries, 80);
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_screen_no_match() {
        let entries = vec![("Mohammed Ben Ali".to_string(), vec![])];
        let matches = screen_name_against_entries("Jean Alaoui", &entries, 80);
        assert!(matches.is_empty());
    }

    #[test]
    fn test_screen_multiple_matches() {
        let entries = vec![
            ("Jean Alaoui".to_string(), vec![]),
            ("Jean Aloui".to_string(), vec![]),
            ("Pierre Dupont".to_string(), vec![]),
        ];
        let matches = screen_name_against_entries("Jean Alaoui", &entries, 80);
        assert!(matches.len() >= 1); // At least exact match
    }
}
