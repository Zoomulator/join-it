use std::borrow::Borrow;
use std::cmp::Ord;



pub struct JoinIt<I, J, KI, KJ>
{
    i: I,
    j: J,
    ki: KI,
    kj: KJ,
}

pub fn join_it<I,J,K,KI,KJ,F>( i: I, j: J, ki: KI, kj: KJ, f: F ) where
    I: IntoIterator,
    J: IntoIterator,
    KI: Fn(&I::Item) -> K,
    KJ: Fn(&J::Item) -> K,
    F: Fn(&I::Item, &J::Item),
    K: Ord
{
    use std::cmp::Ordering::*;
    let mut i = i.into_iter();
    let mut j = j.into_iter();
    let mut row = (i.next(), j.next());

    while let (Some(v), Some(w)) = row {
        match Ord::cmp(&ki(&v), &kj(&w)) {
            Less => row = (i.next(), Some(w)),
            Greater => row = (Some(v), j.next()),
            Equal => {
                f(&v, &w);
                row = (i.next(), j.next());
            },
        }
    }
}


impl<'a,I,J,KI,KJ,K> Iterator for JoinIt<I,J,KI,KJ> where
    I: Iterator,
    J: Iterator,
    I::Item: Clone,
    J::Item: Clone,
    KI: FnMut(&I::Item) -> K,
    KJ: FnMut(&J::Item) -> K,
    K: Ord
{
    type Item = (I::Item, J::Item);

    fn next(&mut self) -> Option<Self::Item> {
        use std::cmp::Ordering::*;

        let mut row = (self.i.next(), self.j.next());

        while let (Some(v), Some(w)) = row {
            match Ord::cmp(&(self.ki)(&v), &(self.kj)(&w)) {
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


/*
impl<I,J,KI,KJ,K> Iterator for JoinIt<I,J,KI,KJ> where
    I: Iterator,
    J: Iterator,
    I::Item: Clone,
    J::Item: Clone,
    KI: FnMut(&I::Item) -> K,
    KJ: FnMut(&J::Item) -> K,
    K: Ord
{
    type Item = (I::Item,J::Item);

    fn next(&mut self) -> Option<Self::Item> {
        use std::cmp::Ordering::*;
        let mut l = self.left.next();
        let mut r = self.right.next();

        let rec = |it_l, it_r, key_l, key_r| {
            let (left,right) = match (it_l.next(), it_r.next()) {
                (&Some(ref left), &Some(ref right)) => (left, right),
                _ => return None;
            }

            match Ord::cmp(&(key_l)(left), &(key_r)(right)) {
                Less    => { l = self.left.next(); continue; },
                Greater => { r = self.right.next(); continue; },
                Equal   => { return Some((left.clone(),right.clone())); }
            }
        }

        while let Some((left,right)) = match (&l,&r) {
                (&Some(ref left), &Some(ref right)) => Some((left, right)),
                _ => None
            } {
        }

        None
    }
}
*/

/*
impl<'a, 'b, ItLeft, ItRight, K, V0, V1> Iterator for JoinIt<ItLeft,ItRight> where
    K: 'a + 'b,
    V0: 'a,
    V1: 'b,
    ItLeft: Iterator<Item=&'a (K, V0)>,
    ItRight: Iterator<Item=&'b (K, V1)>,
{
    type Item = (&'a K, &'a V0, &'b V1);

    next_join!();
}

impl<ItLeft, ItRight, K, V0, V1> Iterator for JoinIt<ItLeft,ItRight> where
    ItLeft: Iterator<Item=(K, V0)>,
    ItRight: Iterator<Item=(K, V1)>,
{
    type Item = (K, V0, V1);

    next_join!();
}
*/




trait Joinable
    where Self: Iterator + Sized
{
    fn join<J,KI,KJ,K>(self, J, KI, KJ) -> JoinIt<Self,J,KI,KJ> where
        J: Iterator,
        KI: FnMut(&Self::Item) -> K,
        KJ: FnMut(&J::Item) -> K;
}


impl<I> Joinable for I where
    I: Iterator
{
    fn join<J,KI,KJ,K>(self, iter: J, ki: KI, kj: KJ) -> JoinIt<I,J,KI,KJ> where
        J: Iterator,
        KI: FnMut(&Self::Item) -> K,
        KJ: FnMut(&J::Item) -> K,
    {
        JoinIt {
            i: self,
            j: iter,
            ki: ki,
            kj: kj
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use Joinable;
    #[test]
    fn internal_iterator() {
        let v = vec!['a', 'b', 'c'];
        let it = v.iter().enumerate();

        let w = vec![66, 77, 88];
        let it2 =  w.iter().enumerate();

        join_it( it, it2, |&(x,_)| x, |&(x,_)| x, |&(_,a), &(_,b)| {
            println!("({:?}, {:?})", a, b);
        });
    }

    #[test]
    fn move_iterators() {
        let v = vec!['a', 'b', 'c'];
        let it = v.iter().enumerate();

        let w = vec![66, 77, 88];
        let it2 =  w.iter().enumerate();

        let mut join_it = it.join(it2, |&(x,_)| x, |&(x,_)| x);

        for x in join_it {
            println!("{:?}",x);
        }
    }

    /*
    #[test]
    fn referencing_iterators() {
        let v: Vec<(u32,u32)> = vec![(0,11), (1,22), (2,33)];
        let mut it = v.iter();

        let w: Vec<(u32,u32)> = vec![(0,11), (1,22), (2,33)];
        let it2 =  w.iter();

        let mut join_it = it.join(it2, |&(x,_)| x, |&(x,_)| x);
        let x = join_it.next();
        println!("{:?}",x);
    }
    */
}
