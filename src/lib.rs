use std::borrow::Borrow;



pub struct JoinIt<I, J, KI, KJ>
{
    left: I,
    right: J,
    key_l: KI,
    key_r: KJ,
}



impl<I,J,KI,KJ,K> Iterator for JoinIt<I,J,KI,KJ> where
    I: Iterator,
    J: Iterator,
    KI: FnMut(I::Item) -> K,
    KJ: FnMut(J::Item) -> K
{
    type Item = (K,K);

    fn next(&mut self) -> Option<Self::Item> {
        Some(
            ((self.key_l)(self.left.next().unwrap()),
             (self.key_r)(self.right.next().unwrap()))
        )
    }
}

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
    fn join<J,KI,KJ>(self, J, KI, KJ) -> JoinIt<Self,J,KI,KJ> where
        J: Iterator;
}


impl<I> Joinable for I where
    I: Iterator
{
    fn join<J,KI,KJ>(self, iter: J, key_l: KI, key_r: KJ)
        -> JoinIt<I,J,KI,KJ> where
        J: Iterator {
        JoinIt {
            left: self,
            right: iter,
            key_l: key_l,
            key_r: key_r
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use Joinable;

    #[test]
    fn move_iterators() {
        let v = vec!['a', 'b', 'c'];
        let it = v.iter().enumerate();

        let w = vec![66, 77, 88];
        let it2 =  w.iter().enumerate();

        let mut join_it = it.join(it2, |(x,_)| x, |(x,_)| x);
        let x = join_it.next();
        println!("{:?}",x);
    }

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
}
