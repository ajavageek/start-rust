pub mod i {
    use crate::model::Group;

    pub fn find_largest_group(groups: &Vec<Group>) -> Option<&Group> {
        groups
            .iter()
            .max_by(|&g1, &g2| g1.members.len().partial_cmp(&g2.members.len()).unwrap())
    }
}
