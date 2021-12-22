const UNCOUNTABLE: &[&'static str] = &[
    "equipment",
    "information",
    "rice",
    "money",
    "species",
    "series",
    "fish",
    "sheep",
    "jeans",
    "police",
    "milk",
    "salt",
    "time",
    "water",
    "paper",
    "food",
    "art",
    "cash",
    "music",
    "help",
    "luck",
    "oil",
    "progress",
    "rain",
    "research",
    "shopping",
    "software",
    "traffic",
];
const IRREGULAR: &[(&'static str, &'static str)] = &[
    ("man", "men"),
    ("child", "children"),
    ("sex", "sexes"),
    ("ombie", "ombies"),
    ("goose", "geese"),
    ("moose", "moose"),
    ("foot", "feet"),
    ("tooth", "teeth"),
    ("mouse", "mice"),
];
const WORDS: &[(&'static str, &'static str)] = &[("testis", "testes")];
const ENDINGS: &[(&'static str, &'static str)] = &[
    ("vertex", "vertices"),
    ("matrix", "matrices"),
    ("index", "indices"),
    //
    ("sis", "ses"),
    //
    ("alias", "aliases"),
    ("status", "statuses"),
    ("campus", "campuses"),
    ("bus", "buses"),
    //
    ("x", "xes"),
    ("ch", "ches"),
    ("ss", "sses"),
    ("sh", "shes"),
    //
    ("ay", "ays"),
    ("ey", "eys"),
    ("iy", "iys"),
    ("oy", "oys"),
    ("uy", "uys"),
    ("y", "ies"),
    //
    ("ffe", "ffes"),
    ("fe", "ves"),
    ("lf", "lves"),
];

fn word_match(w: &str, end: &str) -> bool {
    if w == end {
        return true;
    }
    if let Some(prefix) = w.strip_suffix(end) {
        match *prefix.as_bytes().last().unwrap() {
            b'-' | b' ' | b'.' | b',' | b'_' => true,
            _ => false,
        }
    } else {
        false
    }
}
fn concat(a: &str, b: &str) -> String {
    let mut out = String::with_capacity(a.len() + b.len());
    out.push_str(a);
    out.push_str(b);
    out
}

/// Convert a word from plural to singular
pub fn singularize(word: &str) -> String {
    for &w in UNCOUNTABLE.iter() {
        if word_match(word, w) {
            return word.into();
        }
    }
    for &(singular, plural) in IRREGULAR.iter() {
        if let Some(pref) = word.strip_suffix(plural) {
            return concat(pref, singular);
        }
    }
    for &(singular, plural) in WORDS.iter() {
        if word == plural {
            return singular.to_string();
        }
    }
    for &(singular, plural) in ENDINGS.iter().rev() {
        if let Some(pref) = word.strip_suffix(plural) {
            return concat(pref, singular);
        }
    }
    if let Some(word) = word.strip_suffix("s") {
        word.to_string()
    } else {
        word.to_string()
    }
}

/// Convert a word from singular to plural
pub fn pluralize(word: &str) -> String {
    for &w in UNCOUNTABLE.iter() {
        if word_match(word, w) {
            return word.into();
        }
    }
    for &(singular, plural) in IRREGULAR.iter() {
        if let Some(pref) = word.strip_suffix(singular) {
            return concat(pref, plural);
        }
    }
    for &(singular, plural) in WORDS.iter() {
        if word == singular {
            return plural.to_string();
        }
    }
    for &(singular, plural) in ENDINGS.iter() {
        if let Some(pref) = word.strip_suffix(singular) {
            return concat(pref, plural);
        }
    }
    format!("{}s", word)
}

#[cfg(test)]
fn two_way(singular: &str, plural: &str) {
    if &pluralize(singular) != plural {
        panic!(
            "Plural of {} should be {}, not {}",
            singular,
            plural,
            pluralize(singular)
        )
    }
    if &singularize(plural) != singular {
        panic!(
            "Singular of {} should be {}, not {}",
            plural,
            singular,
            singularize(plural)
        )
    }
}

#[test]
fn known() {
    two_way("equipment", "equipment");
    two_way("good_equipment", "good_equipment");
    two_way("rain", "rain");
    two_way("terrain", "terrains");
    two_way("dog", "dogs");
    two_way("woman", "women");

    two_way("calf", "calves");
    two_way("wolf", "wolves");
    two_way("werewolf", "werewolves");

    two_way("diagnosis", "diagnoses");

    two_way("glass", "glasses");

    two_way("alias", "aliases");
    two_way("status", "statuses");
    two_way("campus", "campuses");
    two_way("bus", "buses");

    two_way("vertex", "vertices");
    two_way("matrix", "matrices");
    two_way("index", "indices");
}
