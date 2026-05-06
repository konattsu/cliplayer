pub(crate) fn resolve_query(
    query: Option<&crate::api::query::input::QueryNode>,
    dictionaries: &crate::index::DictionaryCaches,
) -> Result<
    (
        crate::api::query::resolved::QueryNode,
        Vec<crate::api::response::QueryWarning>,
    ),
    crate::EngineError,
> {
    let mut warnings = Vec::new();
    let resolved = match query {
        Some(query) => resolve_node(query, dictionaries, &mut warnings)?,
        None => crate::api::query::resolved::QueryNode::All,
    };
    Ok((resolved, warnings))
}

fn resolve_node(
    query: &crate::api::query::input::QueryNode,
    dictionaries: &crate::index::DictionaryCaches,
    warnings: &mut Vec<crate::api::response::QueryWarning>,
) -> Result<crate::api::query::resolved::QueryNode, crate::EngineError> {
    match query {
        crate::api::query::input::QueryNode::And { children } => {
            let mut resolved_children = Vec::new();
            for child in children {
                match resolve_node(child, dictionaries, warnings)? {
                    crate::api::query::resolved::QueryNode::All => {}
                    crate::api::query::resolved::QueryNode::Empty => {
                        return Ok(crate::api::query::resolved::QueryNode::Empty);
                    }
                    child => resolved_children.push(child),
                }
            }
            Ok(match resolved_children.len() {
                0 => crate::api::query::resolved::QueryNode::All,
                1 => resolved_children.pop().expect("single child exists"),
                _ => crate::api::query::resolved::QueryNode::And {
                    children: resolved_children,
                },
            })
        }
        crate::api::query::input::QueryNode::Or { children } => {
            let mut resolved_children = Vec::new();
            for child in children {
                match resolve_node(child, dictionaries, warnings)? {
                    crate::api::query::resolved::QueryNode::All => {
                        return Ok(crate::api::query::resolved::QueryNode::All);
                    }
                    crate::api::query::resolved::QueryNode::Empty => {}
                    child => resolved_children.push(child),
                }
            }
            Ok(match resolved_children.len() {
                0 => crate::api::query::resolved::QueryNode::Empty,
                1 => resolved_children.pop().expect("single child exists"),
                _ => crate::api::query::resolved::QueryNode::Or {
                    children: resolved_children,
                },
            })
        }
        crate::api::query::input::QueryNode::Not { child } => {
            Ok(match resolve_node(child, dictionaries, warnings)? {
                crate::api::query::resolved::QueryNode::All => {
                    crate::api::query::resolved::QueryNode::Empty
                }
                crate::api::query::resolved::QueryNode::Empty => {
                    crate::api::query::resolved::QueryNode::All
                }
                child => crate::api::query::resolved::QueryNode::Not {
                    child: Box::new(child),
                },
            })
        }
        crate::api::query::input::QueryNode::Term(term) => {
            resolve_term(term, dictionaries, warnings)
        }
    }
}

fn resolve_term(
    term: &crate::api::query::input::TermNode,
    dictionaries: &crate::index::DictionaryCaches,
    warnings: &mut Vec<crate::api::response::QueryWarning>,
) -> Result<crate::api::query::resolved::QueryNode, crate::EngineError> {
    match term {
        crate::api::query::input::TermNode::ArtistAnyIn { values } => resolve_any_in(
            values,
            &dictionaries.artists,
            warnings,
            |value| crate::api::response::QueryWarning::UnknownArtistId {
                value: value.to_string(),
            },
            |values| crate::api::query::resolved::TermNode::ArtistAnyIn { values },
        ),
        crate::api::query::input::TermNode::TagAnyIn { values } => resolve_any_in(
            values,
            &dictionaries.tags,
            warnings,
            |value| crate::api::response::QueryWarning::UnknownTagId {
                value: value.to_string(),
            },
            |values| crate::api::query::resolved::TermNode::TagAnyIn { values },
        ),
        crate::api::query::input::TermNode::ChannelAnyIn { values } => resolve_any_in(
            values,
            &dictionaries.channels,
            warnings,
            |value| crate::api::response::QueryWarning::UnknownChannelId {
                value: value.to_string(),
            },
            |values| crate::api::query::resolved::TermNode::ChannelAnyIn { values },
        ),
        crate::api::query::input::TermNode::IsUnlistedEq { value } => {
            Ok(crate::api::query::resolved::QueryNode::Term(
                crate::api::query::resolved::TermNode::IsUnlistedEq { value: *value },
            ))
        }
        crate::api::query::input::TermNode::EmbeddableEq { value } => {
            Ok(crate::api::query::resolved::QueryNode::Term(
                crate::api::query::resolved::TermNode::EmbeddableEq { value: *value },
            ))
        }
        crate::api::query::input::TermNode::PublishedAtRange(range) => {
            if is_empty_range(range) {
                Ok(crate::api::query::resolved::QueryNode::Empty)
            } else {
                Ok(crate::api::query::resolved::QueryNode::Term(
                    crate::api::query::resolved::TermNode::PublishedAtRange(
                        range.clone(),
                    ),
                ))
            }
        }
    }
}

fn resolve_any_in<Id, WarningFn, TermFn>(
    values: &[String],
    dictionary: &std::collections::HashMap<std::sync::Arc<str>, Id>,
    warnings: &mut Vec<crate::api::response::QueryWarning>,
    make_warning: WarningFn,
    make_term: TermFn,
) -> Result<crate::api::query::resolved::QueryNode, crate::EngineError>
where
    Id: Copy + Ord,
    WarningFn: Fn(&str) -> crate::api::response::QueryWarning,
    TermFn: Fn(Vec<Id>) -> crate::api::query::resolved::TermNode,
{
    let mut resolved = Vec::with_capacity(values.len());
    for value in values {
        if let Some(id) = dictionary.get(value.as_str()) {
            resolved.push(*id);
        } else {
            warnings.push(make_warning(value));
        }
    }

    resolved.sort();
    resolved.dedup();

    Ok(match resolved.is_empty() {
        true => crate::api::query::resolved::QueryNode::Empty,
        false => crate::api::query::resolved::QueryNode::Term(make_term(resolved)),
    })
}

fn is_empty_range(range: &crate::api::query::types::DateRange) -> bool {
    match (range.lower, range.upper) {
        (Some(lower), Some(upper)) => {
            lower.value > upper.value
                || (lower.value == upper.value && !(lower.inclusive && upper.inclusive))
        }
        _ => false,
    }
}
