use std::{marker::PhantomData, cell::{RefCell, Cell}};

use crate::{lexer::Lexer, tokens::{Token, Keyword, Separator}, nodes::*};


pub struct Parser<'src> {
    save_point: Cell<usize>,
    pos: Cell<usize>,
    tokens: RefCell<Vec<Token<'src>>>,
    lexer: Lexer<'src>,
}

impl<'src> Parser<'src> {
    pub fn new(lexer: Lexer<'src>) -> Self {
        Self {
            save_point: 0.into(),
            pos: 0.into(),
            tokens: vec![].into(),
            lexer,
        }
    }
    fn next(&self) -> Option<Token<'src>> {
        self.pos.set(self.pos.get() + 1);
        if let Some(token) = self.peek() {
            return Some(token);
        }
        let mut tokens = self.tokens.borrow_mut();
        tokens.push(self.lexer.lex()?);
        tokens.last().cloned()
    }
    fn prev(&self) -> Option<Token<'src>> {
        self.pos.set(self.pos.get() - 1);
        self.peek()
    }
    fn quicksave(&self) {
        self.save_point.set(self.pos.get());
    }
    fn restore(&self) {
        self.pos.set(self.save_point.get());
    }
    fn peek(&self) -> Option<Token<'src>> {
        self.tokens.borrow().get(self.pos.get() - 1).cloned()
    }
    fn next_literal(&self) -> Option<Token<'src>> {
        match self.next() {
            Some(token @ Token::Literal(_)) => Some(token),
            _ => None
        }
    }
    fn next_identifier(&self) -> Option<Token<'src>> {
        match self.next() {
            Some(token @ Token::Identifier(_)) => Some(token),
            _ => None
        }
    }
    fn next_signature(&self) -> Option<Token<'src>> {
        match self.next() {
            Some(token @ Token::Signature(_, _)) => Some(token),
            _ => None
        }
    }
    pub fn parse(&self) -> Option<ProgramNode> {
        let mut imports = vec![];
        loop {
            self.quicksave();
            if let Some(import) = self.import() {
                imports.push(import);
                continue;
            }
            self.restore();
            break;
        }
        let meta = self.meta()?;
        let mut declarations = vec![];
        while !matches!(self.peek(), Some(Token::EOF)) {
            if let Some(node) = self.declaration() {
                declarations.push(node);
                continue;
            }
            break;
        }
        Some(ProgramNode { imports, meta, declarations })
    }
    fn declaration(&self) -> Option<DeclarationNode> {
        Some(DeclarationNode { staff: self.staff()? })
    }
    fn import(&self) -> Option<ImportDeclarationNode> {
        if !matches!(self.next(), Some(Token::Keyword(Keyword::Import))) {
            return None;
        }
        if !matches!(self.next(), Some(Token::Separator(Separator::LCurly))) {
            return None;
        }
        let mut items = vec![];
        while let Some(item) = self.next_identifier() {
            items.push(item);
            if matches!(self.next(), Some(Token::Separator(Separator::RCurly))) {
                break;
            }
            if !matches!(self.peek(), Some(Token::Separator(Separator::Comma))) {
                return None;
            }
        }
        if !matches!(self.next(), Some(Token::Keyword(Keyword::From))) {
            return None;
        }
        let source = self.next_literal()?;
        Some(ImportDeclarationNode { items, source })
    }
    fn staff(&self) -> Option<StaffDeclarationNode> {
        if !matches!(self.next(), Some(Token::Keyword(Keyword::Staff))) {
            return None;
        }
        let identifier = self.next_identifier()?;
        if !matches!(self.next(), Some(Token::Keyword(Keyword::Is))) {
            return None;
        }
        let staff_type = self.call()?;
        if !matches!(self.next(), Some(Token::Keyword(Keyword::In))) {
            return None;
        }
        let signature = self.next_signature()?;
        if !matches!(self.next(), Some(Token::Separator(Separator::LCurly))) {
            return None;
        }
        let pickup = self.pickup();

        let mut statements = vec![];
        while let Some(statement) = self.staff_statement() {
            statements.push(statement);
        }
        if !matches!(self.peek(), Some(Token::Separator(Separator::RCurly))) {
            return None;
        }
        Some(StaffDeclarationNode {
            identifier,
            staff_type,
            signature,
            pickup,
            statements,
        })
    }
    fn staff_statement(&self) -> Option<StaffStatementNode> {
        self.quicksave();
        if let measure @ Some(_) = self.measure() {
            return Some(StaffStatementNode {
                measure,
                call: None
            })
        }
        self.restore();
        if let call @ Some(_) = self.call() {
            return Some(StaffStatementNode {
                measure: None,
                call
            })
        }
        None
    }
    fn meta(&self) -> Option<MetaDeclarationNode> {
        if !matches!(self.next(), Some(Token::Keyword(Keyword::Meta))) {
            return None;
        }
        if !matches!(self.next(), Some(Token::Separator(Separator::LCurly))) {
            return None;
        }
        let mut configs = vec![];
        while let Some(call) = self.call() {
            configs.push(call);
        }
        if !matches!(self.peek(), Some(Token::Separator(Separator::RCurly))) {
            return None;
        }
        Some(MetaDeclarationNode { configs })
    }
    fn pickup(&self) -> Option<PickupNode> {
        if !matches!(self.next(), Some(Token::Keyword(Keyword::Pickup))) {
            return None;
        }
        let block = self.block()?;
        Some(PickupNode {block})
    }
    fn measure(&self) -> Option<MeasureNode> {
        if !matches!(self.next(), Some(Token::Keyword(Keyword::Measure))) {
            return None;
        }
        let block = self.block()?;
        Some(MeasureNode {block})
    }
    fn call(&self) -> Option<CallNode> {
        let identifier = self.next_identifier()?;
        if !matches!(self.next(), Some(Token::Separator(Separator::LParan))) {
            return None;
        }
        let mut arguments = vec![];
        while let Some(argument) = self.argument() {
            arguments.push(argument);
            if matches!(self.next(), Some(Token::Separator(Separator::RParan))) {
                break;
            }
            if !matches!(self.peek(), Some(Token::Separator(Separator::Comma))) {
                return None;
            }
        }
        Some(CallNode {
            identifier,
            arguments,
        })
    }
    fn call_with(&self) -> Option<CallWithNode> {
        let call = self.call()?;
        let mut with = vec![];
        loop {
            if !matches!(self.next(), Some(Token::Keyword(Keyword::With))) {
                self.prev();
                break;
            }
            self.quicksave();
            if let call @ Some(_) = self.call() {
                with.push(WithNode {call, identifier: None});
                continue;
            }
            self.restore();
            if let identifier @ Some(_) = self.next_identifier() {
                with.push(WithNode {call: None, identifier});
                continue;
            }
            return None;
        }
        Some(CallWithNode {call, with})
    }
    fn argument(&self) -> Option<ArgumentNode> {
        let argument = match self.next() {
            Some(token @ Token::Literal(_)) => Some(token),
            Some(token @ Token::Identifier(_)) => Some(token),
            Some(token @ Token::Note(_, _)) => Some(token),
            Some(token @ Token::Number(_)) => Some(token),
            _ => None
        }?;
        Some(ArgumentNode {
            argument
        })
    }
    fn block(&self) -> Option<BlockNode> {
        if !matches!(self.next(), Some(Token::Separator(Separator::LCurly))) {
            return None;
        }
        let mut calls = vec![];
        while let Some(call) = self.call_with() {
            calls.push(call);
        }
        if !matches!(self.peek(), Some(Token::Separator(Separator::RCurly))) {
            return None;
        }
        Some(BlockNode { calls })
    }
}