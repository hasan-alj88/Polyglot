//! Visitor trait for AST traversal
//!
//! Enables multiple analysis passes without modifying AST nodes.
//!
//! # Design
//!
//! The Visitor pattern allows you to define operations on AST nodes
//! without changing the node definitions themselves. This is useful for:
//! - IR generation
//! - Type checking
//! - Code analysis
//! - Pretty printing
//! - Optimization passes
//!
//! # Example
//!
//! ```
//! use polyglot_parser::visitor::Visitor;
//! use polyglot_parser::ast::*;
//!
//! struct CountVisitor {
//!     count: usize,
//! }
//!
//! impl Visitor for CountVisitor {
//!     fn visit_pipeline(&mut self, pipeline: &Pipeline) {
//!         self.count += 1;
//!         // Continue traversal
//!         self.visit_block(&pipeline.body);
//!     }
//!
//!     fn visit_statement(&mut self, stmt: &Statement) {
//!         self.count += 1;
//!     }
//! }
//! ```

use crate::ast::*;

/// Visitor trait for AST traversal
///
/// Implement this trait to traverse and analyze the AST.
/// The default implementations perform depth-first traversal.
pub trait Visitor: Sized {
    /// Visit a pipeline
    fn visit_pipeline(&mut self, pipeline: &Pipeline) {
        walk_pipeline(self, pipeline);
    }

    /// Visit a block
    fn visit_block(&mut self, block: &Block) {
        walk_block(self, block);
    }

    /// Visit a statement
    fn visit_statement(&mut self, stmt: &Statement) {
        walk_statement(self, stmt);
    }

    /// Visit an expression
    fn visit_expression(&mut self, expr: &Expression) {
        walk_expression(self, expr);
    }

    /// Visit an identifier
    fn visit_identifier(&mut self, _ident: &Identifier) {
        // Leaf node - no traversal needed
    }

    /// Visit a literal
    fn visit_literal(&mut self, lit: &Literal) {
        walk_literal(self, lit);
    }

    /// Visit a type annotation
    fn visit_type_annotation(&mut self, ty: &TypeAnnotation) {
        walk_type_annotation(self, ty);
    }

    /// Visit an input parameter
    fn visit_input_parameter(&mut self, param: &InputParameter) {
        walk_input_parameter(self, param);
    }

    /// Visit a pattern
    fn visit_pattern(&mut self, _pattern: &Pattern) {
        // Most patterns are leaf nodes
    }

    /// Visit a range expression
    fn visit_range_expression(&mut self, range: &RangeExpression) {
        walk_range_expression(self, range);
    }

    /// Visit trigger configuration
    fn visit_trigger_config(&mut self, _config: &TriggerConfig) {
        // Configuration node - optional override
    }

    /// Visit queue configuration
    fn visit_queue_config(&mut self, _config: &QueueConfig) {
        // Configuration node - optional override
    }

    /// Visit wrapper configuration
    fn visit_wrapper_config(&mut self, _config: &WrapperConfig) {
        // Configuration node - optional override
    }
}

// ============================================================================
// Walk Functions (Default Traversal Implementation)
// ============================================================================

/// Walk a pipeline (depth-first traversal)
pub fn walk_pipeline<V: Visitor>(visitor: &mut V, pipeline: &Pipeline) {
    // Visit input parameters
    if let Some(inputs) = &pipeline.input {
        for param in inputs {
            visitor.visit_input_parameter(param);
        }
    }

    // Visit output type
    if let Some(output) = &pipeline.output {
        visitor.visit_type_annotation(output);
    }

    // Visit configurations
    if let Some(trigger) = &pipeline.trigger {
        visitor.visit_trigger_config(trigger);
    }
    if let Some(queue) = &pipeline.queue {
        visitor.visit_queue_config(queue);
    }
    if let Some(wrapper) = &pipeline.wrapper {
        visitor.visit_wrapper_config(wrapper);
    }

    // Visit body
    visitor.visit_block(&pipeline.body);
}

/// Walk a block (depth-first traversal)
pub fn walk_block<V: Visitor>(visitor: &mut V, block: &Block) {
    for stmt in &block.statements {
        visitor.visit_statement(stmt);
    }
}

/// Walk a statement (depth-first traversal)
pub fn walk_statement<V: Visitor>(visitor: &mut V, stmt: &Statement) {
    match stmt {
        Statement::VariableDeclaration {
            name,
            type_annotation,
            init,
            ..
        } => {
            visitor.visit_identifier(name);
            if let Some(ty) = type_annotation {
                visitor.visit_type_annotation(ty);
            }
            if let Some(expr) = init {
                visitor.visit_expression(expr);
            }
        }
        Statement::Assignment { target, value, .. } => {
            visitor.visit_identifier(target);
            visitor.visit_expression(value);
        }
        Statement::PipelineCall { pipeline, args, .. } => {
            visitor.visit_identifier(pipeline);
            for arg in args {
                visitor.visit_expression(arg);
            }
        }
        Statement::Return { value, .. } => {
            if let Some(expr) = value {
                visitor.visit_expression(expr);
            }
        }
        Statement::Block { block, .. } => {
            visitor.visit_block(block);
        }
        Statement::Conditional {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            visitor.visit_expression(condition);
            visitor.visit_block(then_branch);
            if let Some(else_block) = else_branch {
                visitor.visit_block(else_block);
            }
        }
        Statement::ErrorCatch {
            error_type,
            handler,
            ..
        } => {
            visitor.visit_identifier(error_type);
            visitor.visit_block(handler);
        }
    }
}

/// Walk an expression (depth-first traversal)
pub fn walk_expression<V: Visitor>(visitor: &mut V, expr: &Expression) {
    match expr {
        Expression::Literal { value, .. } => {
            visitor.visit_literal(value);
        }
        Expression::Identifier { name, .. } => {
            visitor.visit_identifier(name);
        }
        Expression::PipelineCall { pipeline, args, .. } => {
            visitor.visit_identifier(pipeline);
            for arg in args {
                visitor.visit_expression(arg);
            }
        }
        Expression::BinaryOp { left, right, .. } => {
            visitor.visit_expression(left);
            visitor.visit_expression(right);
        }
        Expression::UnaryOp { operand, .. } => {
            visitor.visit_expression(operand);
        }
        Expression::RangeCheck { value, range, .. } => {
            visitor.visit_expression(value);
            visitor.visit_range_expression(range);
        }
        Expression::PatternMatch { value, pattern, .. } => {
            visitor.visit_expression(value);
            visitor.visit_pattern(pattern);
        }
        Expression::Unpack { value, .. } => {
            visitor.visit_expression(value);
        }
    }
}

/// Walk a literal (depth-first traversal)
pub fn walk_literal<V: Visitor>(visitor: &mut V, lit: &Literal) {
    match lit {
        Literal::Collection(exprs) => {
            for expr in exprs {
                visitor.visit_expression(expr);
            }
        }
        _ => {
            // Other literals are leaf nodes
        }
    }
}

/// Walk a type annotation (depth-first traversal)
pub fn walk_type_annotation<V: Visitor>(visitor: &mut V, ty: &TypeAnnotation) {
    match ty {
        TypeAnnotation::Collection(inner) => {
            visitor.visit_type_annotation(inner);
        }
        TypeAnnotation::Optional(inner) => {
            visitor.visit_type_annotation(inner);
        }
        _ => {
            // Other type annotations are leaf nodes
        }
    }
}

/// Walk an input parameter (depth-first traversal)
pub fn walk_input_parameter<V: Visitor>(visitor: &mut V, param: &InputParameter) {
    if let Some(ty) = &param.type_annotation {
        visitor.visit_type_annotation(ty);
    }
    if let Some(default) = &param.default {
        visitor.visit_expression(default);
    }
}

/// Walk a range expression (depth-first traversal)
pub fn walk_range_expression<V: Visitor>(visitor: &mut V, range: &RangeExpression) {
    visitor.visit_expression(&range.start);
    visitor.visit_expression(&range.end);
}

// ============================================================================
// Example Visitor: PrintVisitor (for debugging)
// ============================================================================

/// Example visitor that prints the AST structure
///
/// Useful for debugging and understanding the AST.
#[derive(Debug)]
pub struct PrintVisitor {
    indent: usize,
}

impl PrintVisitor {
    pub fn new() -> Self {
        Self { indent: 0 }
    }

    fn print_line(&self, msg: &str) {
        println!("{}{}", "  ".repeat(self.indent), msg);
    }

    fn indent(&mut self) {
        self.indent += 1;
    }

    fn dedent(&mut self) {
        self.indent = self.indent.saturating_sub(1);
    }
}

impl Default for PrintVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for PrintVisitor {
    fn visit_pipeline(&mut self, pipeline: &Pipeline) {
        self.print_line(&format!("Pipeline: {}", pipeline.name));
        self.indent();
        walk_pipeline(self, pipeline);
        self.dedent();
    }

    fn visit_block(&mut self, block: &Block) {
        self.print_line(&format!("Block: {:?}", block.block_type));
        self.indent();
        walk_block(self, block);
        self.dedent();
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VariableDeclaration { name, .. } => {
                self.print_line(&format!("VarDecl: {:?}", name));
            }
            Statement::Assignment { target, .. } => {
                self.print_line(&format!("Assignment: {:?}", target));
            }
            Statement::PipelineCall { pipeline, .. } => {
                self.print_line(&format!("PipelineCall: {:?}", pipeline));
            }
            Statement::Return { .. } => {
                self.print_line("Return");
            }
            Statement::Block { .. } => {
                self.print_line("NestedBlock");
            }
            Statement::Conditional { .. } => {
                self.print_line("Conditional");
            }
            Statement::ErrorCatch { .. } => {
                self.print_line("ErrorCatch");
            }
        }
        self.indent();
        walk_statement(self, stmt);
        self.dedent();
    }

    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Literal { value, .. } => {
                self.print_line(&format!("Literal: {:?}", value));
            }
            Expression::Identifier { name, .. } => {
                self.print_line(&format!("Identifier: {:?}", name));
            }
            Expression::BinaryOp { op, .. } => {
                self.print_line(&format!("BinaryOp: {:?}", op));
            }
            Expression::UnaryOp { op, .. } => {
                self.print_line(&format!("UnaryOp: {:?}", op));
            }
            _ => {
                self.print_line("Expression");
            }
        }
        self.indent();
        walk_expression(self, expr);
        self.dedent();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::{Position, Span};

    struct CountVisitor {
        pipeline_count: usize,
        block_count: usize,
        statement_count: usize,
        expression_count: usize,
    }

    impl CountVisitor {
        fn new() -> Self {
            Self {
                pipeline_count: 0,
                block_count: 0,
                statement_count: 0,
                expression_count: 0,
            }
        }
    }

    impl Visitor for CountVisitor {
        fn visit_pipeline(&mut self, pipeline: &Pipeline) {
            self.pipeline_count += 1;
            walk_pipeline(self, pipeline);
        }

        fn visit_block(&mut self, block: &Block) {
            self.block_count += 1;
            walk_block(self, block);
        }

        fn visit_statement(&mut self, stmt: &Statement) {
            self.statement_count += 1;
            walk_statement(self, stmt);
        }

        fn visit_expression(&mut self, expr: &Expression) {
            self.expression_count += 1;
            walk_expression(self, expr);
        }
    }

    #[test]
    fn test_count_visitor_basic() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 10, 9));

        let pipeline = Pipeline {
            name: "Test".to_string(),
            input: None,
            output: None,
            trigger: None,
            queue: None,
            wrapper: None,
            body: Block {
                block_type: BlockType::Sequential,
                statements: vec![Statement::Return { value: None, span }],
                span,
            },
            span,
        };

        let mut visitor = CountVisitor::new();
        visitor.visit_pipeline(&pipeline);

        assert_eq!(visitor.pipeline_count, 1);
        assert_eq!(visitor.block_count, 1);
        assert_eq!(visitor.statement_count, 1);
    }

    #[test]
    fn test_print_visitor_basic() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 10, 9));

        let pipeline = Pipeline {
            name: "Test".to_string(),
            input: None,
            output: None,
            trigger: None,
            queue: None,
            wrapper: None,
            body: Block {
                block_type: BlockType::Sequential,
                statements: vec![],
                span,
            },
            span,
        };

        let mut visitor = PrintVisitor::new();
        visitor.visit_pipeline(&pipeline);
        // Just verify it doesn't panic
    }
}
