use csv::StringRecord;

pub type Header = StringRecord;
pub type Record = Vec<StringRecord>;

#[derive(Clone, Debug)]
pub struct Data {
    pub header: Header,
    pub rows: Record,
    // TODO make this limit optioned by user with limitations.
    pub limit_to_show: usize
}

