use crate::models::Collidable;
use core::hash::Hash;
use itertools::iproduct;
use itertools::Either;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter;
use std::rc::Rc;

type CollisionSet<K, T> = HashSet<((K, T), (K, T))>;

fn find_pair_collisions<K: Hash + Eq + Clone, T>(
    vectors: impl Iterator<Item = (K, Vec<T>)>,
) -> CollisionSet<K, T>
where
    T: Collidable + Clone + Hash + Eq,
{
    let mut out = HashSet::new();
    for pair in vectors.combinations(2) {
        let (key_a, a) = &pair[0];
        let (key_b, b) = &pair[1];
        for (com1, com2) in iproduct!(a.iter(), b.iter()) {
            if com1.collides(com2) {
                out.insert(((key_a.clone(), com1.clone()), (key_b.clone(), com2.clone())));
            }
        }
    }
    out
}

pub fn recursive_generate<'a, K: Hash + Eq + Clone + 'a, T: Collidable + Hash + Eq + Clone + 'a>(
    pair_collisions: Rc<CollisionSet<K, T>>,
    previously_chosen: Rc<Vec<(K, Option<T>)>>,
    vectors: Vec<(K, Group<T>)>,
) -> Box<dyn Iterator<Item = Vec<(K, Option<T>)>> + 'a> {
    if vectors.is_empty() {
        return Box::new(iter::once(Vec::clone(&previously_chosen)));
    }
    let (chosen_key, to_choose) = vectors[0].clone();

    let collides_with_previous = {
        let previously_chosen = previously_chosen.clone();
        let chosen_key = chosen_key.clone();
        let pair_collisions = pair_collisions.clone();
        move |val: T| {
            previously_chosen.iter().any(|(i, previous)| {
                previous.clone().map_or(false, |previous| {
                    pair_collisions.contains(&((i.clone(), previous), (chosen_key.clone(), val.clone())))
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
                    updated_previously_chosen.push((chosen_key.clone(), val));
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

pub fn generate<'a, K: Hash + Eq + Clone + 'a, T: Collidable + Hash + Eq + Clone + 'a>(
    mandatory: Vec<(K, Vec<T>)>,
    vectors: Vec<(K, Vec<T>)>,
    collision_exceptions: CollisionSet<K, T>,
) -> Box<dyn Iterator<Item = Vec<Option<T>>> + 'a> {
    let pair_collisions =
        find_pair_collisions(mandatory.iter().cloned().chain(vectors.iter().cloned()));
    let pair_collisions =
        HashSet::from_iter(pair_collisions.difference(&collision_exceptions).cloned());

    Box::new(
        recursive_generate(
            Rc::new(pair_collisions),
            Rc::new(vec![]),
            iter::empty()
                .chain(
                    mandatory
                        .into_iter()
                        .map(|(k, items)| (k, Group::mandatory(items))),
                )
                .chain(
                    vectors
                        .into_iter()
                        .map(|(k, items)| (k, Group::optional(items))),
                )
                .collect::<Vec<_>>(),
        )
        .map(|choice| choice.into_iter().map(|(_, o)| o).collect()),
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
            generate(
                vec![("0", vec![sa, sc]),],
                vec![("1", vec![sa, sb, sc]), ("2", vec![sa, sb,]),],
                HashSet::from([(("1", sb), ("2", sb))])
            )
            .collect::<Vec<Vec<_>>>(),
            vec![
                vec![Some(sa), Some(sb), Some(sb)],
                vec![Some(sa), Some(sb), None],
                vec![Some(sa), Some(sc), Some(sb)],
                vec![Some(sa), Some(sc), None],
                vec![Some(sa), None, Some(sb)],
                vec![Some(sa), None, None],
                vec![Some(sc), Some(sa), Some(sb)],
                vec![Some(sc), Some(sa), None],
                vec![Some(sc), Some(sb), Some(sa)],
                vec![Some(sc), Some(sb), Some(sb)],
                vec![Some(sc), Some(sb), None],
                vec![Some(sc), None, Some(sa)],
                vec![Some(sc), None, Some(sb)],
                vec![Some(sc), None, None],
            ]
        );
    }
}
