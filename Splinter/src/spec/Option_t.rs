// Copyright 2018-2024 VMware, Inc., Microsoft Inc., Carnegie Mellon University, ETH Zurich, University of Washington
// SPDX-License-Identifier: BSD-2-Clause
#[allow(unused_imports)]
use builtin::*;
use builtin_macros::*;

verus! {
broadcast use{vstd::seq_lib::group_seq_properties,
              vstd::map_lib::group_map_properties,
              vstd::set_lib::group_set_properties,
              vstd::multiset::group_multiset_properties};

pub enum Option<T> {
    None,
    Some(T),
}

} // verus!
