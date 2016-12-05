use std::char;
use entities::{ENTITIES, Codepoints};


pub fn as_html(i: u32) -> String {
    // Some characters have multiple entity options
    // e.g. &quot; and &QUOT;
    let entity_options = ENTITIES.iter()
        .filter_map(|e| {
            match e.codepoints {
                Codepoints::Single(cp) => if cp == i { Some(e.entity) } else { None },
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    if entity_options.is_empty() {
        format!("&#x{:01$X};", i, 4)
    } else {
        choose_nice_entity(entity_options).to_string()
    }
}


// "nice" means prefer lowercase and ends with a semicolon
fn choose_nice_entity(options: Vec<&str>) -> &str {
    assert!(!options.is_empty());

    let nicest_entity = options.iter()
        .find(|entity| ends_with_semicolon(entity) && is_all_lowercase(entity));

    if let Some(entity) = nicest_entity {
        return entity;
    }

    let less_nice_entity = options.iter()
        .find(|entity| ends_with_semicolon(entity) && !is_all_caps(entity));

    if let Some(entity) = less_nice_entity {
        return entity;
    }

    let ok_entity = options.iter().find(|entity| ends_with_semicolon(entity));

    if let Some(entity) = ok_entity {
        return entity;
    }

    let no_semi_entity = options.iter().find(|entity| is_all_lowercase(entity));

    if let Some(entity) = no_semi_entity {
        return entity;
    }

    let worst_entity = options.iter().find(|entity| !is_all_caps(entity));

    if let Some(entity) = worst_entity {
        return entity;
    }

    options[0]
}


fn ends_with_semicolon(entity: &str) -> bool {
    entity.ends_with(";")
}


fn is_all_caps(entity: &str) -> bool {
    !entity.chars().any(char::is_lowercase)
}


fn is_all_lowercase(entity: &str) -> bool {
    !entity.chars().any(char::is_uppercase)
}


#[cfg(test)]
mod tests {
    use super::{as_html, ends_with_semicolon, choose_nice_entity, is_all_caps, is_all_lowercase};


    #[test]
    fn test_as_html() {
        assert_eq!(as_html(0), r"&#x0000;");
        assert_eq!(as_html(0xFFFF), r"&#xFFFF;");
        assert_ne!(as_html(0xBEEF), r"&#xbeef;");
        assert_eq!(as_html(',' as u32), r"&comma;");
        assert_eq!(as_html('>' as u32), r"&gt;");
        // assert_eq!(as_html(0x10000), ?);
    }


    #[test]
    fn test_choose_nice_entity() {
        let e = ["&BAD", "&Bad", "&bad", "&BAD;", "&Bad;", "&bad;"];

        assert_eq!(choose_nice_entity(vec![e[0], e[1], e[2], e[3], e[4], e[5]]),
                   "&bad;");
        assert_eq!(choose_nice_entity(vec![e[0], e[1], e[2], e[3], e[4]]),
                   "&Bad;");
        assert_eq!(choose_nice_entity(vec![e[0], e[1], e[2], e[3]]), "&BAD;");
        assert_eq!(choose_nice_entity(vec![e[0], e[1], e[2]]), "&bad");
        assert_eq!(choose_nice_entity(vec![e[0], e[1]]), "&Bad");
        assert_eq!(choose_nice_entity(vec![e[0]]), "&BAD");
    }


    #[test]
    fn test_ends_with_semicolon() {
        assert!(ends_with_semicolon(";"));
        assert!(!ends_with_semicolon("beef"));
        assert!(!ends_with_semicolon(";Beef"));
        assert!(ends_with_semicolon("WITH SPACES;"));
        assert!(ends_with_semicolon("Double semicolon;;"));
    }


    #[test]
    fn test_is_all_caps() {
        assert!(is_all_caps("BEEF"));
        assert!(!is_all_caps("beef"));
        assert!(!is_all_caps("Beef"));
        assert!(is_all_caps("WITH SPACES"));
        assert!(is_all_caps("WITH_SYMBOLS&&;;"));
        assert!(!is_all_caps("&WithSymbols"));
    }

    #[test]
    fn test_is_all_lowercase() {
        assert!(!is_all_lowercase("BEEF"));
        assert!(is_all_lowercase("beef"));
        assert!(!is_all_lowercase("Beef"));
        assert!(is_all_lowercase("with spaces"));
        assert!(is_all_lowercase("with_symbols&&;;"));
        assert!(!is_all_lowercase("&WithSymbols"));
    }
}
