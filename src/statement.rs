use crate::{expr::Expr, RuntimeError};

pub trait StmtVisitor {
    type O<'o>
    where
        Self: 'o;
    type E<'e>
    where
        Self: 'e;
    fn visit_expression<'b>(&'b self, n: &'b Expression<'b>) -> Result<Self::O<'b>, Self::E<'b>>;
    fn visit_print<'g>(&'g self, n: &'g Print<'g>) -> Result<Self::O<'g>, Self::E<'g>>;
}

pub trait StmtAcceptor {
    type O<'o>
    where
        Self: 'o;
    type E<'e>
    where
        Self: 'e;
    fn accept<'a>(
        &'a self,
        n: &'a impl StmtVisitor<O<'a> = Self::O<'a>, E<'a> = Self::E<'a>>,
    ) -> Result<Self::O<'a>, Self::E<'a>>;
}

#[derive(Debug)]
pub enum Statement<'e> {
    Expression(Expression<'e>),
    Print(Print<'e>),
}

impl StmtAcceptor for Statement<'_> {
    type O<'o> = () where Self: 'o;
    type E<'e> = RuntimeError<'e> where Self: 'e;
    fn accept<'a>(
        &'a self,
        n: &'a impl StmtVisitor<O<'a> = (), E<'a> = RuntimeError<'a>>,
    ) -> Result<Self::O<'a>, Self::E<'a>> {
        match self {
            Self::Expression(x) => x.accept(n),
            Self::Print(x) => x.accept(n),
        }
    }
}

#[derive(Debug)]
pub struct Expression<'e> {
    pub expr: Expr<'e>,
}

impl<'e> Expression<'e> {
    pub fn new(expr: Expr<'e>) -> Self {
        Self { expr }
    }
}

impl StmtAcceptor for Expression<'_> {
    type O<'o> = () where Self: 'o;
    type E<'e> = RuntimeError<'e> where Self: 'e;
    fn accept<'a>(
        &'a self,
        n: &'a impl StmtVisitor<O<'a> = (), E<'a> = RuntimeError<'a>>,
    ) -> Result<Self::O<'a>, Self::E<'a>> {
        n.visit_expression(self)
    }
}

#[derive(Debug)]
pub struct Print<'p> {
    pub expr: Expr<'p>,
}

impl<'p> Print<'p> {
    pub fn new(expr: Expr<'p>) -> Self {
        Self { expr }
    }
}
impl StmtAcceptor for Print<'_> {
    type O<'o> = () where Self: 'o;
    type E<'e> = RuntimeError<'e> where Self: 'e;
    fn accept<'a>(
        &'a self,
        n: &'a impl StmtVisitor<O<'a> = (), E<'a> = RuntimeError<'a>>,
    ) -> Result<Self::O<'a>, Self::E<'a>> {
        n.visit_print(self)
    }
}
