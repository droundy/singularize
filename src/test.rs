use super::{pluralize, singularize};

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

    for pair in PAIRS.iter() {
        two_way(pair.0, pair.1);
    }
}

const PAIRS: &[(&'static str, &'static str)] = &[
    ("army", "armies"),
    ("ass", "asses"),
    ("baby", "babies"),
    ("bamboo", "bamboos"),
    ("bench", "benches"),
    ("bird", "birds"),
    ("boat", "boats"),
    ("bone", "bones"),
    ("box", "boxes"),
    ("boy", "boys"),
    ("buffalo", "buffaloes"),
    ("bus", "buses"),
    ("bush", "bushes"),
    ("caddy", "caddies"),
    ("calf", "calves"),
    ("car", "cars"),
    ("cat", "cats"),
    ("chair", "chairs"),
    ("chief", "chiefs"),
    ("child", "children"),
    ("city", "cities"),
    ("class", "classes"),
    ("fellow", "fellows"),
    ("cliff", "cliffs"),
    ("clutch", "clutches"),
    ("copy", "copies"),
    ("country", "countries"),
    ("cow", "cows"),
    ("cry", "cries"),
    ("cuckoo", "cuckoos"),
    ("cup", "cups"),
    ("day", "days"),
    ("deer", "deers"),
    ("dog", "dogs"),
    ("donkey", "donkeys"),
    ("dozen", "dozens"),
    ("duty", "duties"),
    ("essay", "essays"),
    ("family", "families"),
    ("father", "fathers"),
    ("fish", "fish"),
    ("fly", "flies"),
    ("foot", "feet"),
    ("fox", "foxes"),
    ("gas", "gases"),
    ("glass", "glasses"),
    ("grass", "grasses"),
    ("hair", "hairs"),
    ("half", "halves"),
    ("hand", "hands"),
    ("hero", "heroes"),
    ("hoof", "hoofs"),
    ("horse", "horses"),
    ("house", "houses"),
    ("inch", "inches"),
    ("jar", "jars"),
    ("key", "keys"),
    ("knife", "knives"),
    ("lady", "ladies"),
    ("lass", "lasses"),
    ("leaf", "leaves"),
    ("save", "saves"),
    ("leg", "legs"),
    ("life", "lives"),
    ("loaf", "loaves"),
    ("loof", "loofs"),
    ("love", "loves"),
    ("maidservant", "maidservants"),
    ("man", "men"),
    ("mango", "mangoes"),
    ("monkey", "monkeys"),
    ("mother", "mothers"),
    ("mouse", "mice"),
    ("news", "news"),
    ("ox", "oxen"),
    ("pencil", "pencils"),
    ("penny", "pennies"),
    ("pitch", "pitches"),
    ("poetry", "poetries"),
    ("potato", "potatoes"),
    ("proof", "proofs"),
    ("quiz", "quizzes"),
    ("radio", "radios"),
    ("ray", "rays"),
    ("river", "rivers"),
    ("scissor", "scissors"),
    ("self", "selves"),
    ("shop", "shops"),
    ("sister", "sisters"),
    ("sky", "skies"),
    ("spy", "spies"),
    ("stepson", "stepsons"),
    ("story", "stories"),
    ("table", "tables"),
    ("thief", "thieves"),
    ("tooth", "teeth"),
    ("toy", "toys"),
    ("trouser", "trousers"),
    ("uncle", "uncles"),
    ("watch", "watches"),
    ("wife", "wives"),
    ("wish", "wishes"),
    ("woman", "women"),
    //
    ("cat", "cats"),
    ("apple", "apples"),
    ("onion", "onions"),
    ("bird", "birds"),
    ("car", "cars"),
    ("house", "houses"),
    ("boy", "boys"),
    ("chair", "chairs"),
    ("shoe", "shoes"),
    //
    ("bench", "benches"),
    ("kiss", "kisses"),
    ("fox", "foxes"),
    ("bus", "buses"),
    ("witch", "witches"),
    ("box", "boxes"),
    ("bush", "bushes"),
    ("ash", "ashes"),
    ("lunch", "lunches"),
    ("class", "classes"),
    //
    ("baby", "babies"),
    ("family", "families"),
    ("party", "parties"),
    ("agency", "agencies"),
    ("city", "cities"),
    //
    // ("domino", "dominoes"),
    //
    ("wife", "wives"),
    ("half", "halves"),
    ("life", "lives"),
    ("thief", "thieves"),
    ("knife", "knives"),
    ("motive", "motives"),
];
