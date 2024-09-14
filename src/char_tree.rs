use std::collections::HashMap;



/// A class for storing character sequences that are easy to search.
///
/// For example, if we wanted to know if a given text string started with one of the following
/// character sequences:
///
/// abcd
/// abdd
/// abde
/// abde
/// abff
///
/// We could use the Python strings `find` method. However, this would require us to call find up
/// to 5 times. If the text to be searched does not start with 'a', then we would not need to look
/// any further.
///
/// So a CharTree is a way of storing character sequences that makes it very easy to determine if a
/// given sequence is *not* in the set.
pub struct CharTree {
    tree: HashMap<char, Option<Self>>,
}
