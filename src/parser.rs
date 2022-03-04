use std::{cell::{RefCell, Cell}};

use crate::{lexer::Lexer, tokens::{Token, Keyword, Separator, TokenType::*}, nodes::*, errors::{ParseResult, ParseFinalResult}, errors::ParseError::*};


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
    fn next(&self) -> ParseResult<Token<'src>> {
        self.pos.set(self.pos.get() + 1);
        if let Ok(token) = self.peek() {
            return Ok(token);
        }
        let mut tokens = self.tokens.borrow_mut();
        tokens.push(self.lexer.lex().ok_or(Unknown)?);
        tokens.last().cloned().ok_or(Unknown)
    }
    fn prev(&self) -> ParseResult<Token<'src>> {
        self.pos.set(self.pos.get() - 1);
        self.peek()
    }
    fn quicksave(&self) {
        self.save_point.set(self.pos.get());
    }
    fn restore(&self) {
        self.pos.set(self.save_point.get());
    }
    fn peek(&self) -> ParseResult<Token<'src>> {
        self.tokens.borrow().get(self.pos.get() - 1).cloned().ok_or(Unknown)
    }
    fn next_literal(&self) -> ParseResult<Token<'src>> {
        match self.next() {
            Ok(token @ Token::Literal(_, _)) => Ok(token),
            _ => Err(ExpectedType(Literal))
        }
    }
    fn next_identifier(&self) -> ParseResult<Token<'src>> {
        match self.next() {
            Ok(token @ Token::Identifier(_, _)) => Ok(token),
            _ => Err(ExpectedType(Identifier))
        }
    }
    fn next_signature(&self) -> ParseResult<Token<'src>> {
        match self.next() {
            Ok(token @ Token::Signature(_, _, _)) => Ok(token),
            _ => Err(ExpectedType(Signature))
        }
    }
    pub fn parse(&self) -> ParseFinalResult<ProgramNode> {
        match self.parse_inner() {
            Ok(node) => Ok(node),
            Err(err) => Err((err, match self.prev().unwrap() {
                Token::Separator(_, loc) => loc,
                Token::Literal(_, loc) => loc,
                Token::Identifier(_, loc) => loc,
                Token::Signature(_, _, loc) => loc,
                Token::Number(_, loc) => loc,
                Token::Note(_, _, loc) => loc,
                Token::Keyword(_, loc) => loc,
                Token::EOF(loc) => loc
            }))
        }
    }
    fn parse_inner(&self) -> ParseResult<ProgramNode> {
        let mut imports = vec![];
        loop {
            self.quicksave();
            if let Some(import) = self.import() {
                imports.push(import?);
                continue;
            }
            self.restore();
            break;
        }
        let meta = self.meta()?;
        let mut declarations = vec![];
        while !matches!(self.peek(), Ok(Token::EOF(_))) {
            if let Some(node) = self.declaration() {
                declarations.push(node?);
                continue;
            }
            break;
        }
        Ok(ProgramNode { imports, meta, declarations })
    }
    fn declaration(&self) -> Option<ParseResult<DeclarationNode>> {
        match self.staff() {
            None => None,
            Some(Ok(staff)) => Some(Ok(DeclarationNode { staff })),
            Some(Err(err)) => Some(Err(err))
        }
    }
    fn import(&self) -> Option<ParseResult<ImportDeclarationNode>> {
        if !matches!(self.next(), Ok(Token::Keyword(Keyword::Import, _))) {
            return None;
        }
        if !matches!(self.next(), Ok(Token::Separator(Separator::LCurly, _))) {
            return Some(Err(ExpectedSeparator(Separator::LCurly)));
        }
        let mut items = vec![];
        while let Ok(item) = self.next_identifier() {
            items.push(item);
            if matches!(self.next(), Ok(Token::Separator(Separator::RCurly, _))) {
                break;
            }
            if !matches!(self.peek(), Ok(Token::Separator(Separator::Comma, _))) {
                return Some(Err(ExpectedSeparator(Separator::Comma)));
            }
        }
        if !matches!(self.next(), Ok(Token::Keyword(Keyword::From, _))) {
            return Some(Err(ExpectedKeyword(Keyword::From)));
        }
        Some(match self.next_literal() {
            Ok(source) => Ok(ImportDeclarationNode { items, source }),
            Err(err) => Err(err)
        })
    }
    fn staff(&self) -> Option<ParseResult<StaffDeclarationNode>> {
        if !matches!(self.next(), Ok(Token::Keyword(Keyword::Staff, _))) {
            return None;
        }
        Some(self.staff_inner())
    }
    fn staff_inner(&self) -> ParseResult<StaffDeclarationNode> {
        let identifier = self.next_identifier()?;
        if !matches!(self.next(), Ok(Token::Keyword(Keyword::Is, _))) {
            return Err(ExpectedKeyword(Keyword::Is));
        }
        let staff_type = self.call().ok_or(ExpectedType(Identifier))??;
        if !matches!(self.next(), Ok(Token::Keyword(Keyword::In, _))) {
            return Err(ExpectedKeyword(Keyword::In));
        }
        let signature = self.next_signature()?;
        if !matches!(self.next(), Ok(Token::Separator(Separator::LCurly, _))) {
            return Err(ExpectedSeparator(Separator::LCurly));
        }
        let pickup = match self.pickup() {
            Some(node) => Some(node?),
            None => None
        };

        let mut statements = vec![];
        while let Some(statement) = self.staff_statement() {
            statements.push(statement?);
        }
        if !matches!(self.peek(), Ok(Token::Separator(Separator::RCurly, _))) {
            return Err(ExpectedSeparator(Separator::RCurly));
        }
        Ok(StaffDeclarationNode {
            identifier,
            staff_type,
            signature,
            pickup,
            statements,
        })
    }
    fn staff_statement(&self) -> Option<ParseResult<StaffStatementNode>> {
        self.quicksave();
        if let Some(measure) = self.measure() {
            let measure = match measure {
                Ok(measure) => measure,
                Err(err) => return Some(Err(err))
            };
            return Some(Ok(StaffStatementNode {
                measure: Some(measure),
                call: None
            }))
        }
        self.restore();
        match self.call() {
            None => None,
            Some(Ok(call)) => Some(Ok(StaffStatementNode {
                measure: None,
                call: Some(call)
            })),
            Some(Err(err)) => Some(Err(err))
        }
    }
    fn meta(&self) -> ParseResult<MetaDeclarationNode> {
        if !matches!(self.next(), Ok(Token::Keyword(Keyword::Meta, _))) {
            return Err(ExpectedKeyword(Keyword::Meta));
        }
        if !matches!(self.next(), Ok(Token::Separator(Separator::LCurly, _))) {
            return Err(ExpectedSeparator(Separator::LCurly));
        }
        let mut configs = vec![];
        while let Ok(call) = self.call().ok_or(ExpectedType(Identifier)) {
            configs.push(call?);
        }
        if configs.is_empty() {
            return Err(EmptyMeta);
        }
        if !matches!(self.peek(), Ok(Token::Separator(Separator::RCurly, _))) {
            return Err(ExpectedSeparator(Separator::RCurly));
        }
        Ok(MetaDeclarationNode { configs })
    }
    fn pickup(&self) -> Option<ParseResult<PickupNode>> {
        if !matches!(self.next(), Ok(Token::Keyword(Keyword::Pickup, _))) {
            return None;
        }
        Some(match self.block() {
            Ok(block) => Ok(PickupNode {block}),
            Err(err) => Err(err)
        })
    }
    fn measure(&self) -> Option<ParseResult<MeasureNode>> {
        if !matches!(self.next(), Ok(Token::Keyword(Keyword::Measure, _))) {
            return None;
        }
        let block = match self.block() {
            Ok(block) => block,
            Err(err) => return Some(Err(err))
        };
        Some(Ok(MeasureNode {block}))
    }
    fn call(&self) -> Option<ParseResult<CallNode>> {
        let identifier = match self.next_identifier() {
            Ok(identifier) => identifier,
            Err(_) => return None
        };
        if !matches!(self.next(), Ok(Token::Separator(Separator::LParan, _))) {
            return None;
        }
        let mut arguments = vec![];
        loop {
            let argument = match self.argument() {
                Ok(argument) => argument,
                Err(err) => return Some(Err(err))
            };
            arguments.push(argument);
            if matches!(self.next(), Ok(Token::Separator(Separator::RParan, _))) {
                break;
            }
            if !matches!(self.peek(), Ok(Token::Separator(Separator::Comma, _))) {
                return Some(Err(ExpectedSeparator(Separator::Comma)));
            }
        }
        Some(Ok(CallNode {
            identifier,
            arguments,
        }))
    }
    fn call_with(&self) -> Option<ParseResult<CallWithNode>> {
        let call = match self.call()? {
            Ok(call) => call,
            Err(_) => return None
        };
        let mut with = vec![];
        loop {
            if !matches!(self.next(), Ok(Token::Keyword(Keyword::With, _))) {
                match self.prev() {
                    Ok(_) => break,
                    Err(err) => return Some(Err(err))
                };
            }
            self.quicksave();
            if let Some(call) = self.call() {
                let call = Some(match call {
                    Ok(call) => call,
                    Err(err) => return Some(Err(err))
                });
                with.push(WithNode {call, identifier: None});
                continue;
            }
            self.restore();
            let identifier = Some(match self.next_identifier() {
                Ok(identifier) => identifier,
                Err(err) => return Some(Err(err))
            });
            with.push(WithNode {call: None, identifier});
        }
        Some(Ok(CallWithNode {call, with}))
    }
    fn argument(&self) -> ParseResult<ArgumentNode> {
        let argument = match self.next() {
            Ok(token @ Token::Literal(_, _)) => Ok(token),
            Ok(token @ Token::Identifier(_, _)) => Ok(token),
            Ok(token @ Token::Note(_, _, _)) => Ok(token),
            Ok(token @ Token::Number(_, _)) => Ok(token),
            _ => Err(ExpectedArgument)
        }?;
        Ok(ArgumentNode {
            argument
        })
    }
    fn block(&self) -> ParseResult<BlockNode> {
        if !matches!(self.next(), Ok(Token::Separator(Separator::LCurly, _))) {
            return Err(ExpectedSeparator(Separator::LCurly));
        }
        let mut calls = vec![];
        while let Some(call) = self.call_with() {
            calls.push(call?);
        }
        if !matches!(self.peek(), Ok(Token::Separator(Separator::RCurly, _))) {
            return Err(ExpectedSeparator(Separator::RCurly));
        }
        Ok(BlockNode { calls })
    }
}