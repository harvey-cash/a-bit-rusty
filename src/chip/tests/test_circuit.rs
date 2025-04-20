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
// [ ] Chips have a Pin for Ground, Supply, each Input and each Output.
// [ ] Pins exist at points adjacent to their Chip's surface area on the front layer.
// [ ] Pins are two dimensional lines in Z, occupying Z = [0, 1] at a single XY co-ordinate.
// [ ] Pins may not be coincident with other Pins.
// [ ] Traces can be added to a Circuit.
// [ ] Traces are Nodes in a TraceGraph.
// [ ] Traces have two end points in 3D space.
// [ ] The two ends of a Trace may be the same point - making it a Dot.
// [ ] If on the same Z layer, the two ends of a Trace may be adjacent or diagonal.
// [ ] If on different Z layers, the two ends of a Trace must be adjacent - making it a Via.
// [ ] Traces can not overlap any point covered by a Chip (on the front layer).
// [ ] Traces can intersect Pins.
// [ ] A Trace Via can not be coincident with a Pin's XY coordinates.
// [ ] Traces can be deleted from a Circuit.
// [ ] Traces that share an end point are part of the same TraceGraph. 
// [ ] Traces belonging to different TraceGraphs have no common end points.
// [ ] A TraceGraph has >=1 Traces.
// [ ] When multiple Pins intersect Traces belonging to the same TraceGraph, they are connected.
// [ ] TraceGraphs have a state value which defaults to 0.
// [ ] If a TraceGraph intersects an Output Pin, its value equals the state of the Output Pin.
// [ ] If a TraceGraph intersects multiple Output Pins it is invalid.
// [ ] The fundamental Chips are Ground, Supply, Input, NAnd, and Output.
// [ ] Ground Chips have a single Output Pin which is 0.
// [ ] Supply Chips have a single Output Pin which is 1 (if the Machine is powered).
// [ ] Chips have >=1 Ground Input Pins and >=1 Supply Input Pins.
// [ ] Chip Output Pins are all 0 if Ground Input != 0 or Supply Input != 1.
// [ ] If a Circuit is valid, it can be compiled to a ChipDescription.
// [ ] Circuits are invalid if any TraceGraph is invalid.
// [ ] Compilation turns TraceGraphs into Links in the ChipDescription.
// [ ] ChipDescriptions define the XY size of a new Chip.
// [ ] ChipDescriptions define the co-ordinates of new Pins adjacent the the size bounds.
// [ ] If a ChipDescription is valid, it can be used to create a new Chip.
// [ ] ChipDescriptions are invalid if any Chip's Ground Pin is not connected to a Ground Chip.
// [ ] ChipDescriptions are invalid if any Chip's Supply Pin is not connected to a Supply Chip.
// [ ] ChipDescriptions are invalid if there is not >=1 Ground, >=1 Supply, >=1 Input and >=1 Output.
// [ ] ChipDescriptions are invalid if multiple sources have the same target.
// [ ] ChipDescriptions are invalid if any Chip is unconnected.
//
