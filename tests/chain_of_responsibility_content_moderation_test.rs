use design_patterns_rust::patterns::gof::behavioral::chain_of_responsibility::content_moderation::{
    ModerationChain, UserMessage,
};

#[test]
fn chain_allows_clean_content_after_all_moderation_filters_pass() {
    let chain = ModerationChain::default();
    let message = UserMessage::new("Rust makes service code easier to reason about");

    let result = chain.review(&message);

    assert_eq!(result, Ok("message approved".to_string()));
}

#[test]
fn chain_rejects_spam_before_running_later_filters() {
    let chain = ModerationChain::default();
    let message = UserMessage::new("buy now and click this spam offer");

    let result = chain.review(&message);

    assert_eq!(result, Err("message rejected as spam".to_string()));
}
