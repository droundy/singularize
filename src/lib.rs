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
    "news",
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
    ("thief", "thieves"),
    //
    ("alias", "aliases"),
    ("status", "statuses"),
    ("campus", "campuses"),
    ("bus", "buses"),
    ("af", "aves"),
    ("ife", "ives"),
    //
    ("osis", "oses"),
    ("psis", "pses"),
];
const WORDS: &[(&'static str, &'static str)] = &[("testis", "testes"), ("ox", "oxen")];
const ENDINGS: &[(&'static str, &'static str)] = &[
    ("vertex", "vertices"),
    ("matrix", "matrices"),
    ("index", "indices"),
    ("buffalo", "buffaloes"),
    ("quiz", "quizzes"),
    ("potato", "potatoes"),
    ("hero", "heroes"),
    ("ango", "angoes"),
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
    // ("fe", "ves"),
    ("lf", "lves"),
    //
    ("s", "ses"),
    //
    ("rse", "rses"),
    ("use", "uses"),
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
mod test;
