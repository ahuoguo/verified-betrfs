// Copyright 2018-2024 VMware, Inc., Microsoft Inc., Carnegie Mellon University, ETH Zurich, University of Washington
// SPDX-License-Identifier: BSD-2-Clause
verus!{
broadcast use{vstd::seq_lib::group_seq_properties,
              vstd::map_lib::group_map_properties,
              vstd::set_lib::group_set_properties,
              vstd::multiset::group_multiset_properties};
    // delayed response refines to the one that write writes as it is permitted
    // received requests & crash before taking the transition

    // iteration
    state_machine

    struct Real<T>{

    }

    impl<T> Real{

    }

}