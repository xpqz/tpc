//! inner doc
/// outer doc
// line comment
/*
multi-line
block
*/
/* single-line block */
/// doc for main
fn main() {
    // inside
    /* inside block */
    let x = 1; // trailing line comment
    let y = 2; /* trailing block */
    /// stray doc
    let w = x + y;
}
