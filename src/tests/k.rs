#[cfg(test)]
use crate::model::Group;
use crate::solutions::k;
use crate::tests::samples;
use crate::tests::samples::{superman, wonder_woman, catman, batman, flash, deadshot, kid_flash, robin};
use crate::model::Alignment::{Good, Evil};

#[test]
fn should_return_map_with_empty_vec_if_supers_is_empty() {
    let supers = Vec::new();
    let result = k::group_sidekicks_by_alignment(&supers);
    assert_eq!(result.len(), 2);
    assert!(result.get(&Evil).unwrap().is_empty());
    assert!(result.get(&Good).unwrap().is_empty());
}

#[test]
fn should_return_map_with_empty_vec_if_supers_have_no_sidekicks() {
    let supers = vec![superman(), wonder_woman(), catman()];
    let result = k::group_sidekicks_by_alignment(&supers);
    assert_eq!(result.len(), 2);
    assert!(result.get(&Evil).unwrap().is_empty());
    assert!(result.get(&Good).unwrap().is_empty());
}

#[test]
fn should_return_all_supers_who_have_sidekicks() {
    let supers = vec![superman(), flash(), wonder_woman(), catman(), batman(), deadshot()];
    let result = k::group_sidekicks_by_alignment(&supers);
    assert_eq!(result.len(), 2);
    assert!(result.get(&Evil).unwrap().is_empty());
    let good_sidekicks = result.get(&Good).unwrap();
    assert_ne!(good_sidekicks.is_empty(), true);
    assert_eq!(good_sidekicks.len(), 2);
    assert_eq!(*good_sidekicks.first().unwrap(), &kid_flash());
    assert_eq!(*good_sidekicks.last().unwrap(), &robin());
}
