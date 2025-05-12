use crate::comp::item_list_view::{create_tokens, match_tokens, token_finder};

#[test]
pub fn test_create_tokens1() {
    let text = "a o";
    let out = create_tokens(text, false);
    assert_eq!(out.len(), 2);

    let text = "a b    c";
    let out = create_tokens(text, false);
    assert_eq!(out.len(), 3);

    let text = "a o a";
    let out = create_tokens(text, false);
    assert_eq!(out.len(), 2);

    let text = "a o A";
    let out = create_tokens(text, false);
    assert_eq!(out.len(), 3);

    let text = "mama anorta";
    let out = create_tokens(text, false);
    assert_eq!(out.len(), 6);
}

#[test]
pub fn test_token_finder1() {
    let item = "frederick has a kangaroo";
    let text = "f h k";
    let out = token_finder(item, text, false);
    assert_eq!(out.len(), 8);
    let reds = out.iter().filter(|&(_ch, r)| *r).count();
    assert_eq!(reds, 4);
}

#[test]
pub fn test_token_finder2() {
    let item = "git commit --amend";
    let text = "g cm a";
    let out = token_finder(item, text, false);
    assert_eq!(out.len(), 8);
    let reds = out.iter().filter(|&(_ch, r)| *r).count();
    assert_eq!(reds, 4);
}

#[test]
pub fn test_match_tokens1() {
    let item = "frederick has a kangaroo";
    let text = " ";
    let out = match_tokens(item, text, false);
    assert_eq!(out.len(), 1);
}
