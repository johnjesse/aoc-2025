// Parse into coords
// Create pairs
// calculate rectangle ares

fn main() {
    let input =
        std::fs::read_to_string("src/bin/day09/input.txt").expect("Failed to read input.txt");

    let coords: Vec<(i64, i64)> = parse_data(&input).collect();

    let pairs = get_pairs(coords.clone());

    let mut largest_area = 0;

    for pair in pairs.clone() {
        let area = get_area(pair.0, pair.1);
        // println!("Area: {} {:?} {:?}", area, pair.0, pair.1);
        if area > largest_area {
            largest_area = area;
        }
    }

    println!("Largest area: {}", largest_area);

    let mut sorted_pairs_by_area = pairs.clone();

    sorted_pairs_by_area.sort_by(|a, b| {
        let area_a = get_area(a.0, a.1);
        let area_b = get_area(b.0, b.1);
        return area_b.cmp(&area_a);
    });

    let largest_valid_pair = sorted_pairs_by_area
        .iter()
        .map(|pair| {
            // println!("Pair: {:?}", pair);
            return pair;
        })
        .find(|pair| is_rectangle_within_polygon(coords.clone(), **pair))
        .expect("No valid pair found");

    println!("Largest valid pair: {:?}", largest_valid_pair);
    println!(
        "Area: {}",
        get_area(largest_valid_pair.0, largest_valid_pair.1)
    );
}

fn is_rectangle_within_polygon(
    polygon_coords: Vec<(i64, i64)>,
    rectangle: ((i64, i64), (i64, i64)),
) -> bool {
    let (x_1, y_1) = rectangle.0;
    let (x_2, y_2) = rectangle.1;

    let rectangle_coords = vec![(x_1, y_1), (x_1, y_2), (x_2, y_2), (x_2, y_1)];
    let corners_inside = rectangle_coords.iter().all(|rectangle_coord| {
        return point_in_polygon(*rectangle_coord, polygon_coords.clone());
    });

    if !corners_inside {
        return false;
    }

    return !rectangle_edges_intersect_polygon(&rectangle_coords, &polygon_coords);
}

fn point_on_segment(p: (i64, i64), a: (i64, i64), b: (i64, i64)) -> bool {
    // Check if point p lies on line segment a-b
    let cross = (p.1 - a.1) * (b.0 - a.0) - (p.0 - a.0) * (b.1 - a.1);
    if cross != 0 {
        return false; // not collinear
    }

    // Check if p is within the bounding box of a-b
    return p.0 >= a.0.min(b.0)
        && p.0 <= a.0.max(b.0)
        && p.1 >= a.1.min(b.1)
        && p.1 <= a.1.max(b.1);
}

fn point_in_polygon(point: (i64, i64), polygon: Vec<(i64, i64)>) -> bool {
    let n = polygon.len();

    // First check if point is on any edge
    for i in 0..n {
        let j = (i + 1) % n;
        if point_on_segment(point, polygon[i], polygon[j]) {
            return true; // on edge counts as inside
        }
    }

    let (px, py) = (point.0 as f64, point.1 as f64);
    let mut inside = false;

    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = (polygon[i].0 as f64, polygon[i].1 as f64);
        let (xj, yj) = (polygon[j].0 as f64, polygon[j].1 as f64);

        // Check if ray from point crosses this edge
        if ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }

    return inside;
}

fn segments_intersect(
    a1: (i64, i64),
    a2: (i64, i64), // segment A
    b1: (i64, i64),
    b2: (i64, i64), // segment B
) -> bool {
    fn ccw(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> i64 {
        (c.1 - a.1) * (b.0 - a.0) - (b.1 - a.1) * (c.0 - a.0)
    }

    let d1 = ccw(a1, a2, b1);
    let d2 = ccw(a1, a2, b2);
    let d3 = ccw(b1, b2, a1);
    let d4 = ccw(b1, b2, a2);

    return ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0))
        && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0));
}

fn rectangle_edges_intersect_polygon(rect: &Vec<(i64, i64)>, polygon: &[(i64, i64)]) -> bool {
    let rect_edges = [
        (rect[0], rect[1]),
        (rect[1], rect[2]),
        (rect[2], rect[3]),
        (rect[3], rect[0]),
    ];

    let n = polygon.len();
    for i in 0..n {
        let poly_edge = (polygon[i], polygon[(i + 1) % n]);
        for &(r1, r2) in &rect_edges {
            if segments_intersect(r1, r2, poly_edge.0, poly_edge.1) {
                return true;
            }
        }
    }
    return false;
}

fn parse_data(input: &str) -> impl Iterator<Item = (i64, i64)> {
    return input.lines().map(|line| {
        let arr: [i64; 2] = line
            .split(",")
            .map(|num| {
                // println!("num: {}", num);
                return num.parse::<i64>().unwrap();
            })
            .collect::<Vec<i64>>()
            .try_into()
            .expect("Failed to parse line into [i64; 2]");

        return (arr[0], arr[1]);
    });
}

fn get_pairs(corners: Vec<(i64, i64)>) -> Vec<((i64, i64), (i64, i64))> {
    return corners
        .iter()
        .enumerate()
        .flat_map(|(index, corner)| {
            corners
                .iter()
                .skip(index + 1)
                .map(|other_corner| (*corner, *other_corner))
        })
        .collect::<Vec<((i64, i64), (i64, i64))>>();
}

fn get_area(corner_1: (i64, i64), corner_2: (i64, i64)) -> i64 {
    let width = (corner_2.0 - corner_1.0).abs() + 1;
    let height = (corner_2.1 - corner_1.1).abs() + 1;
    return (width * height).abs();
}
