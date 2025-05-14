use std::rc::Rc;

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::{HItem, SearchOptions, prepare_string};

pub fn filter_items_monkey<'a>(
    items: &'a [Rc<HItem>],
    options: &SearchOptions,
    matcher: &SkimMatcherV2,
) -> Vec<&'a Rc<HItem>> {
    let input = if options.is_case_insensitive() {
        prepare_string(&options.input).to_lowercase()
    } else {
        prepare_string(&options.input)
    };
    let reversed_input: String = input.chars().rev().collect();

    let mut matches: Vec<(&'a Rc<HItem>, i64)> = items
        .iter()
        .filter_map(|item| {
            let target = if options.is_case_insensitive() {
                item.command().to_lowercase()
            } else {
                item.command()
            };

            let forward_score = matcher.fuzzy_match(&target, &input);
            let backward_score = matcher.fuzzy_match(&target, &reversed_input);

            match (forward_score, backward_score) {
                (Some(f), Some(b)) => Some((item, f.max(b))),
                (Some(f), None) => Some((item, f)),
                (None, Some(b)) => Some((item, b)),
                (None, None) => None,
            }
        })
        .collect();

    matches.sort_by(|a, b| b.0.hits().cmp(&a.0.hits()).then(b.1.cmp(&a.1)));

    matches.into_iter().map(|(item, _score)| item).collect()
}

pub fn filter_items_exact<'a>(items: &'a [Rc<HItem>], options: &SearchOptions) -> Vec<&'a Rc<HItem>> {
    let input = if options.is_case_insensitive() {
        options.input.to_lowercase()
    } else {
        options.input.to_string()
    };

    items
        .iter()
        .filter_map(|item| {
            let haystack = if options.is_case_insensitive() {
                item.command().to_lowercase()
            } else {
                item.command()
            };
            if haystack.contains(&input) { Some(item) } else { None }
        })
        .collect()
}

pub fn filter_items_regex<'a>(items: &'a [Rc<HItem>], options: &SearchOptions) -> Vec<&'a Rc<HItem>> {
    let pattern = if options.is_case_insensitive() {
        format!("(?i){}", options.input)
    } else {
        options.input.clone()
    };

    let re = match regex::Regex::new(&pattern) {
        Ok(re) => re,
        Err(_) => return vec![], // return empty if the regex is invalid
    };

    items
        .iter()
        .filter_map(|item| if re.is_match(&item.command()) { Some(item) } else { None })
        .collect()
}
