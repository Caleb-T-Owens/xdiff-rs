#[expect(dead_code)]
type XDMerge = Vec<XDMergeEntry>;

#[expect(dead_code)]
struct XDMergeEntry {
    /// 0 = conflict,
    /// 1 = no conflict, take first,
    /// 2 = no conflict, take second.
    /// 3 = no conflict, take both.
    mode: u8,
    /// These point at the respective postimages.  E.g. <i1,chg1> is
    /// how side #1 wants to change the common ancestor; if there is no
    /// overlap, lines before i1 in the postimage of side #1 appear
    /// in the merge result as a region touched by neither side.
    i1: i64,
    /// See doc on [Self::i1].
    i2: i64,
    /// See doc on [Self::i1].
    chg1: i64,
    /// See doc on [Self::i1].
    chg2: i64,
    /// These point at the preimage; of course there is just one
    /// preimage, that is from the shared common ancestor.
    i0: i64,
    /// See doc on [Self::i1] and [Self::i0] for context.
    chg0: i64,
}
