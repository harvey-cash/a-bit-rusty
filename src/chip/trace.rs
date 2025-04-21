use std::collections::{HashMap, HashSet};

use nalgebra::{self, Matrix};

pub type Vector3 = nalgebra::Vector3<i32>;

pub struct TraceMap {
    trace_map: HashMap<Vector3, HashSet<Vector3>>,
}

impl TraceMap {
    pub fn new() -> Self {
        let trace_map: HashMap<Vector3, HashSet<Vector3>> = HashMap::new();
        Self { trace_map }
    }

    pub fn add(&mut self, a: Vector3, b: Vector3) {
        Self::panic_if_ends_too_far_apart(&a, &b);

        self.trace_map.entry(a).or_default().insert(b);
        self.trace_map.entry(b).or_default().insert(a);
    }

    pub fn get_graphs(&self) -> HashMap<i32, HashSet<Vector3>> {
        let mut trace_id = 0;
        let mut traces: HashMap<i32, HashSet<Vector3>> = HashMap::new();
        let mut covered_points: HashSet<Vector3> = HashSet::new();

        for (point, connections) in &self.trace_map {
            if covered_points.contains(point) {
                continue;
            }

            let mut points_on_trace: HashSet<Vector3> = HashSet::new();
            self.find_all_points_on_trace(
                &point,
                &connections,
                &mut points_on_trace,
                &mut covered_points,
            );

            traces.insert(trace_id, points_on_trace.clone());
            trace_id += 1;
        }

        return traces;
    }

    fn find_all_points_on_trace(
        &self,
        point: &Vector3,
        connections: &HashSet<Vector3>,
        points_on_trace: &mut HashSet<Vector3>,
        covered_points: &mut HashSet<Vector3>,
    ) {
        if covered_points.contains(point) {
            return;
        }

        points_on_trace.insert(*point);
        covered_points.insert(*point);

        for connected_point in connections {
            let connections_at_point: &HashSet<Vector3> =
                self.trace_map.get(connected_point).unwrap();

            self.find_all_points_on_trace(
                connected_point,
                connections_at_point,
                points_on_trace,
                covered_points,
            );
        }
    }

    fn panic_if_ends_too_far_apart(a: &Vector3, b: &Vector3) {
        let delta = a - b;
        let abs_delta = Matrix::abs(&delta);
        if abs_delta.x > 1 || abs_delta.y > 1 || abs_delta.z > 1 {
            panic!("Trace ends too far apart! {} - {}", a, b);
        }
    }

    /*
    void FindAllPointsOnTrace(TraceSquare traceSquare, std::set<BoardPoint> &pointsOnTrace, std::set<BoardPoint> &coveredPoints) const
        {
            if (coveredPoints.find(traceSquare.position) != coveredPoints.end())
            {
                return; // This point already covered.
            }

            pointsOnTrace.insert(traceSquare.position);
            coveredPoints.insert(traceSquare.position);

            for (const BoardPoint &connection : traceSquare.connections)
            {
                const TraceSquare &connectedSquare = mTraceSquares.at(connection);
                FindAllPointsOnTrace(connectedSquare, pointsOnTrace, coveredPoints); // Recurse on connections.
            }
        }
    */

    /*
    std::map<int, std::set<BoardPoint>> GetTraces() const
        {
            int traceID = 0;
            std::map<int, std::set<BoardPoint>> traces;

            std::set<BoardPoint> coveredPoints;

            for (const std::pair<BoardPoint, TraceSquare> &squarePair : mTraceSquares)
            {
                if (coveredPoints.find(squarePair.first) != coveredPoints.end())
                {
                    continue; // This point already covered in a previous trace.
                }

                std::set<BoardPoint> pointsOnTrace;

                FindAllPointsOnTrace(squarePair.second, pointsOnTrace, coveredPoints);

                traces.emplace(traceID, std::set<BoardPoint>(pointsOnTrace));
                traceID++;
            }

            return traces;
        }
         */
}
