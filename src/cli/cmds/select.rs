use std::ffi::OsString;

#[derive(Debug, Clone)]
pub struct OhRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct RangeInput {
    pub range: OsString,
    pub holder: Vec<String>
}

impl RangeInput {
    pub fn new(selection_str: Option<OsString>) -> Self {
        let sel = selection_str.expect("Please provide the selection value");
        let holder = sel
            .clone()
            .into_string()
            .unwrap()
            .split(':')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
            RangeInput {
                range: sel,
                holder: holder
        }
    }


    //TODO validate the ranges that cames from the user function
    //TODO make -rc `range` `range | name`
    pub fn parse_range(self) -> Option<OhRange> {
        let [start, end] = self
            .holder
            .get(0..2)
            .expect("you have to specifiy the `start:end` range syntax") else {
            return None
        };
        // TODO consider usize::MAX and usize::MIN
        let begin: usize = start.parse()
            .expect("you should type a number between 0..any positve unsigned number");
        let finish: usize = end.parse()
            .expect("you should type a number between 0..any positve unsigned number");

        Some(OhRange {
            start: begin,
            end: finish
        })
    }
}

pub fn handle_select_opt(
    name_or_indx: Option<OsString>,
    index: Option<usize>,
    range: Option<OsString>
) {
}

