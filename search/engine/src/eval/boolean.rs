pub(crate) fn evaluate_node(
    context: &super::context::EvalContext<'_>,
    query: &crate::api::query::resolved::QueryNode,
) -> Result<crate::doc_set::DocSet, crate::EngineError> {
    use crate::api::query::resolved::QueryNode;
    use crate::doc_set::DocSet;

    match query {
        QueryNode::All => Ok(DocSet::All),
        QueryNode::Empty => Ok(DocSet::Empty),
        QueryNode::And { children } => evaluate_and(context, children),
        QueryNode::Or { children } => evaluate_or(context, children),
        QueryNode::Not { child } => {
            let evaluated = evaluate_node(context, child)?;
            DocSet::difference(&DocSet::All, &evaluated, context.record_count)
        }
        QueryNode::Term(term) => super::terms::evaluate_term(context, term),
    }
}

fn evaluate_and(
    context: &super::context::EvalContext<'_>,
    children: &[crate::api::query::resolved::QueryNode],
) -> Result<crate::doc_set::DocSet, crate::EngineError> {
    use crate::doc_set::DocSet;

    let mut current = DocSet::All;
    for child in children {
        let evaluated = evaluate_node(context, child)?;
        current = DocSet::intersect(&current, &evaluated, context.record_count)?;
        if current.is_empty() {
            break;
        }
    }
    Ok(current)
}

fn evaluate_or(
    context: &super::context::EvalContext<'_>,
    children: &[crate::api::query::resolved::QueryNode],
) -> Result<crate::doc_set::DocSet, crate::EngineError> {
    use crate::doc_set::DocSet;

    let mut current = DocSet::Empty;
    for child in children {
        let evaluated = evaluate_node(context, child)?;
        current = DocSet::union(&current, &evaluated, context.record_count)?;
        if matches!(current, DocSet::All) {
            break;
        }
    }
    Ok(current)
}
