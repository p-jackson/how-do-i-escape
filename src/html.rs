use std::char;
use entities;
use entities::ENTITIES;


pub fn as_html(i: u32) -> String {
    // Some characters have multiple entity options
    // e.g. &quot; and &QUOT;
    let entity_options = ENTITIES.iter()
        .filter(|e| {
            match e.codepoints {
                entities::Codepoints::Single(cp) => cp == i,
                _ => false,
            }
        })
        .collect::<Vec<_>>();

    if entity_options.is_empty() {
        format!("&#x{:01$X};", i, 4)
    } else {
        let nicest = choose_nice_entity(entity_options);
        nicest.entity.to_string()
    }
}


// "nice" means prefer lowercase and ends with a semicolon
fn choose_nice_entity(options: Vec<&entities::Entity>) -> &entities::Entity {
    assert!(!options.is_empty());

    let nicest_entity = options.iter().find(|entity| has_semicolon(entity) && !is_all_caps(entity));

    if let Some(entity) = nicest_entity {
        return entity;
    }

    let less_nice_entity = options.iter().find(|entity| has_semicolon(entity));

    if let Some(entity) = less_nice_entity {
        return entity;
    }

    let ok_entity = options.iter().find(|entity| !is_all_caps(entity));

    if let Some(entity) = ok_entity {
        return entity;
    }

    options[0]
}


fn has_semicolon(entity: &entities::Entity) -> bool {
    entity.entity.ends_with(";")
}


fn is_all_caps(entity: &entities::Entity) -> bool {
    !entity.entity.chars().any(char::is_lowercase)
}


#[cfg(test)]
mod tests {
    use super::as_html;


    #[test]
    fn test_as_html() {
        assert_eq!(as_html(0), r"&#x0000;");
        assert_eq!(as_html(0xFFFF), r"&#xFFFF;");
        assert_ne!(as_html(0xBEEF), r"&#xbeef;");
        assert_eq!(as_html(',' as u32), r"&comma;");
        assert_eq!(as_html('>' as u32), r"&gt;");
        // assert_eq!(as_html(0x10000), ?);
    }
}
