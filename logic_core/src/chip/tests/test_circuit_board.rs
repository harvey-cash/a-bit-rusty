// ToDo:
// [ ] CircuitBoards have an arrangement of Traces and Chips.
// [ ] CircuitBoards have a 3D integer coordinate space. Z=0 is the "front layer".
// [ ] Chips are placed on the front layer.
// [ ] Chips occupy a non-zero 2D area of points on the board.
// [ ] Chips can be rotated in 90 degree increments.
// [ ] Chips may not overlap other Chips.
// [ ] Chips can be removed from a CircuitBoard.
// [ ] Pins exist at points adjacent to their Chip's surface area on the front layer.
// [ ] Pins are two dimensional lines in Z, occupying Z = [0, 1] at a single XY co-ordinate.
// [ ] Pins may not be coincident with other Pins (of other Chips).
// [ ] TraceSegments can be added to a CircuitBoard.
// [ ] TraceSegments can not overlap any point covered by a Chip (on the front layer).
// [ ] TraceSegments can intersect Pins.
// [ ] A Trace Via can not be coincident with a Pin's XY coordinates.
// [ ] TraceSegments can be deleted from a CircuitBoard.
// [ ] When multiple Pins intersect TraceSegments belonging to the same Trace, they are connected.
// [ ] Traces have a state value which defaults to 0.
// [ ] If a Trace intersects one Output Pin, its value equals the state of the Output Pin.
// [ ] If a Trace intersects multiple Output Pins it is invalid.
// [ ] CircuitBoards are invalid if any Trace is invalid.
// [ ] Compilation turns Traces into Links in the CircuitDescription.
// [ ] Ticking a CircuitBoard calls tick on all Chips with no Input pins connected to an invalid Trace.
// [ ] Before a Chip is ticked, its Inputs are set to the values of the connected Traces.
// [ ] After a Chip is ticked, Traces connected to its Outputs have their value set.
// [ ] After a CircuitBoard is ticked, all output Chips have their values set to the value of the connected Traces.
// [ ] Trace states can be read from a CircuitBoard.