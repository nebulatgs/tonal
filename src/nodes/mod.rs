use crate::tokens::Token;
pub(super) use std::fmt;

#[derive(Debug)]
pub struct ProgramNode<'src> {
    pub imports: Vec<ImportDeclarationNode<'src>>,
    pub meta: MetaDeclarationNode<'src>,
    pub declarations: Vec<DeclarationNode<'src>>,
}

#[derive(Debug)]
pub struct DeclarationNode<'src> {
    pub staff: StaffDeclarationNode<'src>,
}
#[derive(Debug)]
pub struct StaffDeclarationNode<'src: 'src> {
    pub identifier: Token<'src>,
    pub staff_type: CallNode<'src>,
    pub signature: Token<'src>,
    pub pickup: Option<PickupNode<'src>>,
    pub statements: Vec<StaffStatementNode<'src>>,
}
#[derive(Debug)]
pub struct PickupNode<'src> {
    pub block: BlockNode<'src>,
}
#[derive(Debug)]
pub struct MeasureNode<'src> {
    pub block: BlockNode<'src>,
}
#[derive(Debug)]
pub struct BlockNode<'src> {
    pub calls: Vec<CallWithNode<'src>>,
}
#[derive(Debug)]
pub struct CallWithNode<'src> {
    pub call: CallNode<'src>,
    pub with: Vec<WithNode<'src>>,
}
#[derive(Debug)]
pub struct WithNode<'src> {
    pub identifier: Option<Token<'src>>,
    pub call: Option<CallNode<'src>>,
}
#[derive(Debug)]
pub struct StaffStatementNode<'src> {
    pub measure: Option<MeasureNode<'src>>,
    pub call: Option<CallNode<'src>>,
}
#[derive(Debug)]
pub struct MetaDeclarationNode<'src> {
    pub configs: Vec<CallNode<'src>>,
}

#[derive(Debug)]
pub struct CallNode<'src> {
    pub identifier: Token<'src>,
    pub arguments: Vec<ArgumentNode<'src>>,
}
#[derive(Debug)]
pub struct ArgumentNode<'src> {
    pub argument: Token<'src>,
}
#[derive(Debug)]
pub struct ImportDeclarationNode<'src> {
    pub items: Vec<Token<'src>>,
    pub source: Token<'src>,
}

impl<'src> ImportDeclarationNode<'src> {
    fn items(&self) -> Result<String, std::fmt::Error> {
        use std::fmt::Write;
        let mut s = String::new();
        for item in &self.items[..self.items.len() - 1] {
            write!(s, "{}, ", item)?;
        }
        write!(s, "{}, ", self.items.last().unwrap())?;
        Ok(s)
    }
}

impl<'src> fmt::Display for ImportDeclarationNode<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "import {{ {} }} from {}", self.items()?, self.source)
    }
}
