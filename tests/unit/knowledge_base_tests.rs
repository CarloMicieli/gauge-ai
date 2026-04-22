use std::collections::BTreeMap;

use gauge_ai::ai::knowledge_base::KnowledgeBase;

#[test]
fn matching_prototypes_is_case_insensitive_and_returns_canonical_keys() {
    let mut knowledge_base = KnowledgeBase::default();
    knowledge_base.prototypes = BTreeMap::from([(
        "SBB Ce 6/8 II".to_string(),
        vec!["krokodil".to_string(), "ce 6/8".to_string()],
    )]);

    let matches = knowledge_base.matching_prototypes("Looking for KROKODIL models");
    assert_eq!(matches, vec!["SBB Ce 6/8 II".to_string()]);
}

#[test]
fn matching_liveries_is_case_insensitive_and_returns_canonical_keys() {
    let mut knowledge_base = KnowledgeBase::default();
    knowledge_base.liveries = BTreeMap::from([(
        "Ocean Blue/Beige".to_string(),
        vec!["blue beige".to_string(), "ocean blue".to_string()],
    )]);

    let matches = knowledge_base.matching_liveries("Need an OCEAN BLUE locomotive");
    assert_eq!(matches, vec!["Ocean Blue/Beige".to_string()]);
}
