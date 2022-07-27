use crate::models::Collidable;
use core::hash::Hash;
use itertools::iproduct;
use itertools::Either;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter;
use std::rc::Rc;

type CollisionSet<T> = HashSet<((usize, T), (usize, T))>;

fn find_pair_collisions<T>(vectors: impl Iterator<Item = Vec<T>>) -> CollisionSet<T>
where
    T: Collidable + Clone + Hash + Eq,
{
    let mut out = HashSet::new();
    for pair in vectors.enumerate().combinations(2) {
        for (com1, com2) in iproduct!(pair[0].1.iter(), pair[1].1.iter()) {
            if com1.collides(com2) {
                out.insert(((pair[0].0, com1.clone()), (pair[1].0, com2.clone())));
            }
        }
    }
    out
}

pub fn recursive_generate<'a, T: Collidable + Hash + Eq + Clone + 'a>(
    pair_collisions: Rc<CollisionSet<T>>,
    previously_chosen: Rc<Vec<Option<T>>>,
    vectors: Vec<Group<T>>,
) -> Box<dyn Iterator<Item = Vec<Option<T>>> + 'a> {
    if vectors.is_empty() {
        return Box::new(iter::once(Vec::clone(&previously_chosen)));
    }
    let to_choose = vectors[0].clone();
    let current_index = previously_chosen.len();

    let collides_with_previous = {
        let previously_chosen = previously_chosen.clone();
        let pair_collisions = pair_collisions.clone();
        move |val: T| {
            previously_chosen.iter().enumerate().any(|(i, previous)| {
                previous.clone().map_or(false, |previous| {
                    pair_collisions.contains(&((i, previous), (current_index, val.clone())))
                })
            })
        }
    };

    Box::new(
        to_choose
            .items
            .into_iter()
            .filter(move |val| !collides_with_previous(val.clone()))
            .map(Some)
            .chain(if to_choose.mandatory {
                Either::Left(iter::empty())
            } else {
                Either::Right(iter::once(None))
            })
            .flat_map({
                let rest = vectors[1..].to_vec();
                move |val| {
                    let mut updated_previously_chosen = (*previously_chosen).clone();
                    updated_previously_chosen.push(val);
                    recursive_generate(
                        pair_collisions.clone(),
                        Rc::new(updated_previously_chosen),
                        rest.clone(),
                    )
                }
            }),
    )
}

#[derive(Debug, Clone)]
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

pub fn generate<'a, T: Collidable + Hash + Eq + Clone + 'a>(
    mandatory: Vec<Vec<T>>,
    vectors: Vec<Vec<T>>,
) -> Box<dyn Iterator<Item = Vec<Option<T>>> + 'a> {
    let pair_collisions =
        find_pair_collisions(mandatory.iter().cloned().chain(vectors.iter().cloned()));

    recursive_generate(
        Rc::new(pair_collisions),
        Rc::new(vec![]),
        iter::empty()
            .chain(mandatory.into_iter().map(|items| Group::mandatory(items)))
            .chain(vectors.into_iter().map(|items| Group::optional(items)))
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Span;

    #[test]
    fn generate_test() {
        let sa = Span::new("00:00".parse().unwrap(), "01:00".parse().unwrap());
        let sb = Span::new("01:00".parse().unwrap(), "02:00".parse().unwrap());
        let sc = Span::new("02:00".parse().unwrap(), "03:00".parse().unwrap());
        assert_eq!(
            generate(vec![vec![sa, sc],], vec![vec![sa, sb, sc], vec![sa, sb,],])
                .collect::<Vec<Vec<_>>>(),
            vec![
                vec![Some(sa), Some(sb), None],
                vec![Some(sa), Some(sc), Some(sb)],
                vec![Some(sa), Some(sc), None],
                vec![Some(sa), None, Some(sb)],
                vec![Some(sa), None, None],
                vec![Some(sc), Some(sa), Some(sb)],
                vec![Some(sc), Some(sa), None],
                vec![Some(sc), Some(sb), Some(sa)],
                vec![Some(sc), Some(sb), None],
                vec![Some(sc), None, Some(sa)],
                vec![Some(sc), None, Some(sb)],
                vec![Some(sc), None, None],
            ]
        );
    }
}
