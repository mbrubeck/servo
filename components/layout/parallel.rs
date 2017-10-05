/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Implements parallel traversals over the DOM and flow trees.
//!
//! This code is highly unsafe. Keep this file small and easy to audit.

#![allow(unsafe_code)]

use context::LayoutContext;
use flow::{self, Flow};
use profile_traits::time::{self, TimerMetadata, profile};
use rayon;
use rayon::iter::ParallelIterator;
use servo_config::opts;
use traversal::{AssignBSizes, AssignISizes, BubbleISizes};
use traversal::{PostorderFlowTraversal, PreorderFlowTraversal};

fn top_down_flow(flow: &mut Flow,
                 assign_isize_traversal: &AssignISizes,
                 assign_bsize_traversal: &AssignBSizes)
{
    if assign_isize_traversal.should_process(flow) {
        // Perform the appropriate traversal.
        assign_isize_traversal.process(flow);
    }

    // Traverse the children.
    flow::child_par_iter_mut(flow).for_each(|kid|
        top_down_flow(kid, &assign_isize_traversal, &assign_bsize_traversal));

    // Perform the appropriate traversal.
    if assign_bsize_traversal.should_process(flow) {
        assign_bsize_traversal.process(flow);
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

    profile(time::ProfilerCategory::LayoutParallelWarmup,
            profiler_metadata, time_profiler_chan, move || {
                top_down_flow(root, assign_isize_traversal, assign_bsize_traversal);
    });
}
