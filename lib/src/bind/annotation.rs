use std::any::Any;
use std::fmt::Debug;

pub trait BindAnnotation: Debug + Any {}

#[derive(Debug)]
pub struct Named(pub &'static str);

impl BindAnnotation for Named {}
