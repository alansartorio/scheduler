use itertools::Either;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter;
use std::rc::Rc;

use crate::models::collidable::Collidable;
use core::hash::Hash;

fn find_pair_collisions<'a, T>(
    vectors: &'a Vec<Group<T>>,
) -> HashSet<((usize, &'a T), (usize, &'a T))>
where
    T: Collidable + Hash + Eq,
{
    let mut out = HashSet::new();
    for pair in vectors.iter().enumerate().combinations(2) {
        for (com1, com2) in pair[0]
            .1
            .items
            .iter()
            .cartesian_product(pair[1].1.items.iter())
        {
            if com1.collides(&com2) {
                out.insert(((pair[0].0, com1), (pair[1].0, com2)));
            }
        }
    }
    out
}

pub fn recursive_generate<'a, T: Collidable + Hash + Eq + Clone>(
    pair_collisions: Rc<HashSet<((usize, &'a T), (usize, &'a T))>>,
    previously_chosen: Rc<Vec<Option<&'a T>>>,
    vectors: &'a [Group<T>],
) -> Box<dyn Iterator<Item = Vec<Option<&'a T>>> + 'a> {
    if vectors.len() == 0 {
        return Box::new(iter::once((*previously_chosen).clone()));
    }
    let to_choose = &vectors[0];
    let current_index = previously_chosen.len();

    let collides_with_previous = {
        let previously_chosen = previously_chosen.clone();
        let pair_collisions = pair_collisions.clone();
        move |val: &T| {
            previously_chosen.iter().enumerate().any(|(i, previous)| {
                previous
                    .map(|previous| {
                        pair_collisions.contains(&((i, previous), (current_index, val)))
                    })
                    .unwrap_or(false)
            })
        }
    };

    Box::new(
        to_choose
            .items
            .iter()
            .filter(move |val| !collides_with_previous(val))
            .map(Some)
            .chain(if to_choose.mandatory {
                Either::Left(iter::empty())
            } else {
                Either::Right(iter::once(None))
            })
            .flat_map(move |val| {
                let mut updated_previously_chosen = (*previously_chosen).clone();
                updated_previously_chosen.push(val);
                recursive_generate(
                    pair_collisions.clone(),
                    Rc::new(updated_previously_chosen),
                    &vectors[1..],
                )
            }),
    )
}

#[derive(Debug)]
pub struct Group<T> {
    pub items: Vec<T>,
    pub mandatory: bool,
}

impl<T> Group<T> {
    pub fn mandatory(items: Vec<T>) -> Self {
        Self {
            items,
            mandatory: true,
        }
    }
    pub fn optional(items: Vec<T>) -> Self {
        Self {
            items,
            mandatory: false,
        }
    }
}

pub fn generate<'a, T: Collidable + Hash + Eq + Clone>(
    vectors: &'a Vec<Group<T>>,
) -> impl Iterator<Item = Vec<Option<&'a T>>> + 'a {
    let pair_collisions = find_pair_collisions(vectors);

    recursive_generate(Rc::new(pair_collisions), Rc::new(vec![]), vectors)
}

#[cfg(test)]
mod tests {
    use crate::models::span::Span;

    use super::*;

    #[test]
    fn generate_test() {
        let sa = Span::new("00:00".parse().unwrap(), "01:00".parse().unwrap());
        let sb = Span::new("01:00".parse().unwrap(), "02:00".parse().unwrap());
        let sc = Span::new("02:00".parse().unwrap(), "03:00".parse().unwrap());
        assert_eq!(
            generate(&vec![
                Group::optional(vec![sa, sb, sc]),
                Group::mandatory(vec![sa, sc]),
                Group::optional(vec![sa, sb,]),
            ])
            .map(|v| v.into_iter().map(|x| x.map(Clone::clone)))
            .map(|v| v.collect_vec())
            .collect::<Vec<Vec<_>>>(),
            vec![
                vec![Some(sa), Some(sc), Some(sb)],
                vec![Some(sa), Some(sc), None],
                vec![Some(sb), Some(sa), None],
                vec![Some(sb), Some(sc), Some(sa)],
                vec![Some(sb), Some(sc), None],
                vec![Some(sc), Some(sa), Some(sb)],
                vec![Some(sc), Some(sa), None],
                vec![None, Some(sa), Some(sb)],
                vec![None, Some(sa), None],
                vec![None, Some(sc), Some(sa)],
                vec![None, Some(sc), Some(sb)],
                vec![None, Some(sc), None],
            ]
        );
    }
}
