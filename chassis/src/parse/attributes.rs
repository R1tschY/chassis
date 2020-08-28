use proc_macro2::TokenStream;
use syn::{Attribute, PathArguments, PathSegment};

use crate::utils::to_tokens;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum InjectAttrType {
    Annotation,
    Singleton,
}

pub struct InjectAttr {
    pub ty: InjectAttrType,
    pub tokens: TokenStream,
}

pub fn is_chassis_attr(attr: &Attribute) -> bool {
    let segs = &attr.path.segments;
    segs.len() == 1 && (segs[0].ident == "annotation" || segs[0].ident == "singleton")
}

pub fn parse_attr(attr: Attribute) -> InjectAttr {
    let parts: Vec<&PathSegment> = attr.path.segments.iter().collect();
    for part in &parts {
        if let PathArguments::None = part.arguments {
            // okay
        } else {
            panic!(
                "Unsupported part arguments in chassis attribute: {}",
                to_tokens(&attr)
            );
        }
    }

    if parts.len() != 1 {
        panic!("Unknown chassis attribute: {}", to_tokens(&attr));
    }

    let ty = match &parts[0].ident.to_string() as &str {
        "annotation" => InjectAttrType::Annotation,
        "singleton" => InjectAttrType::Singleton,
        _ => panic!("Unknown chassis attribute: {}", to_tokens(&attr)),
    };

    InjectAttr {
        ty,
        tokens: attr.tokens,
    }
}

#[allow(dead_code)]
pub fn get_annotation_attribute(attr: &[InjectAttr]) -> Option<&InjectAttr> {
    attr.iter()
        .find(|attr| attr.ty == InjectAttrType::Annotation)
}
