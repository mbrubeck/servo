/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Implements parallel traversals over the DOM and flow trees.
//!
//! This code is highly unsafe. Keep this file small and easy to audit.

#![allow(unsafe_code)]

use context::LayoutContext;
use flow::{self, Flow};
use flow_ref::FlowRef;
use profile_traits::time::{self, TimerMetadata, profile};
use rayon;
use rayon::iter::ParallelIterator;
use servo_config::opts;
use std::mem;
use traversal::{AssignBSizes, AssignISizes, BubbleISizes};
use traversal::{PostorderFlowTraversal, PreorderFlowTraversal};

/// Vtable + pointer representation of a Flow trait object.
pub type UnsafeFlow = (usize, usize);

pub fn mut_owned_flow_to_unsafe_flow(flow: *mut FlowRef) -> UnsafeFlow {
    unsafe {
        mem::transmute::<&Flow, UnsafeFlow>(&**flow)
    }
}

pub fn borrowed_flow_to_unsafe_flow(flow: &Flow) -> UnsafeFlow {
    unsafe {
        mem::transmute::<&Flow, UnsafeFlow>(flow)
    }
}

fn top_down_flow(unsafe_flow: &UnsafeFlow,
                 assign_isize_traversal: &AssignISizes,
                 assign_bsize_traversal: &AssignBSizes)
{
    unsafe {
        // Get a real flow.
        let flow: &mut Flow = mem::transmute(*unsafe_flow);

        if assign_isize_traversal.should_process(flow) {
            // Perform the appropriate traversal.
            assign_isize_traversal.process(flow);
        }

        // Possibly enqueue the children.
        flow::par_child_iter(flow).for_each(|kid| {
            top_down_flow(&borrowed_flow_to_unsafe_flow(kid),
                          &assign_isize_traversal,
                          &assign_bsize_traversal);
        });

        // Perform the appropriate traversal.
        if assign_bsize_traversal.should_process(flow) {
            assign_bsize_traversal.process(flow);
        }
    }
}

/// Run the main layout passes in parallel.
pub fn reflow(
        root: &mut Flow,
        profiler_metadata: Option<TimerMetadata>,
        time_profiler_chan: time::ProfilerChan,
        context: &LayoutContext,
        _queue: &rayon::ThreadPool) {
    if opts::get().bubble_inline_sizes_separately {
        let bubble_inline_sizes = BubbleISizes { layout_context: &context };
        bubble_inline_sizes.traverse(root);
    }

    let assign_isize_traversal = &AssignISizes { layout_context: &context };
    let assign_bsize_traversal = &AssignBSizes { layout_context: &context };
    let flow = borrowed_flow_to_unsafe_flow(root);

    profile(time::ProfilerCategory::LayoutParallelWarmup,
            profiler_metadata, time_profiler_chan, move || {
                top_down_flow(&flow, assign_isize_traversal, assign_bsize_traversal);
    });
}
