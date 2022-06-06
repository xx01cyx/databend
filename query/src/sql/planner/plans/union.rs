// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::sql::optimizer::{PhysicalProperty, RelationalProperty, RelExpr, SExpr};
use crate::sql::plans::LogicalPlan;
use crate::sql::plans::Operator;
use crate::sql::plans::PhysicalPlan;
use crate::sql::plans::RelOp;

#[derive(Clone, Debug)]
pub struct UnionPlan {}

impl Operator for UnionPlan {
    fn plan_type(&self) -> RelOp {
        RelOp::Union
    }

    fn is_physical(&self) -> bool {
        true
    }

    fn is_logical(&self) -> bool {
        true
    }

    fn as_logical(&self) -> Option<&dyn LogicalPlan> {
        Some(self)
    }

    fn as_physical(&self) -> Option<&dyn PhysicalPlan> {
        Some(self)
    }
}

impl LogicalPlan for UnionPlan {
    fn derive_relational_prop<'a>(&self, rel_expr: &RelExpr<'a>) -> common_exception::Result<RelationalProperty> {
        todo!()
    }
}

impl PhysicalPlan for UnionPlan {
    fn compute_physical_prop(&self, expression: &SExpr) -> PhysicalProperty {
        todo!()
    }
}
