// CIRCUIT REQUIREMENTS:
// [ ] A Circuit is an arrangement of TracePoints and Chips.
// [ ] The Circuit has a 3D integer coordinate space.
// [ ] The Circuit is infinite in X and Y but has only two Z layers: 0 (front) and 1 (back).
// [ ] Chips can be added to a Circuit.
// [ ] Chips are placed on the front layer.
// [ ] Chips occupy a non-zero 2D area of points on the board.
// [ ] Chips may not overlap other Chips.
// [ ] Chips can be removed from a Circuit.
// [ ] Chips have a Pin for Ground, Supply, each Input and each Output.
// [ ] Pins exist at points adjacent to their Chip's surface area on the front layer.
// [ ] Pins are two dimensional lines in Z, occupying Z = [0, 1] at a single XY co-ordinate.
// [ ] Pins may not be coincident with other Pins.
// [ ] TracePoints can not be placed at any point covered by a Chip (on the front layer).
// [ ] TracePoints can be coincident with Pins.
// [ ] Two TracePoints stacked vertically in Z are called a Via.
// [ ] Traces are continuously adjacently connected trees of TracePoints.
// [ ] A Trace has >=1 TracePoints.
// [ ] When multiple Pins are coincident with TracePoints belonging to the same Trace, they are connected.
// [ ] The fundamental Chips are Ground, Supply, Input, NAnd, and Output.
// [ ] If the Circuit is valid, it can be compiled to a Chip.
// [ ] The Circuit is invalid if it does not have >=1 Ground, >=1 Supply, >=1 Input and >=1 Output.
// [ ] The Circuit may have 0 or more NAnds and other custom Chips.
// [ ] The Circuit is invalid if the Chip it would be compiled to would not be valid.
// [ ] The Circuit is invalid if multiple Output Pins are connected on the same Trace.
// [ ] The Circuit is invalid if a Ground Pin of any Chip is not connected to a Ground Chip of the Circuit.
// [ ] The Circuit is invalid if a Supply Pin of any Chip is not connected to a Supply Chip of the Circuit.
// [ ] A Circuit is invalid if any Chip has 0 Pins connected to any other Chip.
//
