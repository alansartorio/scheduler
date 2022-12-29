use super::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Task<T> {
    pub span: Span,
    pub info: T,
}

impl<T> Task<T> {
    pub fn new(span: Span, info: T) -> Task<T> {
        Task { span, info }
    }
}
