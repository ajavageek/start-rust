#[cfg(test)]
use crate::model::Group;
use crate::solutions::i;

#[test]
fn should_return_none_if_groups_is_empty() {
    let groups = Vec::new();
    let result = i::find_largest_group(&groups);
    assert!(result.is_none());
}

#[test]
fn should_return_group_if_groups_has_only_one_element() {
    let group = Group {
        name: String::from("The Misfits of Science"),
        members: Vec::new(),
    };
    let groups = vec![group];
    let result = i::find_largest_group(&groups);
    assert!(result.is_some());
    assert_eq!(result, groups.first());
}
