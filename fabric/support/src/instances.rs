// This file is part of Tetcore.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Some instance placeholder to be used in [`fabric_support::noble`] attribute macro.
//!
//! [`fabric_support::noble`] attribute macro does only requires the instance generic `I` to be
//! static (contrary to `decl_*` macro which requires instance generic to implement
//! [`fabric_support::traits::Instance`]).
//!
//! Thus support provides some instance types to be used, This allow some instantiable noble to
//! depend on specific instance of another:
//! ```
//! # mod another_noble { pub trait Config<I: 'static = ()> {} }
//! pub trait Config<I: 'static = ()>: another_noble::Config<I> {}
//! ```
//!
//! NOTE: [`fabric_support::noble`] will reexport them inside the module, in order to make them
//! accessible to [`fabric_support::construct_runtime`].

/// Instance0 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance0;

/// Instance1 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance1;

/// Instance2 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance2;

/// Instance3 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance3;

/// Instance4 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance4;

/// Instance5 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance5;

/// Instance6 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance6;

/// Instance7 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance7;

/// Instance8 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance8;

/// Instance9 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance9;

/// Instance10 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance10;

/// Instance11 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance11;

/// Instance12 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance12;

/// Instance13 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance13;

/// Instance14 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance14;

/// Instance15 to be used for instantiable noble define with `noble` macro.
#[derive(Clone, Copy, PartialEq, Eq, crate::RuntimeDebugNoBound)]
pub struct Instance15;
