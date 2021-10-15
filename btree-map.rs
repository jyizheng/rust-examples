struct RString {
    s: String,
}
 
use std::cmp::Ord;
use std::cmp::Ordering;
 
impl PartialEq for RString {
    fn eq(&self, other:&Self) -> bool {
        if self.s == other.s {
            true
        } else {
            false
        }
    }
}
 
// this does not actually have any methods, it's just a flag on the type
impl Eq for RString { }
 
// make partial_cmp() just return result from cmp()
impl PartialOrd for RString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let me: &String = &(self.s);
        let them: &String = &(other.s);
 
        Some( me.cmp( them ) )
    }
}
 
impl Ord for RString {
    fn cmp(&self, other:&Self) -> Ordering {
        let me: &String = &(self.s);
        let them: &String = &(other.s);
 
        if me > them {
            Ordering::Less
        } else if me < them {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
 
use std::env;
use std::collections::btree_map::BTreeMap;
fn main() {
    // collect environment variable keys into a vector we can sort
    let mut sortedmap : BTreeMap<Box<RString>,Box<String>> = BTreeMap::new();
 
    for (key, value) in env::vars() {
        sortedmap.insert(
            Box::<RString>::new( RString { s: key } ),
            Box::<String>::new( value )
        );
    }
 
    for (key, value) in sortedmap {
        println!( "{} => {}", (*key).s, *value );
    }
}
