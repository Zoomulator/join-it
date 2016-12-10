



pub struct JoinIt<ItLeft, ItRight> where
    ItLeft: Iterator,
    ItRight: Iterator,
{
    left: ItLeft,
    right: ItRight
}


impl<ItLeft, ItRight, K, V0, V1> Iterator for JoinIt<ItLeft,ItRight> where
    ItLeft: Iterator<Item=(K,V0)>,
    ItRight: Iterator<Item=(K, V1)>,
{
    type Item = (K, V0, V1);

    fn next(&mut self) -> Option<Self::Item> {
        let l = self.left.next().unwrap();
        let r = self.right.next().unwrap();
        Some((l.0, l.1, r.1))
    }
}


trait Joinable
    where Self: Iterator + Sized
{
    fn join<I>(self, I) -> JoinIt<Self, I> where
        I: Iterator;
}


impl<I> Joinable for I where
    I: Iterator
{
    fn join<J>(self, iter: J)
        -> JoinIt<Self, J>
        where J: Iterator {
        JoinIt {
            left: self,
            right: iter
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use Joinable;

    #[test]
    fn it_works() {
        let v = vec!['a', 'b', 'c'];
        let it = v.iter().enumerate();

        let w = vec![66, 77, 88];
        let it2 =  w.iter().enumerate();

        let mut join_it = it.join(it2);
        let x = join_it.next();
        println!("{:?}",x);
    }

    #[test]
    fn it_doesnt_work() {
        let v: Vec<(u32,u32)> = vec![(0,11), (1,22), (2,33)];
        let mut it = v.iter();

        let w = vec![66, 77, 88];
        let it2 =  w.iter().enumerate();

        let mut join_it = it.join(it2);
        let x = join_it.next();
        println!("{:?}",x);
    }
}
