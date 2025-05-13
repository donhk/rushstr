use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::{HItem, SearchOptions, prepare_string};

pub fn filter_items_monkey(items: &[HItem], options: &SearchOptions) -> Vec<HItem> {
    let matcher = SkimMatcherV2::default();
    let input = if options.is_case_insensitive() {
        prepare_string(&options.input).to_lowercase()
    } else {
        prepare_string(&options.input)
    };
    let reversed_input: String = input.chars().rev().collect();

    let mut matches: Vec<(HItem, i64)> = items
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
                (Some(f), Some(b)) => Some((item.clone(), f.max(b))),
                (Some(f), None) => Some((item.clone(), f)),
                (None, Some(b)) => Some((item.clone(), b)),
                (None, None) => None,
            }
        })
        .collect();
    matches.sort_by(|a, b| {
        b.0.hits()
            .cmp(&a.0.hits()) // order by hits first
            .then(b.1.cmp(&a.1)) // then order by score
    });
    matches.into_iter().map(|(item, _score)| item).collect()
}

pub fn filter_items_exact(items: &[HItem], options: &SearchOptions) -> Vec<HItem> {
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
            if haystack.contains(&input) {
                Some(item.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn filter_items_regex(items: &[HItem], options: &SearchOptions) -> Vec<HItem> {
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
        .filter_map(|item| {
            if re.is_match(&item.command()) {
                Some(item.clone())
            } else {
                None
            }
        })
        .collect()
}
