// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_ast::{IntegerType, Type};
use leo_errors::{emitter::Handler, TypeCheckerError};
use leo_span::{Span, Symbol};

use crate::SymbolTable;

pub struct TypeChecker<'a> {
    pub(crate) symbol_table: &'a mut SymbolTable<'a>,
    pub(crate) handler: &'a Handler,
    pub(crate) parent: Option<Symbol>,
    pub(crate) negate: bool,
    pub(crate) expected_type: Option<Type>,
    pub(crate) span: Span,
}

const INT_TYPES: [Type; 10] = [
    Type::IntegerType(IntegerType::I8),
    Type::IntegerType(IntegerType::I16),
    Type::IntegerType(IntegerType::I32),
    Type::IntegerType(IntegerType::I64),
    Type::IntegerType(IntegerType::I128),
    Type::IntegerType(IntegerType::U8),
    Type::IntegerType(IntegerType::U16),
    Type::IntegerType(IntegerType::U32),
    Type::IntegerType(IntegerType::U64),
    Type::IntegerType(IntegerType::U128),
];

const fn create_type_superset<const S: usize, const A: usize, const O: usize>(
    subset: [Type; S],
    additional: [Type; A],
) -> [Type; O] {
    let mut superset: [Type; O] = [Type::IntegerType(IntegerType::U8); O];
    let mut i = 0;
    while i < S {
        superset[i] = subset[i];
        i += 1;
    }
    let mut j = 0;
    while j < A {
        superset[i + j] = additional[j];
        j += 1;
    }
    superset
}

const FIELD_INT_TYPES: [Type; 11] = create_type_superset(INT_TYPES, [Type::Field]);

const FIELD_SCALAR_INT_TYPES: [Type; 12] = create_type_superset(FIELD_INT_TYPES, [Type::Scalar]);

const FIELD_GROUP_INT_TYPES: [Type; 12] = create_type_superset(FIELD_INT_TYPES, [Type::Group]);

const FIELD_GROUP_SCALAR_INT_TYPES: [Type; 13] = create_type_superset(FIELD_GROUP_INT_TYPES, [Type::Scalar]);

impl<'a> TypeChecker<'a> {
    /// Returns a new type checker given a symbol table and error handler.
    pub fn new(symbol_table: &'a mut SymbolTable<'a>, handler: &'a Handler) -> Self {
        Self {
            symbol_table,
            handler,
            parent: None,
            negate: false,
            expected_type: None,
            span: Default::default(),
        }
    }

    // Checks wether two given types are the same and if not emits an error.
    pub(crate) fn assert_eq_types(&self, t1: Option<Type>, t2: Option<Type>, span: Span) {
        match (t1, t2) {
            (Some(t1), Some(t2)) if t1 != t2 => self
                .handler
                .emit_err(TypeCheckerError::type_should_be(t1, t2, span).into()),
            (Some(type_), None) | (None, Some(type_)) => self
                .handler
                .emit_err(TypeCheckerError::type_should_be("no type", type_, span).into()),
            _ => {}
        }
    }

    /// Returns the given type if it equals the expected type or the expected type is none.
    pub(crate) fn assert_type(&mut self, type_: Type, expected: Option<Type>) -> Type {
        if let Some(expected) = expected {
            if type_ != expected {
                self.handler
                    .emit_err(TypeCheckerError::type_should_be(type_, expected, self.span).into());
            }
        }

        type_
    }

    /// Emits an error to the error handler if the given type is not equal to any of the expected types.
    pub(crate) fn assert_one_of_types(&self, type_: Option<Type>, expected: &[Type], span: Span) {
        if let Some(type_) = type_ {
            if !expected.iter().any(|t: &Type| t == &type_) {
                self.handler.emit_err(
                    TypeCheckerError::expected_one_type_of(
                        expected.iter().map(|t| t.to_string() + ",").collect::<String>(),
                        type_,
                        span,
                    )
                    .into(),
                );
            }
        }
    }

    /// Emits an error to the handler if the given type is not a field or integer.
    pub(crate) fn assert_field_int_type(&self, type_: Option<Type>, span: Span) {
        self.assert_one_of_types(type_, &FIELD_INT_TYPES, span)
    }

    /// Emits an error to the handler if the given type is not a field, scalar, or integer.
    pub(crate) fn assert_field_scalar_int_type(&self, type_: Option<Type>, span: Span) {
        self.assert_one_of_types(type_, &FIELD_SCALAR_INT_TYPES, span)
    }

    /// Emits an error to the handler if the given type is not a field, group, or integer.
    pub(crate) fn assert_field_group_int_type(&self, type_: Option<Type>, span: Span) {
        self.assert_one_of_types(type_, &FIELD_GROUP_INT_TYPES, span)
    }

    /// Emits an error to the handler if the given type is not a field, group, scalar or integer.
    pub(crate) fn assert_field_group_scalar_int_type(&self, type_: Option<Type>, span: Span) {
        self.assert_one_of_types(type_, &FIELD_GROUP_SCALAR_INT_TYPES, span)
    }
}
