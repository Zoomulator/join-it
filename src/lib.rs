//! # Joins
//! Sequences sorted by a common key can be joined (like a table inner join) into an iterator or via an
//! internal iteration.
//!
//! # Iterator
//! This module extends the `IntoIterator` trait with a [`join`] method, taking another
//! `IntoIterator` along with two key extracting closures and produces a new iterator, the output
//! of which will be a joined row for each `next()`.
//!
//! ```
//! use join_it::Joinable;
//!
//! let v = vec![(0,'a'),(2,'c')];
//! let w = vec![33,44,55,66];
//! let i = v.iter();
//! let j = w.iter().enumerate();
//!
//! for (&(k0,a),(k1,b)) in i.join(j, |&(k,_)| k, |(k,_)| k) {
//!     assert_eq!(k0, k1);
//!     println!("Join result: ({},{})", a, *b);
//! }
//! ```
//!
//! # Inner iteration
//! You're also provided with a function that runs an inner iteration. [`join_it`] takes two
//! `IntoIterator`s, their key extractor closures and finally a body closure to handle the joined
//! rows.
//!
//! # The algorithm
//! Given two sequences with the values sorted by a unique key, you're able to make a join between keys with
//! O(n) complexity. The keys of the current iterator values are compared. It's a match whenever
//! the two keys are equal, whereby a tuple of the two values are returned as a result of a joined row.
//! If the keys don't match, the lesser key's iterator will increment until they do or until one of
//! the iterators returns `None`.

use std::cmp::Ord;


pub struct JoinIt<I, J, KI, KJ>
{
    i: I,
    j: J,
    ki: KI,
    kj: KJ,
}


/// Maps f over the join between `i` and `j`, based on the key extractors `ki` and `kj`.
///
/// ```
/// use join_it::join_it;
/// let v = vec![33,44,55,66].into_iter().enumerate();
/// let w = vec![(0,'a'),(2,'c')];
/// join_it(v, w, |(k,_)| k, |(k,_)| k, |(k0,a),(k1,b)| {
///     assert_eq!(k0, k1);
///     println!("Join result: ({},{})", a, b);
/// });
/// ```
pub fn join_it<I,J,K,KI,KJ,F>( i: I, j: J, ki: KI, kj: KJ, mut f: F ) where
    I: IntoIterator,
    J: IntoIterator,
    I::Item: Copy,
    J::Item: Copy,
    KI: Fn(I::Item) -> K,
    KJ: Fn(J::Item) -> K,
    F: FnMut(I::Item, J::Item),
    K: Ord
{
    use std::cmp::Ordering::*;
    let mut i = i.into_iter();
    let mut j = j.into_iter();
    let mut row = (i.next(), j.next());

    while let (Some(v), Some(w)) = row {
        match Ord::cmp(&ki(v), &kj(w)) {
            Less => row = (i.next(), Some(w)),
            Greater => row = (Some(v), j.next()),
            Equal => {
                f(v, w);
                row = (i.next(), j.next());
            },
        }
    }
}


impl<I,J,KI,KJ,K> Iterator for JoinIt<I,J,KI,KJ> where
    I: Iterator,
    J: Iterator,
    I::Item: Copy,
    J::Item: Copy,
    KI: FnMut(I::Item) -> K,
    KJ: FnMut(J::Item) -> K,
    K: Ord
{
    type Item = (I::Item, J::Item);

    fn next(&mut self) -> Option<Self::Item> {
        use std::cmp::Ordering::*;

        let mut row = (self.i.next(), self.j.next());

        while let (Some(v), Some(w)) = row {
            match Ord::cmp(&(self.ki)(v), &(self.kj)(w)) {
                Less => row = (self.i.next(), Some(w)),
                Greater => row = (Some(v), self.j.next()),
                Equal => {
                    return Some((v, w));
                },
            }
        }

        None
    }
}



pub trait Joinable
    where Self: IntoIterator + Sized,
          Self::Item: Copy
{
    fn join<J,KI,KJ,K>(self, J, KI, KJ) -> JoinIt<Self::IntoIter,J::IntoIter,KI,KJ> where
        J: IntoIterator,
        J::Item: Copy,
        KI: FnMut(Self::Item) -> K,
        KJ: FnMut(J::Item) -> K;
}



impl<I> Joinable for I where
    I: IntoIterator,
    I::Item: Copy
{
    fn join<J,KI,KJ,K>(self, iter: J, ki: KI, kj: KJ) -> JoinIt<I::IntoIter,J::IntoIter,KI,KJ> where
        J: IntoIterator,
        J::Item: Copy,
        KI: FnMut(Self::Item) -> K,
        KJ: FnMut(J::Item) -> K,
    {
        JoinIt {
            i: self.into_iter(),
            j: iter.into_iter(),
            ki: ki,
            kj: kj
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal_iterator() {
        let v = vec![(0,'a'), (1,'b'), (2,'c')];
        let it = v.iter(); // Iterator returning &({int}, char).

        let w = vec![66, 77, 88];
        let it2 =  w.iter().enumerate(); // Iterator returning ({int}, &{int}).

        let mut r = vec![];
        join_it( it, it2, |&(x,_)| x, |(x,_)| x, |&(_,a), (_,b)| {
            r.push((a,*b));
        });

        assert_eq!( vec![('a',66), ('b',77), ('c',88)], r );
    }

    #[test]
    fn move_iterators() {
        let v = vec!['a', 'b', 'c'];
        let it = v.iter().enumerate();

        let w = vec![66, 77, 88];
        let it2 =  w.iter().enumerate();

        let join_it = it.join(it2, |(x,_)| x, |(x,_)| x)
            .map(|((_,a),(_,b))| (*a,*b));

        assert_eq!( vec![('a',66), ('b',77), ('c',88)], join_it.collect::<Vec<(char,u32)>>() );
    }

    #[test]
    fn referencing_iterators() {
        let v = vec![(0,'a'), (1,'b'), (2,'c')];
        let it = v.iter();

        let w = vec![(0,66), (1,77), (2,88)];
        let it2 =  w.iter();

        let join_it = it.join(it2, |&(x,_)| x, |&(x,_)| x)
            .map(|(&(_,a),&(_,b))| (a, b));

        assert_eq!( vec![('a',66), ('b',77), ('c',88)], join_it.collect::<Vec<(char,u32)>>() );
    }


    #[test]
    fn key_jumping() {
        let v = vec![(1,'b'), (2,'c'), (3,'d')];
        let it = v.iter();

        let w = vec![(0,66), (1,77), (3,99), (4,11)];
        let it2 =  w.iter();

        let join_it = it.join(it2, |&(x,_)| x, |&(x,_)| x)
            .map(|(&(_,a),&(_,b))| (a, b));

        assert_eq!( vec![('b',77), ('d',99)], join_it.collect::<Vec<(char,u32)>>() );
    }


    #[test]
    fn into_iter_consumption() {
        let v = vec![(1,'b'), (2,'c'), (3,'d')];

        let w = vec![(0,66), (1,77), (3,99), (4,11)];

        // Join v & w 'directly' via IntoIter trait.
        let join_it = v.join(w, |(x,_)| x, |(x,_)| x)
            .map(|((_,a),(_,b))| (a, b));

        assert_eq!( vec![('b',77), ('d',99)], join_it.collect::<Vec<(char,u32)>>() );
    }


    struct A {
        key: u32,
        c: char
    }

    struct B {
        key: u32,
        i: i32
    }

    #[test]
    fn keys_in_structs() {
        let v = vec![A{key:0, c:'a'}, A{key:1, c:'b'}, A{key:2,c:'c'}];
        let w = vec![B{key:1, i:10}, B{key:2,i:22}, B{key:3, i:33}];

        let join_it = v.iter().join(w.iter(), |&A{key,..}| key, |&B{key,..}| key)
            .map(|(&A{c,..}, &B{i,..})| (c,i));

        assert_eq!( vec![('b',10),('c',22)], join_it.collect::<Vec<(char,i32)>>() );
    }
}
