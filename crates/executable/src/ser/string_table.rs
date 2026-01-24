use core::{cmp::Ordering, ops::Deref};
use std::collections::{BTreeSet, btree_set};

/// An uncompiled string table.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StringTable {
    /// The underlying data.
    strings: BTreeSet<Entry>,

    /// The total length of all strings inserted thus far.
    total_len: usize,
}

impl StringTable {
    /// Creates an empty string table.
    pub const fn new() -> Self {
        Self {
            strings: BTreeSet::new(),
            total_len: 0,
        }
    }

    /// Adds a string to the string table, returning `true` iff the string was actually added.
    pub fn insert(&mut self, string: impl Into<Entry>) -> bool {
        let entry = string.into();
        let len_of_entry = entry.len();
        let inserted = self.strings.insert(entry);
        self.total_len += if inserted { len_of_entry } else { 0 };
        inserted
    }

    /// Iterates over all the entries in the string table in ascending order.
    ///
    /// For details on the exact ordering, see the documentation for [`Entry`].
    pub fn iter(&self) -> btree_set::Iter<'_, Entry> {
        self.strings.iter()
    }

    /// Compiles the string table into a blob of characters.
    ///
    /// This process attempts to reduce the size of the compiled table by abusing the presence of
    /// substrings. There is no guarantee that this will produce the optimal output, but it should
    /// be reasonably close most of the time.
    pub fn compile(self) -> String {
        let mut compiled = String::with_capacity(self.total_len);
        for entry in self.into_iter().rev() {
            let entry_str = entry.deref();
            if !compiled.contains(entry_str) {
                compiled.push_str(entry_str);
            }
        }

        compiled
    }
}

impl IntoIterator for StringTable {
    type IntoIter = <BTreeSet<Entry> as IntoIterator>::IntoIter;
    type Item = Entry;

    fn into_iter(self) -> Self::IntoIter {
        self.strings.into_iter()
    }
}

/// An entry in an [uncompiled string table](StringTable).
///
/// This is essentially just a thin wrapper around a [`Box<str>`] that exists for the sole purpose
/// of overriding the implementations of [`Ord`] and [`PartialOrd`]. The reason for this is that,
/// when [compiling](StringTable::compile) a string table, it's generally a good idea to iterate
/// over the strings in descending order of length (i.e., longer strings are looked at first). This
/// increases the chances of finding substrings and therefore reduces the size of the compiled
/// table. [`str`] implements [`Ord`] in a way that respects lexicographic ordering, which doesn't
/// really care much about the length of either string.
///
/// Entries are compared by first comparing their lengths. If they have different lengths, the
/// result of the aforementioned comparison is returned. Otherwise, the implementation defers to
/// [`str`]'s implementation of [`Ord`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry(Box<str>);

impl Deref for Entry {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl From<Box<str>> for Entry {
    fn from(string: Box<str>) -> Self {
        Self(string)
    }
}

impl From<&str> for Entry {
    fn from(string: &str) -> Self {
        string.to_string().into()
    }
}

impl From<String> for Entry {
    fn from(string: String) -> Self {
        Self(string.into_boxed_str())
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        let len_cmp = Ord::cmp(&self.len(), &other.len());
        if len_cmp == Ordering::Equal {
            self.0.cmp(&other.0)
        } else {
            len_cmp
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
