use std::char;
use entities::{ENTITIES, Codepoints};


pub fn as_html(iter: &mut Iterator<Item = u32>) -> Option<String> {
    iter.next().map(|i| {
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
    })
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
    use std::iter::{empty, once};


    #[test]
    fn empty_iterator() {
        assert_eq!(as_html(&mut empty()), None);
    }


    #[test]
    fn test_as_html() {
        let expected1 = Some(r"&#x0000;".to_string());
        assert_eq!(as_html(&mut once(0)), expected1);

        let expected2 = Some(r"&#xFFFF;".to_string());
        assert_eq!(as_html(&mut once(0xFFFF)), expected2);

        let expected3 = Some(r"&#xbeef;".to_string());
        assert_ne!(as_html(&mut once(0xBEEF)), expected3);

        let expected4 = Some(r"&comma;".to_string());
        assert_eq!(as_html(&mut once(',' as u32)), expected4);

        let expected5 = Some(r"&gt;".to_string());
        assert_eq!(as_html(&mut once('>' as u32)), expected5);

        // assert_eq!(as_html(&mut once(0x10000)), ?);
    }


    #[test]
    fn loop_without_crashing() {
        let v = vec![0, 1, 2];
        let mut iter = v.into_iter();

        while let Some(_) = as_html(&mut iter) {}
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
