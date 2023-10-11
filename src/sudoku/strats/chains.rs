use std::collections::{HashMap, VecDeque};

use crate::{
    bitset::Set,
    sudoku::{Board, Candidate},
};

use super::{
    link::{Bilocal, Bivalue, LinkClass, LinkGraph, Strong, Weak, WeakUnit},
    Strategy, StrategyResult,
};

// =============================================================================

pub const X_CHAIN_SIMPLE: Strategy = Strategy {
    name: "X-Chain Simple",
    find: find_chain::<Bilocal, Bilocal>,
};

pub const X_CHAIN: Strategy = Strategy {
    name: "X-Chain",
    find: find_chain::<Bilocal, WeakUnit>,
};

pub const XY_CHAIN: Strategy = Strategy {
    name: "XY-Chain",
    find: find_chain::<Bivalue, WeakUnit>,
};

pub const AIC: Strategy = Strategy {
    name: "AIC",
    find: find_chain::<Strong, Weak>,
};

// =============================================================================

fn find_chain<S: LinkClass, W: LinkClass>(board: &Board) -> StrategyResult {
    let mut chains = Vec::new();

    let strong_links = LinkGraph::new::<S>(board);
    let weak_links = LinkGraph::new::<W>(board);

    let links = |parity| match parity {
        Parity::Even => &strong_links,
        Parity::Odd => &weak_links,
    };

    for root_cell in board.iter_unsolved() {
        for root_digit in board.get_notes(&root_cell).unwrap().iter() {
            let root = (root_cell, root_digit).into();

            let mut queue = VecDeque::<QueueItem>::new();
            let mut visited = Set::<Candidate>::new();
            let mut parents = HashMap::<Candidate, Candidate>::new();

            queue.push_back(QueueItem::new(root, Parity::Even));

            while let Some(u_item) = queue.pop_front() {
                let u = u_item.vertex;
                let u_parity = u_item.parity;

                if visited.contains(u) {
                    continue;
                }

                // check if chain is nontrivial and useful
                if u_parity == Parity::Odd {
                    // type 1: same digit on both ends
                    if u.digit() == root.digit() {
                        let digit = u.digit();

                        let common_neighbors = u.cell().neighbors() & root.cell().neighbors();

                        let eliminations = (common_neighbors & board.cells_with_note(digit))
                            .map(|cell| (cell, digit).into());

                        if !eliminations.is_empty() {
                            let chain = backtrack_chain(u, &parents);

                            let mut highlights = Set::new();
                            let mut highlights2 = Set::new();

                            for (i, &c) in chain.iter().enumerate() {
                                if i % 2 == 0 {
                                    highlights.insert(c);
                                } else {
                                    highlights2.insert(c);
                                }
                            }

                            chains.push((
                                chain,
                                StrategyResult {
                                    eliminations,
                                    highlights,
                                    highlights2,
                                    ..Default::default()
                                },
                            ));
                        }
                    }

                    // type 2: different digits which see each other
                }

                visited.insert(u_item.vertex);

                let neighbors = links(u_parity).neighbors(&u);

                for v in neighbors {
                    if visited.contains(v) {
                        continue;
                    }

                    parents.insert(v, u);
                    queue.push_back(QueueItem::new(v, u_parity.flip()));
                }
            }
        }
    }

    let minimum = chains.into_iter().min_by_key(|(chain, _)| chain.len());

    if let Some(minimum) = minimum {
        return minimum.1;
    }

    StrategyResult::default()
}

// -----------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Parity {
    Even,
    Odd,
}

impl Parity {
    fn flip(&self) -> Self {
        match self {
            Self::Even => Self::Odd,
            Self::Odd => Self::Even,
        }
    }
}

// -----------------------------------------------------------------------------

struct QueueItem {
    vertex: Candidate,
    parity: Parity,
}

impl QueueItem {
    fn new(vertex: Candidate, parity: Parity) -> Self {
        Self { vertex, parity }
    }
}

// -----------------------------------------------------------------------------

fn backtrack_chain(u: Candidate, parents: &HashMap<Candidate, Candidate>) -> Vec<Candidate> {
    let mut chain = vec![u];

    let mut v = u;
    while let Some(parent) = parents.get(&v) {
        chain.push(*parent);
        v = *parent;
    }

    chain.reverse();
    chain
}
