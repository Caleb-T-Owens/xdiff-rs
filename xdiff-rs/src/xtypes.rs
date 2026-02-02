use std::rc::Rc;

#[expect(dead_code)]
pub(crate) struct ChaStore {
    pub store: Vec<Vec<usize>>,
    pub ancur: usize,
    pub sncur: usize,
    pub scurr: usize,
}

#[expect(dead_code)]
pub(crate) struct XRecord {
    pub chars: Vec<u8>,
    // I think this is a hash of the string
    pub ha: i64,
}

#[expect(dead_code)]
pub(crate) struct XDFile {
    pub recs: Vec<XRecord>,
    pub dstart: i64,
    pub dend: i64,
    pub changed: Vec<bool>,
    pub rindex: Vec<i64>,
    pub nreff: i64,
}

#[expect(dead_code)]
pub(crate) struct XDEnv {
    pub xdf1: Rc<XDFile>,
    pub xdf2: Rc<XDFile>,
}
