use syn::punctuated::Punctuated;
use syn::{Attribute, Expr, Ident, Token};

mod expand;
mod parse;

/// Expand the `quantity!` macro invocation.
pub(crate) use expand::expand;

/// Quantity definition. TODO: Remove `allow(dead_code)` once parsing/expansion implemented.
#[allow(dead_code)]
pub(crate) struct Quantity {
    name_attributes: Vec<Attribute>,
    name: Ident,
    description: Expr,
    dimension_attributes: Vec<Attribute>,
    system: Ident,
    dimensions: Punctuated<Ident, Token![,]>,
    kind: Option<Ident>,
    units: Punctuated<Unit, Token![;]>,
}

/// Unit definition. TODO: Remove `allow(dead_code)` once parsing/expansion implemented.
#[allow(dead_code)]
pub(crate) struct Unit {
    attributes: Vec<Attribute>,
    name: Ident,
    coefficient: Expr,
    constant: Option<Expr>,
    abbreviation: Expr,
    singular: Expr,
    plural: Expr,
}
