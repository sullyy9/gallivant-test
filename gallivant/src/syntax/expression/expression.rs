use std::{borrow::Borrow, ops::Range};

use super::kind::ExprKind;

////////////////////////////////////////////////////////////////
// types
////////////////////////////////////////////////////////////////

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    String(String),
    UInt(u32),

    ScriptComment(String),

    HPMode,
    Comment(Box<ParsedExpr>),
    Wait(Box<ParsedExpr>),
    OpenDialog(Box<ParsedExpr>),
    WaitDialog(Box<ParsedExpr>),
    Flush,
    Protocol,
    Print(Vec<ParsedExpr>),
    SetTimeFormat(Box<ParsedExpr>),

    /// This requires getting the current time from the OS and sending it to the printer via the
    /// TCU. Need to consider that the time must be acquired just before the command is sent.
    SetTime,
    SetOption {
        option: Box<ParsedExpr>,
        setting: Box<ParsedExpr>,
    },
    TCUClose(Box<ParsedExpr>),
    TCUOpen(Box<ParsedExpr>),
    TCUTest {
        channel: Box<ParsedExpr>,
        min: Box<ParsedExpr>,
        max: Box<ParsedExpr>,
        retries: Box<ParsedExpr>,
        message: Box<ParsedExpr>,
    },
    PrinterSet(Box<ParsedExpr>),
    PrinterTest {
        channel: Box<ParsedExpr>,
        min: Box<ParsedExpr>,
        max: Box<ParsedExpr>,
        retries: Box<ParsedExpr>,
        message: Box<ParsedExpr>,
    },
    IssueTest(Box<ParsedExpr>), // Unused.
    TestResult {
        // Unused.
        min: Box<ParsedExpr>,
        max: Box<ParsedExpr>,
        message: Box<ParsedExpr>,
    },
    USBOpen,
    USBClose,
    USBPrint(Vec<ParsedExpr>),
    USBSetTimeFormat(Box<ParsedExpr>),
    USBSetTime,
    USBSetOption {
        option: Box<ParsedExpr>,
        setting: Box<ParsedExpr>,
    },
    USBPrinterSet(Box<ParsedExpr>),
    USBPrinterTest {
        channel: Box<ParsedExpr>,
        min: Box<ParsedExpr>,
        max: Box<ParsedExpr>,
        retries: Box<ParsedExpr>,
        message: Box<ParsedExpr>,
    },
}

////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct ParsedExpr {
    expr: Expr,
    span: Range<usize>,
}

////////////////////////////////////////////////////////////////
// construction / conversion
////////////////////////////////////////////////////////////////

impl ParsedExpr {
    pub fn from_kind_and_span(expr: Expr, span: Range<usize>) -> Self {
        Self { expr, span }
    }

    /// Return a new Expr from the given ExprKind and with a default span. Primariliy intended for
    /// use in testing.
    ///
    pub fn from_kind_default(expr: Expr) -> Self {
        Self {
            expr,
            span: Range::default(),
        }
    }

    /// Return a new String kind Expr with a default span. Primariliy intended for use in testing.
    ///
    pub fn from_str_default(string: &str) -> Self {
        Self {
            expr: Expr::String(string.to_string()),
            span: Range::default(),
        }
    }

    /// Return a new Uint kind Expr with a default span. Primariliy intended for use in testing.
    ///
    pub fn from_uint_default(uint: u32) -> Self {
        Self {
            expr: Expr::UInt(uint),
            span: Range::default(),
        }
    }
}

////////////////////////////////////////////////////////////////

impl<T: Borrow<Expr>> From<T> for ExprKind {
    fn from(expr: T) -> Self {
        match expr.borrow() {
            Expr::String(_) => ExprKind::String,
            Expr::UInt(_) => ExprKind::UInt,
            Expr::ScriptComment(_) => ExprKind::ScriptComment,
            Expr::HPMode => ExprKind::HPMode,
            Expr::Comment(_) => ExprKind::Comment,
            Expr::Wait(_) => ExprKind::Wait,
            Expr::OpenDialog(_) => ExprKind::OpenDialog,
            Expr::WaitDialog(_) => ExprKind::WaitDialog,
            Expr::Flush => ExprKind::Flush,
            Expr::Protocol => ExprKind::Protocol,
            Expr::Print(_) => ExprKind::Print,
            Expr::SetTimeFormat(_) => ExprKind::SetTimeFormat,
            Expr::SetTime => ExprKind::SetTime,
            Expr::SetOption { .. } => ExprKind::SetOption,
            Expr::TCUClose(_) => ExprKind::TCUClose,
            Expr::TCUOpen(_) => ExprKind::TCUOpen,
            Expr::TCUTest { .. } => ExprKind::TCUTest,
            Expr::PrinterSet(_) => ExprKind::PrinterSet,
            Expr::PrinterTest { .. } => ExprKind::PrinterTest,
            Expr::IssueTest(_) => ExprKind::IssueTest,
            Expr::TestResult { .. } => ExprKind::TestResult,
            Expr::USBOpen => ExprKind::USBOpen,
            Expr::USBClose => ExprKind::USBClose,
            Expr::USBPrint(_) => ExprKind::USBPrint,
            Expr::USBSetTimeFormat(_) => ExprKind::USBSetTimeFormat,
            Expr::USBSetTime => ExprKind::USBSetTime,
            Expr::USBSetOption { .. } => ExprKind::USBSetOption,
            Expr::USBPrinterSet(_) => ExprKind::USBPrinterSet,
            Expr::USBPrinterTest { .. } => ExprKind::USBPrinterTest,
        }
    }
}

////////////////////////////////////////////////////////////////

#[cfg(test)]
impl From<Expr> for ParsedExpr {
    fn from(expr: Expr) -> Self {
        ParsedExpr {
            expr,
            span: Range::default(),
        }
    }
}

#[cfg(test)]
impl From<Expr> for Box<ParsedExpr> {
    fn from(expr: Expr) -> Self {
        Box::new(ParsedExpr {
            expr,
            span: Range::default(),
        })
    }
}

////////////////////////////////////////////////////////////////
// field access
////////////////////////////////////////////////////////////////

impl ParsedExpr {
    pub fn expression(&self) -> &Expr {
        &self.expr
    }

    pub fn expression_kind(&self) -> ExprKind {
        ExprKind::from(&self.expr)
    }

    pub fn span(&self) -> &Range<usize> {
        &self.span
    }
}

////////////////////////////////////////////////////////////////
// comparison
////////////////////////////////////////////////////////////////

impl std::cmp::PartialEq for ParsedExpr {
    fn eq(&self, other: &Self) -> bool {
        // Only compare the expression kind. Makes testing much easier.
        self.expr == other.expr
    }
}

////////////////////////////////////////////////////////////////
