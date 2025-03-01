// Copyright 2018-2024 VMware, Inc., Microsoft Inc., Carnegie Mellon University, ETH Zurich, University of Washington
// SPDX-License-Identifier: BSD-2-Clause

//! This file contains types relating to generic disk addressing and referencing.

#[allow(unused_imports)]
use builtin::*;
use builtin_macros::*;
use vstd::prelude::*;

use crate::spec::AsyncDisk_t;
use crate::spec::ImplDisk_t;

verus!{

/// Exporting from trusted disk model
pub type AU = AsyncDisk_t::AU;
pub type Page = AsyncDisk_t::Page;
pub type Address = AsyncDisk_t::Address;

pub type IAU = ImplDisk_t::IAU;
pub type IPage = ImplDisk_t::IPage;
pub type IAddress = ImplDisk_t::IAddress;

pub open spec(checked) fn page_count() -> Page
{
    AsyncDisk_t::page_count()
}

pub open spec(checked) fn au_count() -> AU
{
    AsyncDisk_t::au_count()
}

impl Address {
    /// Returns the Address for the first page of this AU.
    pub open spec(checked) fn first_page(self) -> Address {
        Address{page: 0, ..self}
    }

    /// Returns the previous Address in this AU (may not be well-formed).
    pub open spec(checked) fn previous(self) -> Address {
        Address{page: (self.page-1) as nat, ..self}
    }
    
    /// Returns the next Address in this AU (may not be well-formed).
    pub open spec(checked) fn next(self) -> Address {
        Address{page: self.page+1, ..self}
    }

    // Returns true for WF addresses within the same AU but after self
    pub open spec(checked) fn after_page(self, addr: Self) -> bool {
        &&& addr.wf()
        &&& addr.au == self.au
        &&& addr.page > self.page
    }
}

pub open spec(checked) fn first_page(ptr: Pointer) -> Pointer {
    match ptr {
        None => None,
        Some(addr) => Some(addr.first_page()),
    }
}

/// Return the lowest of two addresses. Addresses are first compared by AU,
/// then by Page index (if AUs match).
pub open spec(checked) fn min_addr(a: Address, b: Address) -> Address {
    if a.au < b.au { a }
    else if a.au > b.au { b }
    else if a.page <= b.page { a }
    else { b }
}

/// Returns the set of AUs that the provided set of Addresses live in.
pub open spec(checked) fn to_aus(addrs: Set<Address>) -> Set<AU> {
    addrs.map(|addr: Address| addr.au)
}

pub open spec(checked) fn addr_range(au: IAU, start: IPage, end_excl: IPage) -> (addrs: Set<Address>) {
    Set::new(|addr: Address| {
        &&& addr.au == au 
        &&& addr.page >= start 
        &&& addr.page < end_excl
    })
}

impl View for IAddress {
    type V = Address;

    open spec fn view(&self) -> Self::V
    {
        Address{au: self.au as nat, page: self.page as nat}
    }
}

/// A Pointer is either an Address or None. (i.e.: we wrap Address with the semantics for
/// "NULL" pointers). Used when certain data structures might have unallocated pointers.
pub type Pointer = Option<Address>;
pub type Ranking = Map<Address, nat>;   // Used by Linked* layers to show acyclicity.

}
