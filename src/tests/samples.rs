use crate::model::{Group, Super};

pub(in crate::tests) fn batman<'a>() -> Super<'a> {
    Super {
        super_name: "Batman",
        real_name: "Bruce Wayne",
        power: 50,
    }
}

pub(in crate::tests) fn superman<'a>() -> Super<'a> {
    Super {
        super_name: "Superman",
        real_name: "Clark Kent",
        power: 100,
    }
}

pub(in crate::tests) fn kid_flash<'a>() -> Super<'a> {
    Super {
        super_name: "Kid Flash",
        real_name: "Wally West",
        power: 40,
    }
}

pub(in crate::tests) fn robin<'a>() -> Super<'a> {
    Super {
        super_name: "Robin",
        real_name: "Dick Grayson",
        power: 20,
    }
}

pub(in crate::tests) fn wonder_woman<'a>() -> Super<'a> {
    Super {
        super_name: "Wonder Woman",
        real_name: "Diana Prince",
        power: 95,
    }
}

pub(in crate::tests) fn flash<'a>() -> Super<'a> {
    Super {
        super_name: "Flash",
        real_name: "Barry Allen",
        power: 80,
    }
}

pub(in crate::tests) fn aquaman<'a>() -> Super<'a> {
    Super {
        super_name: "Aquaman",
        real_name: "Arthur Curry",
        power: 40,
    }
}

pub(in crate::tests) fn cyborg<'a>() -> Super<'a> {
    Super {
        super_name: "Cyborg",
        real_name: "Victor Stone",
        power: 60,
    }
}

pub(in crate::tests) fn justice_league<'a>() -> Group<'a> {
    Group {
        name: "Justice League",
        members: vec![
            superman(),
            wonder_woman(),
            batman(),
            flash(),
            aquaman(),
            cyborg(),
        ],
    }
}

pub(in crate::tests) fn catman<'a>() -> Super<'a> {
    Super {
        super_name: "Catman",
        real_name: "Thomas Blake",
        power: 60,
    }
}

pub(in crate::tests) fn deadshot<'a>() -> Super<'a> {
    Super {
        super_name: "Catman",
        real_name: "Floyd Lawton",
        power: 40,
    }
}

pub(in crate::tests) fn scandal<'a>() -> Super<'a> {
    Super {
        super_name: "Scandal",
        real_name: "Scandal Savage",
        power: 50,
    }
}

pub(in crate::tests) fn rag_doll<'a>() -> Super<'a> {
    Super {
        super_name: "Rag Doll",
        real_name: "Peter Merkel Jr",
        power: 60,
    }
}

pub(in crate::tests) fn sinister_six<'a>() -> Group<'a> {
    Group {
        name: "Sinister Six",
        members: vec![catman(), deadshot(), scandal(), rag_doll()],
    }
}
