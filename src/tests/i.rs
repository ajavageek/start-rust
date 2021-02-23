#[cfg(test)]
use crate::model::Group;
use crate::solutions::i;

#[test]
fn should_return_none_if_groups_is_empty() {
    let groups = Vec::new();
    let result = i::find_largest_group(&groups);
    assert!(result.is_none());
}
