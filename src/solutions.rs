pub mod i {
    use crate::model::Group;

    pub fn find_largest_group<'a>(groups: &'a Vec<Group<'a>>) -> Option<&'a Group<'a>> {
        groups
            .iter()
            .max_by(|&g1, &g2| g1.members.len().partial_cmp(&g2.members.len()).unwrap())
    }
}

pub mod j {
    use crate::model::Super;

    pub fn find_supers_with_sidekicks<'a>(supers: &'a Vec<Super<'a>>) -> Vec<&Super<'a>> {
        supers
            .iter()
            .filter(|&s| s.sidekick.is_some())
            .collect()
    }
}
