use gen_iter::gen_iter;
use itertools::{iproduct, Itertools, MultiProduct};
use permutator::{CartesianProduct, CartesianProductIterator};
use std::iter::{self, Map};

//fn generate<const N: usize>(counts: [u8; N]) -> impl Iterator<Item = [Option<u8>; N]> {
//let iters: [Vec<Option<u8>>; N] = counts.map(|count| iter::once(None).chain((0..count).map(|n| Some(n)))).into();

//CartesianProductIterator::new(iters).into_iter()
//}

//fn generate<const N: usize, T: Clone>(vectors: [Vec<T>; N]) -> impl Iterator<Item = > {
//vectors.iter().map(|v| iter::once(None).chain(v.iter().map(Option::Some))).multi_cartesian_product()
//}
//fn generate<const N: usize>(counts: [u8; N]) -> impl Iterator<Item = [Option<u8>; N]> {
//if N == 1 {
//return (0..counts[0]).map(|n| [Some(n)]).into_iter();
//}
//gen_iter!(move {
//match N {
//0 => {yield [None; N];},
//1 => {for i in 0..counts[0] {
//yield [Some(i); N];
//}
//},
//N => {
//for opt in generate(counts[..counts.len() - 1].try_into().unwrap()) {
//let mut with_end: [Option<u8>; N] = opt.clone();
//with_end[N - 1] = None;
//yield with_end;
//for i in 0..counts[counts.len() - 1] {
//with_end[N - 1] = Some(i);
//yield with_end;
//}
//}}
//};
//})
//}

fn generate<'a, T: Copy + 'a>(vectors: &'a Vec<Vec<T>>) -> impl Iterator<Item = Vec<Option<T>>> + 'a {
    vectors
        .iter()
        .map(|v| iter::once(None).chain(v.iter().map(Clone::clone).map(Option::Some)))
        .multi_cartesian_product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_test() {
        //let generate = |vectors: Vec<Vec<_>>| {
        //vectors
        //.iter()
        //.map(|v| iter::once(None).chain(v.iter().map(Option::Some)))
        //.multi_cartesian_product()
        //};

        assert_eq!(
            generate(&vec![
                (0..1u8).collect::<Vec<u8>>(),
                (0..2u8).collect::<Vec<u8>>()
            ])
            .map(|v| v.into_iter().collect_vec())
            .collect::<Vec<Vec<Option<u8>>>>(),
            vec![
                vec![None, None],
                vec![None, Some(0u8)],
                vec![None, Some(1u8)],
                vec![Some(0u8), None],
                vec![Some(0u8), Some(0u8)],
                vec![Some(0u8), Some(1u8)]
            ]
        );
    }
}
