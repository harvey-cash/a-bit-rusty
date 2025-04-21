// TRACE REQUIREMENTS:
// [ ] TraceSegments are Nodes in a Trace.
// [ ] TraceSegments have two end points in 3D space.
// [ ] The two ends of a TraceSegment may be the same point - making it a Dot.
// [ ] If on the same Z layer, the two ends of a TraceSegment may be adjacent or diagonal.
// [ ] If on different Z layers, the two ends of a TraceSegment must be adjacent - making it a Via.
// [ ] TraceSegments that share an end point are part of the same Trace.
// [ ] TraceSegments belonging to different Traces have no common end points.
// [ ] A Trace has >=1 TraceSegments.
// [ ] Traces have a state value which defaults to 0.

//GivenSingleTraceSegment_WhenDeleteSinglePoint_ThenStillSingleTrace
//GivenSingleTraceSegment_WhenDeleteBothPoints_ThenNoTraces
//GivenTwoTraceSegmentsOverlap_ThenOneTraceExists
//GivenTwoTraceSegmentsOverlap_WhenDeletePointOfOverlap_ThenTwoTracesExist
//GivenTwoTraces_ThenTwoTracesExist
//GiveTwoTraces_WhenJoined_ThenOneExists
//GivenTwoTracesOnDifferentLayers_ThenTwoTracesExist
//GivenTwoTracesOnDifferentLayers_WhenJoinedByVia_ThenOneTraceExists
//GivenTwoTracesJoinedByVia_WhenViaDeleted_ThenTwoTracesExist

use nalgebra::vector;

use crate::chip::TraceMap;

#[test]
fn given_no_traces_then_no_trace_graph() {
    let map = TraceMap::new();
    assert_eq!(map.get_graphs().len(), 0);
}

#[test]
#[should_panic]
fn given_trace_ends_too_far_apart_then_panics() {
    let mut map = TraceMap::new();
    map.add(vector![0, 0, 0], vector![2, 0, 0]);
}

#[test]
fn given_one_point_then_one_trace_graph() {
    let mut map = TraceMap::new();
    map.add(vector![0, 0, 0], vector![0, 0, 0]);
    assert_eq!(map.get_graphs().len(), 1);
}

#[test]
fn given_two_separate_points_then_two_trace_graphs() {
    let mut map = TraceMap::new();
    map.add(vector!(0, 0, 0), vector!(0, 0, 0));
    map.add(vector!(1, 1, 1), vector!(1, 1, 1));
    assert_eq!(map.get_graphs().len(), 2);
}
