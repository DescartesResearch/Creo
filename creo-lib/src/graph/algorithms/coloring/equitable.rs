use std::collections::{HashMap, HashSet};

use indexmap::{IndexMap, IndexSet};

use super::{ColorIndex, Error, Result};
use crate::graph::{DiGraph, DiGraphBuilder, Edge, NodeIndex, NodeView, Successors};

pub fn equitable_coloring(graph: &DiGraph, color_count: usize) -> Result<Vec<ColorIndex>> {
    let maximum_degree = graph.maximum_degree();
    if maximum_degree >= color_count {
        return Err(Error::NotEnoughColors {
            given: color_count,
            expected: maximum_degree + 1,
        });
    }
    if !graph.is_acyclic() {
        return Err(Error::CyclicGraph);
    }

    let padded_graph = PaddedGraph::new(graph, color_count);

    let mut edge_map = initialize_edge_map(&padded_graph);

    let mut coloring = initialize_arbitrary_equitable_coloring(&padded_graph, color_count);

    let mut color_map = initialize_color_map(&coloring, color_count);

    let mut neighbourhood_map = initialize_neighbourhood_map(&edge_map, &color_map);

    let mut witness_map = initialize_witness_map(&color_map, &neighbourhood_map);

    let mut edges_seen = HashSet::<Edge>::new();

    for u in padded_graph.iter_nodes() {
        let u_color = coloring[u.0];

        let mut sorted_neighbors: Vec<NodeIndex> = padded_graph.successors(u).collect();
        sorted_neighbors.sort_by(|a, b| a.0.cmp(&b.0));

        for v in sorted_neighbors {
            if edges_seen.contains(&(v, u).into()) {
                continue;
            }

            edges_seen.insert((u, v).into());

            edge_map[u.0].insert(v);
            edge_map[v.0].insert(u);

            let v_color = coloring[v.0];

            let key = (u, v_color);
            let color_count = neighbourhood_map
                .get_mut(&key)
                .expect("invalid key into neighbourhood map");
            *color_count += 1;

            if v_color != u_color && *color_count == 1 {
                let key = (u_color, v_color);
                *witness_map
                    .get_mut(&key)
                    .expect("invalid key into witness map") -= 1;
            }

            let key = (v, u_color);
            let color_count = neighbourhood_map
                .get_mut(&key)
                .expect("invalid key into neighbourhood map");
            *color_count += 1;

            if v_color != u_color && *color_count == 1 {
                let key = (v_color, u_color);
                *witness_map
                    .get_mut(&key)
                    .expect("invalid key into witness map") -= 1;
            }
        }

        let key = (u, u_color);
        let u_color_count = neighbourhood_map
            .get(&key)
            .expect("invalid key into neighbourhood map");
        if *u_color_count != 0 {
            let unused_color = *color_map
                .keys()
                .find(|color| {
                    *neighbourhood_map
                        .get(&(u, **color))
                        .expect("invalid key into neighbourhood map")
                        == 0
                })
                .expect("shoudl find an unused color");

            change_color(
                u,
                u_color,
                unused_color,
                &mut coloring,
                &mut color_map,
                &mut neighbourhood_map,
                &mut witness_map,
                &edge_map,
            );

            procedure_p(
                u_color,
                unused_color,
                &mut neighbourhood_map,
                &mut witness_map,
                &mut coloring,
                &mut color_map,
                &edge_map,
                &mut HashSet::new(),
            )
        }
    }

    coloring.truncate(graph.node_count());
    Ok(coloring)
}

pub fn is_coloring<T>(edges: T, coloring: &[ColorIndex]) -> Result<()>
where
    T: IntoIterator<Item = Edge>,
{
    edges.into_iter().try_for_each(|edge| {
        if coloring[edge.source.0] == coloring[edge.target.0] {
            return Err(Error::InvalidColoring {
                edge,
                color: coloring[edge.source.0],
            });
        }
        Ok(())
    })
}

pub fn is_equitable<T>(edges: T, coloring: &[ColorIndex], color_count: Option<usize>) -> bool
where
    T: IntoIterator<Item = Edge>,
{
    if is_coloring(edges, coloring).is_err() {
        return false;
    }

    let mut color_set_size = HashMap::<ColorIndex, u32>::new();
    for color in coloring {
        *color_set_size.entry(*color).or_default() += 1;
    }

    if let Some(cc) = color_count {
        for color in 0..cc {
            color_set_size.entry(color.into()).or_insert(0);
        }
    }

    // If there are more than 2 distinct values, the coloring cannot be equitable
    let all_set_sizes = HashSet::<&u32>::from_iter(color_set_size.values());

    match all_set_sizes.len() {
        0 => color_count.is_none(),
        2 => {
            let mut it = all_set_sizes.iter();
            let (a, b) = (it.next().unwrap(), it.next().unwrap());
            a.abs_diff(**b) <= 1
        }
        size => size == 1,
    }
}

struct PaddedGraph<'graph> {
    graph: &'graph DiGraph,
    k_p: DiGraph,
}

impl<'graph> PaddedGraph<'graph> {
    /// Add a disconnected, complete clique `K_p` such that the node count of the graph becomes a
    /// multiple of [`color_count`]
    pub fn new(graph: &'graph DiGraph, cc: usize) -> Self {
        let nc = graph.node_count();

        let nc_per_c = nc / cc;

        let k_p = if nc != nc_per_c * cc {
            let p_nc = cc - nc % cc;

            let mut builder = DiGraphBuilder::with_node_count(p_nc).add_nodes(p_nc);
            for u in 0..p_nc {
                for v in (u + 1)..p_nc {
                    builder = builder.add_edge((u, v).into());
                }
            }
            builder.build()
        } else {
            DiGraphBuilder::new().build()
        };

        Self { graph, k_p }
    }

    pub fn iter_nodes(&'graph self) -> NodeView {
        NodeView::from(self.graph.node_count() + self.k_p.node_count())
    }

    pub fn successors(&self, source: NodeIndex) -> Successors {
        if source.0 < self.graph.node_count() {
            return self.graph.successors(source);
        }

        return self
            .k_p
            .successors((source.0 - self.graph.node_count()).into());
    }
}

fn initialize_arbitrary_equitable_coloring(graph: &PaddedGraph, cc: usize) -> Vec<ColorIndex> {
    Vec::from_iter(graph.iter_nodes().map(|index| ColorIndex(index.0 % cc)))
}

type EdgeMap = IndexMap<NodeIndex, IndexSet<NodeIndex>>;

fn initialize_edge_map(graph: &PaddedGraph) -> EdgeMap {
    EdgeMap::from_iter(graph.iter_nodes().map(|node| (node, IndexSet::new())))
}

type ColorMap = IndexMap<ColorIndex, IndexSet<NodeIndex>>;

fn initialize_color_map(c: &[ColorIndex], cc: usize) -> ColorMap {
    let mut map = ColorMap::with_capacity(cc);
    for (node, color) in c.iter().enumerate() {
        map.entry(*color).or_default().insert(node.into());
    }

    map
}

type NeighbourhoodMap = IndexMap<(NodeIndex, ColorIndex), usize>;

fn initialize_neighbourhood_map(e_map: &EdgeMap, c_map: &ColorMap) -> NeighbourhoodMap {
    let mut map = NeighbourhoodMap::with_capacity(e_map.len() + c_map.len());
    for node in e_map.keys() {
        for color in c_map.keys() {
            map.insert((*node, *color), 0);
        }
    }

    map
}

type WitnessMap = IndexMap<(ColorIndex, ColorIndex), usize>;

fn initialize_witness_map(c_map: &ColorMap, n_map: &NeighbourhoodMap) -> WitnessMap {
    let mut map = WitnessMap::with_capacity(c_map.len() * c_map.len());
    for (c1, nodes) in c_map {
        for c2 in c_map.keys() {
            let count = nodes
                .iter()
                .filter(|node| {
                    let key = (**node, *c2);
                    *n_map
                        .get(&key)
                        .expect("failed to find a color count for withness map")
                        == 0
                })
                .count();
            map.insert((*c1, *c2), count);
        }
    }

    map
}

#[allow(clippy::too_many_arguments)]
fn change_color(
    node: NodeIndex,
    old_c: ColorIndex,
    new_c: ColorIndex,
    c: &mut [ColorIndex],
    c_map: &mut ColorMap,
    n_map: &mut NeighbourhoodMap,
    w_map: &mut WitnessMap,
    e_map: &EdgeMap,
) {
    c[node.0] = new_c;

    for color in c_map.keys() {
        // `u` witnesses an edge from color -> new_color instead of from color -> old_color now.
        let count = *n_map
            .get(&(node, *color))
            .expect("could not find neighbourhood count");
        if count == 0 {
            let key = (old_c, *color);
            *w_map.get_mut(&key).expect("could not find witness count") -= 1;

            let key = (new_c, *color);
            *w_map.get_mut(&key).expect("could not find witness count") += 1;
        }
    }

    let n = &e_map[node.0];
    for v in n {
        // `v` has lost a neighbor with color `old_color` and gained one with color `new_color`
        {
            let key = (*v, old_c);
            let old_color_count = n_map
                .get_mut(&key)
                .expect("could not find neighbourhood count");
            *old_color_count -= 1;

            if *old_color_count == 0 {
                // `v` witnesses `coloring[v]` -> `old_color`
                let v_color = c[v.0];

                let key = (v_color, old_c);
                *w_map.get_mut(&key).expect("could not find witness count") += 1;
            }
        }

        let key = (*v, new_c);
        let new_color_count = n_map
            .get_mut(&key)
            .expect("could not find neighbourhood count");
        *new_color_count += 1;

        if *new_color_count == 1 {
            // `v` no longer witnesses `coloring[v]` -> `new_color`
            let v_color = c[v.0];

            let key = (v_color, new_c);
            *w_map.get_mut(&key).expect("could not find witness count") -= 1;
        }
    }

    c_map[old_c.0].swap_remove(&node);
    c_map[new_c.0].insert(node);
}

#[allow(clippy::too_many_arguments)]
fn procedure_p(
    v_minus: ColorIndex,
    v_plus: ColorIndex,
    n_map: &mut NeighbourhoodMap,
    w_map: &mut WitnessMap,
    c: &mut Vec<ColorIndex>,
    c_map: &mut ColorMap,
    e_map: &EdgeMap,
    excluded_colors: &mut HashSet<ColorIndex>,
) {
    let mut a_cal = HashSet::<ColorIndex>::new();
    let mut t_cal = HashMap::<ColorIndex, ColorIndex>::new();
    let mut r_cal = Vec::<ColorIndex>::new();

    // BFS to determine a_cal, i.e., colors reachable from v_minus
    let mut reachable = vec![v_minus];
    let mut marked = HashSet::<ColorIndex>::from_iter([v_minus]);
    let mut idx = 0;

    while idx < reachable.len() {
        let pop = reachable[idx];

        idx += 1;

        a_cal.insert(pop);
        r_cal.push(pop);

        let mut next_layer = Vec::<ColorIndex>::new();

        for color in c_map.keys() {
            let key = (*color, pop);
            let w_count = *w_map.get(&key).expect("could not find witness count");
            if w_count > 0
                && !a_cal.contains(color)
                && !excluded_colors.contains(color)
                && !marked.contains(color)
            {
                next_layer.push(*color);
            }
        }

        for dst in &next_layer {
            t_cal.insert(*dst, pop);
        }

        marked.extend(next_layer.iter().cloned());
        reachable.extend(next_layer);
    }

    // variables for the algorithm
    let b = c_map.len() - a_cal.len();

    if a_cal.contains(&v_plus) {
        // Easy case: v_plus is in a_cal
        // move one node from v_plus to v_minus using t_cal to find the parents.
        move_witness(v_plus, v_minus, n_map, w_map, c, c_map, &t_cal, e_map);
    } else {
        // If there is a solo edge, we can resolve the situation by
        // moving witnesses from B to A, making G[A] equitable and then
        // recursively balancing G[B - w] with a different V_minus and
        // but the same V_plus.

        let mut a_0 = HashSet::<NodeIndex>::new();
        let mut a_cal_0 = HashSet::<ColorIndex>::new();

        let mut num_terminal_sets_found = 0;
        let mut made_equitable = false;

        for w_1 in r_cal.iter().cloned().rev() {
            let did_break = 'mark: {
                for v in c_map[w_1.0].clone().into_iter() {
                    let mut x = Option::<ColorIndex>::None;

                    for color in c_map.keys().cloned() {
                        let key = (v, color);
                        let color_count = *n_map
                            .get(&key)
                            .expect("invalid index into neighbourhood map");
                        if color_count == 0 && a_cal.contains(&color) && color != w_1 {
                            x = Some(color);
                        }
                    }

                    // v does not witness an edge in H[a_cal]
                    let Some(x) = x else {
                        continue;
                    };

                    for color in c_map.keys().cloned() {
                        // Note: Departing from the paper here.

                        let key = (v, color);
                        let color_count = *n_map
                            .get(&key)
                            .expect("invalid index into neighbourhood map");

                        if color_count >= 1 && !a_cal.contains(&color) {
                            let x_prime = color;
                            let w = v;

                            let y = e_map[w.0]
                                .iter()
                                .find(|neighbour| {
                                    let color = c[neighbour.0];

                                    let key = (**neighbour, w_1);
                                    let color_count = *n_map
                                        .get(&key)
                                        .expect("invalid index into neighbourhood map");

                                    color == x_prime && color_count == 1
                                })
                                .expect("should find a y");

                            let capital_w = w_1;

                            // Move w from W to X, now X has one extra node.
                            change_color(w, capital_w, x, c, c_map, n_map, w_map, e_map);

                            // Move witness from X to v_minus, making the coloring
                            // equitable.
                            move_witness(x, v_minus, n_map, w_map, c, c_map, &t_cal, e_map);

                            // Move y from x_prime to W, making W the correct size.
                            change_color(*y, x_prime, capital_w, c, c_map, n_map, w_map, e_map);

                            // then call the procedure on G[B - y]
                            procedure_p(
                                x_prime,
                                v_plus,
                                n_map,
                                w_map,
                                c,
                                c_map,
                                e_map,
                                excluded_colors,
                            );
                            made_equitable = true;
                            break;
                        }
                    }

                    if made_equitable {
                        break 'mark true;
                    }
                }

                // did not break in for loop
                false
            };

            if !did_break {
                // No node in W_1 was found such that it had a solo neighbour.
                a_cal_0.insert(w_1);
                a_0.extend(c_map[w_1.0].iter());
                num_terminal_sets_found += 1;
            }

            if num_terminal_sets_found == b {
                // Otherwise, construct the maximal independent set and find
                // a pair of z_1, z_2 as in case II.

                // BFS to determine b_cal': the set of colors reachable from v_plus
                let mut b_cal_prime = HashSet::<ColorIndex>::new();
                let mut t_cal_prime = HashMap::<ColorIndex, ColorIndex>::new();

                let mut reachable = vec![v_plus];
                let mut marked = HashSet::<ColorIndex>::from_iter([v_plus]);
                let mut idx = 0;

                while idx < reachable.len() {
                    let pop = reachable[idx];
                    idx += 1;

                    b_cal_prime.insert(pop);

                    // No need to check for excluded colors here because they only exclude colors
                    // from A_cal
                    let next_layer = Vec::from_iter(
                        c_map
                            .keys()
                            .filter(|color| {
                                let key = (pop, **color);
                                *w_map.get(&key).expect("invalid index into witness map") > 0
                            })
                            .filter(|color| !b_cal_prime.contains(*color))
                            .filter(|color| !marked.contains(*color))
                            .cloned(),
                    );

                    for dst in next_layer.iter().cloned() {
                        t_cal_prime.insert(pop, dst);
                    }

                    marked.extend(next_layer.iter().cloned());
                    reachable.extend(next_layer);
                }

                // Construct the idependent set of G[B']
                let mut i_set = HashSet::<NodeIndex>::new();
                let mut i_covered = HashSet::<NodeIndex>::new();
                let mut w_covering = HashMap::<NodeIndex, NodeIndex>::new();

                let b_prime =
                    Vec::from_iter(b_cal_prime.iter().flat_map(|color| c_map[color.0].clone()));

                // Add the nodes in v_plus to I first.
                for z in c_map[v_plus.0]
                    .clone()
                    .into_iter()
                    .chain(b_prime.iter().cloned())
                {
                    if i_covered.contains(&z) || !b_cal_prime.contains(&c[z.0]) {
                        continue;
                    }

                    i_set.insert(z);
                    i_covered.insert(z);
                    let n = &e_map[z.0];
                    i_covered.extend(n);

                    for w in n {
                        let color = c[w.0];

                        let key = (z, color);
                        let n_cc = *n_map.get(&key).expect("invalid key into neighbourhood map");

                        if a_cal_0.contains(&color) && n_cc == 1 {
                            if !w_covering.contains_key(w) {
                                w_covering.insert(*w, z);
                            } else {
                                // Found z1, z2 which have the same solo neighbour in some W
                                let z_1 = w_covering[w];

                                let capital_z = c[z_1.0];
                                let capital_w = c[w.0];

                                move_witness(
                                    capital_w, v_minus, n_map, w_map, c, c_map, &t_cal, e_map,
                                );

                                // shift nodes along v_plus to Z
                                move_witness(
                                    capital_w, v_minus, n_map, w_map, c, c_map, &t_cal, e_map,
                                );

                                // change color of z_1 to W
                                change_color(
                                    z_1, capital_z, capital_w, c, c_map, n_map, w_map, e_map,
                                );

                                // change color of w to some color in B_cal
                                let capital_w_plus = *c_map
                                    .keys()
                                    .filter(|color| {
                                        let key = (*w, **color);
                                        *n_map
                                            .get(&key)
                                            .expect("invalid index into neighbourhood map")
                                            == 0
                                    })
                                    .find(|color| !b_cal_prime.contains(*color))
                                    .expect("should find an unused color");
                                change_color(
                                    *w,
                                    capital_w,
                                    capital_w_plus,
                                    c,
                                    c_map,
                                    n_map,
                                    w_map,
                                    e_map,
                                );

                                // recurse with G[B \cup W*]
                                excluded_colors.extend(
                                    c_map
                                        .keys()
                                        .filter(|color| **color != capital_w)
                                        .filter(|color| !b_cal_prime.contains(*color)),
                                );

                                procedure_p(
                                    capital_w,
                                    capital_w_plus,
                                    n_map,
                                    w_map,
                                    c,
                                    c_map,
                                    e_map,
                                    excluded_colors,
                                );

                                made_equitable = true;
                                break;
                            }
                        }
                    }

                    if made_equitable {
                        break;
                    }
                }
            }
            if made_equitable {
                continue;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn move_witness(
    mut src: ColorIndex,
    dst: ColorIndex,
    n_map: &mut NeighbourhoodMap,
    w_map: &mut WitnessMap,
    c: &mut [ColorIndex],
    c_map: &mut ColorMap,
    t_cal: &HashMap<ColorIndex, ColorIndex>,
    e_map: &EdgeMap,
) {
    while src != dst {
        let y = *t_cal.get(&src).expect("invalid index into t_cal");
        let w = *c_map[src.0]
            .iter()
            .find(|node| {
                let key = (**node, y);
                *n_map
                    .get(&key)
                    .expect("could not find neighbourhood color count")
                    == 0
            })
            .expect("could not find a witness");
        change_color(w, src, y, c, c_map, n_map, w_map, e_map);
        src = y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_equitable() {
        let graph = DiGraphBuilder::with_node_and_edge_count(3, 2)
            .add_nodes(3)
            .add_edges(&[(0, 1).into(), (1, 2).into()])
            .build();
        let coloring: Vec<ColorIndex> = vec![0.into(), 1.into(), 0.into()];

        assert!(is_equitable(graph.iter_edges(), &coloring, None));
    }

    #[test]
    fn test_color_count() {
        let graph = DiGraphBuilder::with_node_and_edge_count(4, 3)
            .add_nodes(4)
            .add_edges(&[(0, 1).into(), (0, 2).into(), (0, 3).into()])
            .build();

        let result = equitable_coloring(&graph, 2);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            Error::NotEnoughColors {
                given: 2,
                expected: 4
            }
        );
    }

    #[test]
    fn test_equitable_color() {
        let graph = DiGraphBuilder::with_node_and_edge_count(8, 13)
            .add_nodes(8)
            .add_edges(&[
                (0, 1).into(),
                (0, 2).into(),
                (0, 3).into(),
                (1, 2).into(),
                (1, 3).into(),
                (1, 7).into(),
                (2, 3).into(),
                (2, 4).into(),
                (3, 4).into(),
                (3, 6).into(),
                (4, 5).into(),
                (5, 6).into(),
                (6, 7).into(),
            ])
            .build();

        let result = equitable_coloring(&graph, 6);
        assert!(result.is_ok());
        let coloring = result.unwrap();
        assert!(is_equitable(graph.iter_edges(), &coloring, None));
    }

    #[test]
    fn test_equitable_color_with_padding() {
        let graph = DiGraphBuilder::with_node_and_edge_count(7, 11)
            .add_nodes(7)
            .add_edges(&[
                (0, 1).into(),
                (0, 2).into(),
                (0, 3).into(),
                (1, 2).into(),
                (1, 3).into(),
                (2, 3).into(),
                (2, 4).into(),
                (3, 4).into(),
                (3, 6).into(),
                (4, 5).into(),
                (5, 6).into(),
            ])
            .build();

        let result = equitable_coloring(&graph, 6);
        assert!(result.is_ok());
        let coloring = result.unwrap();
        assert!(is_equitable(graph.iter_edges(), &coloring, None));
    }
}
