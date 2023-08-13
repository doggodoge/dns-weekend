#[repr(u16)]
pub enum QClass {
    /// The internet (The only one you need!)
    IN = 1,
    /// The CSNET class (obsolete)
    CS = 2,
    /// The CHAOS class
    CH = 3,
    /// Hesiod [Dyer 87]
    HS = 4,
}
