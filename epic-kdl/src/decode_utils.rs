pub fn validate_single_arg<S: ::knuffel::traits::ErrorSpan>(
    ctx: &mut ::knuffel::decode::Context<S>,
    node: &::knuffel::ast::SpannedNode<S>,
    mut iter_args: std::slice::Iter<knuffel::ast::Value<S>>,
) -> Result<(), ::knuffel::errors::DecodeError<S>> {
    if let Some(val) = iter_args.next() {
        ctx.emit_error(::knuffel::errors::DecodeError::unexpected(
            &val.literal,
            "argument",
            "unexpected argument",
        ));
    }
    for (name, val) in node.properties.iter() {
        match &***name {
            name_str => {
                ctx.emit_error(::knuffel::errors::DecodeError::unexpected(
                    name,
                    "property",
                    format!("unexpected property `{}`", name_str.escape_default()),
                ));
            }
        }
    }
    let children = node.children.as_ref().map(|lst| &lst[..]).unwrap_or(&[]);
    children
        .iter()
        .flat_map(|child| match &**child.node_name {
            name_str => {
                ctx.emit_error(::knuffel::errors::DecodeError::unexpected(
                    child,
                    "node",
                    format!("unexpected node `{}`", name_str.escape_default()),
                ));
                None
            }
        })
        .collect::<Result<(), ::knuffel::errors::DecodeError<_>>>()
}
