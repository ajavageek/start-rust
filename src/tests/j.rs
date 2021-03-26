#[cfg(test)]
use crate::model::Group;
use crate::solutions::j;
use crate::tests::samples;
use crate::tests::samples::{superman, wonder_woman, catman, batman, flash};

#[test]
fn should_return_none_if_supers_is_empty() {
    let supers = Vec::new();
    let result = j::find_supers_with_sidekicks(&supers);
    assert!(result.is_empty());
}

#[test]
fn should_return_none_if_supers_have_no_sidekicks() {
    let supers = vec![superman(), wonder_woman(), catman()];
    let result = j::find_supers_with_sidekicks(&supers);
    assert!(result.is_empty());
}

#[test]
fn should_return_all_supers_who_have_sidekicks() {
    let supers = vec![superman(), flash(), wonder_woman(), catman(), batman()];
    let result = j::find_supers_with_sidekicks(&supers);
    assert_eq!(result.len(), 2);
    assert_eq!(*result.first().unwrap(), &flash());
    assert_eq!(*result.last().unwrap(), &batman());
}
