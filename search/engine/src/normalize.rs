pub(crate) const MAX_LIMIT: u32 = 100;
const MAX_QUERY_DEPTH: usize = 32;
const MAX_BOOLEAN_NODES: usize = 128;
const MAX_TERMS: usize = 128;
const MAX_ANY_IN_VALUES: usize = 256;

#[derive(Debug, Clone)]
pub(crate) struct ValidatedRequest {
    pub(crate) query: Option<crate::api::query::input::QueryNode>,
    pub(crate) sort: crate::api::query::input::SortSpec,
    pub(crate) limit: u32,
    pub(crate) cursor: Option<crate::api::pagination::Cursor>,
    pub(crate) total_mode: crate::api::query::types::TotalMode,
    pub(crate) query_fingerprint: u64,
}

#[derive(Debug, Default)]
struct ComplexityState {
    boolean_nodes: usize,
    terms: usize,
}

pub(crate) fn validate_and_normalize_request(
    request: &crate::api::query::input::SearchRequest,
) -> Result<ValidatedRequest, crate::EngineError> {
    let sort = validate_sort(&request.sort)?;
    if request.page.limit == 0 {
        return Err(crate::EngineError::InvalidRequest(
            "page limit must be greater than zero",
        ));
    }
    if request.page.limit > MAX_LIMIT {
        return Err(crate::EngineError::InvalidRequest(
            "page limit exceeds maximum",
        ));
    }

    let query = match &request.query {
        Some(query) => Some(normalize_query(query)?),
        None => None,
    };
    let query_fingerprint = fingerprint_query(query.as_ref());

    Ok(ValidatedRequest {
        query,
        sort,
        limit: request.page.limit,
        cursor: request.page.cursor.clone(),
        total_mode: request.total_mode,
        query_fingerprint,
    })
}

fn validate_sort(
    sort: &[crate::api::query::input::SortSpec],
) -> Result<crate::api::query::input::SortSpec, crate::EngineError> {
    if sort.len() != 1 {
        return Err(crate::EngineError::InvalidRequest(
            "exactly one sort spec is required",
        ));
    }

    let sort = sort[0].clone();
    if sort.field != crate::api::query::types::SortField::PublishedAt {
        return Err(crate::EngineError::InvalidRequest("unsupported sort field"));
    }

    Ok(sort)
}

fn normalize_query(
    query: &crate::api::query::input::QueryNode,
) -> Result<crate::api::query::input::QueryNode, crate::EngineError> {
    let mut state = ComplexityState::default();
    normalize_node(query, false, 1, &mut state)
}

fn normalize_node(
    query: &crate::api::query::input::QueryNode,
    negated: bool,
    depth: usize,
    state: &mut ComplexityState,
) -> Result<crate::api::query::input::QueryNode, crate::EngineError> {
    use crate::EngineError;
    use crate::api::query::input::QueryNode;

    if depth > MAX_QUERY_DEPTH {
        return Err(EngineError::QueryTooComplex("query depth exceeds maximum"));
    }

    match query {
        QueryNode::And { children } => {
            if children.is_empty() {
                return Err(EngineError::InvalidRequest(
                    "and must contain at least one child",
                ));
            }
            state.boolean_nodes += 1;
            if state.boolean_nodes > MAX_BOOLEAN_NODES {
                return Err(EngineError::QueryTooComplex(
                    "boolean node count exceeds maximum",
                ));
            }

            let mut normalized_children = Vec::new();
            for child in children {
                let normalized = normalize_node(child, negated, depth + 1, state)?;
                push_boolean_child(&mut normalized_children, normalized, !negated);
            }
            collapse_boolean_node(normalized_children, !negated)
        }
        QueryNode::Or { children } => {
            if children.is_empty() {
                return Err(EngineError::InvalidRequest(
                    "or must contain at least one child",
                ));
            }
            state.boolean_nodes += 1;
            if state.boolean_nodes > MAX_BOOLEAN_NODES {
                return Err(EngineError::QueryTooComplex(
                    "boolean node count exceeds maximum",
                ));
            }

            let mut normalized_children = Vec::new();
            for child in children {
                let normalized = normalize_node(child, negated, depth + 1, state)?;
                push_boolean_child(&mut normalized_children, normalized, negated);
            }
            collapse_boolean_node(normalized_children, negated)
        }
        QueryNode::Not { child } => normalize_node(child, !negated, depth + 1, state),
        QueryNode::Term(term) => {
            state.terms += 1;
            if state.terms > MAX_TERMS {
                return Err(EngineError::QueryTooComplex("term count exceeds maximum"));
            }

            let normalized_term = normalize_term(term)?;
            let term_node = QueryNode::Term(normalized_term);
            Ok(if negated {
                QueryNode::Not {
                    child: Box::new(term_node),
                }
            } else {
                term_node
            })
        }
    }
}

fn normalize_term(
    term: &crate::api::query::input::TermNode,
) -> Result<crate::api::query::input::TermNode, crate::EngineError> {
    use crate::api::query::input::TermNode;

    match term {
        TermNode::ArtistAnyIn { values } => Ok(TermNode::ArtistAnyIn {
            values: normalize_values(values)?,
        }),
        TermNode::TagAnyIn { values } => Ok(TermNode::TagAnyIn {
            values: normalize_values(values)?,
        }),
        TermNode::ChannelAnyIn { values } => Ok(TermNode::ChannelAnyIn {
            values: normalize_values(values)?,
        }),
        TermNode::IsUnlistedEq { value } => {
            Ok(TermNode::IsUnlistedEq { value: *value })
        }
        TermNode::EmbeddableEq { value } => {
            Ok(TermNode::EmbeddableEq { value: *value })
        }
        TermNode::PublishedAtRange(range) => {
            Ok(TermNode::PublishedAtRange(range.clone()))
        }
    }
}

fn normalize_values<T: Ord + Clone>(
    values: &[T],
) -> Result<Vec<T>, crate::EngineError> {
    if values.is_empty() {
        return Err(crate::EngineError::InvalidRequest(
            "any_in must contain at least one value",
        ));
    }
    if values.len() > MAX_ANY_IN_VALUES {
        return Err(crate::EngineError::QueryTooComplex(
            "any_in value count exceeds maximum",
        ));
    }

    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    Ok(values)
}

fn push_boolean_child(
    target: &mut Vec<crate::api::query::input::QueryNode>,
    child: crate::api::query::input::QueryNode,
    is_and: bool,
) {
    use crate::api::query::input::QueryNode;

    match child {
        QueryNode::And { children } if is_and => {
            target.extend(children);
        }
        QueryNode::Or { children } if !is_and => {
            target.extend(children);
        }
        child => target.push(child),
    }
}

fn collapse_boolean_node(
    mut children: Vec<crate::api::query::input::QueryNode>,
    is_and: bool,
) -> Result<crate::api::query::input::QueryNode, crate::EngineError> {
    use crate::api::query::input::QueryNode;

    if children.is_empty() {
        return Err(crate::EngineError::InvalidRequest(
            "boolean query must contain at least one child",
        ));
    }

    if children.len() == 1 {
        Ok(children.pop().expect("single child exists"))
    } else if is_and {
        Ok(QueryNode::And { children })
    } else {
        Ok(QueryNode::Or { children })
    }
}

fn fingerprint_query(query: Option<&crate::api::query::input::QueryNode>) -> u64 {
    use std::hash::Hash;
    use std::hash::Hasher;

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    query.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_normalize_flattens_nested_and() {
        let query = crate::api::query::input::QueryNode::And {
            children: vec![
                crate::api::query::input::QueryNode::And {
                    children: vec![crate::api::query::input::QueryNode::Term(
                        crate::api::query::input::TermNode::IsUnlistedEq {
                            value: false,
                        },
                    )],
                },
                crate::api::query::input::QueryNode::Term(
                    crate::api::query::input::TermNode::EmbeddableEq { value: true },
                ),
            ],
        };

        let normalized = super::normalize_query(&query).unwrap();
        match normalized {
            crate::api::query::input::QueryNode::And { children } => {
                assert_eq!(children.len(), 2);
            }
            _ => panic!("expected and"),
        }
    }
}
