pub mod i {
    use crate::model::Group;

    pub fn find_largest_group<'a>(groups: &'a Vec<Group<'a>>) -> Option<&'a Group<'a>> {
        groups
            .iter()
            .max_by(|&g1, &g2| g1.members.len().partial_cmp(&g2.members.len()).unwrap())
    }
}
