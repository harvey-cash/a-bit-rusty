// CIRCUIT REQUIREMENTS:
// [ ] A Circuit is an arrangement of Traces and Chips.
// [ ] The Circuit has a 3D integer coordinate space.
// [ ] The Circuit is infinite in X and Y but has only two Z layers: 0 (front) and 1 (back).
// [ ] Chips can be added to a Circuit.
// [ ] Chips are placed on the front layer.
// [ ] Chips occupy a non-zero 2D area of points on the board.
// [ ] Chips can be rotated in 90 degree increments.
// [ ] Chips may not overlap other Chips.
// [ ] Chips can be removed from a Circuit.
// [ ] Pins exist at points adjacent to their Chip's surface area on the front layer.
// [ ] Pins are two dimensional lines in Z, occupying Z = [0, 1] at a single XY co-ordinate.
// [ ] Pins may not be coincident with other Pins.
// [ ] Traces can be added to a Circuit.
// [ ] Traces can not overlap any point covered by a Chip (on the front layer).
// [ ] Traces can intersect Pins.
// [ ] A Trace Via can not be coincident with a Pin's XY coordinates.
// [ ] Traces can be deleted from a Circuit.
// [ ] When multiple Pins intersect Traces belonging to the same TraceGraph, they are connected.
// [ ] If a TraceGraph intersects an Output Pin, its value equals the state of the Output Pin.
// [ ] If a TraceGraph intersects multiple Output Pins it is invalid.
// [ ] If a Circuit is valid, it can be compiled to a ChipDescription.
// [ ] Circuits are invalid if any TraceGraph is invalid.
// [ ] Compilation turns TraceGraphs into Links in the ChipDescription.
//
