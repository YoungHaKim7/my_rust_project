// A named tuple with two fields.
pub struct Coordinate(f64, f64);

// A named tuple that contains two named tuples.
pub struct Line(Coordinate, Coordinate);

// This function takes an optional named tuple and returns an optional Box smart pointer containing four
// coordinates.
pub fn make_quad_coordinates(maybe_coordinate: Option<Coordinate>) -> Option<Box<[Coordinate; 4]>> {
    // Check if the Option uses the Some variant. If it does, then extract the contents of the tuple
    // into x and y variables. If a None variant is used, the function simply returns.
    // The question mark is syntactic sugar for matching and extracting from a Some and returning on None.
    let Coordinate(x, y) = maybe_coordinate?;

    // Create a new Box smart pointer containing four coordinates. Note that this involves memory
    // allocation on the heap.
    Some(Box::new([
        Coordinate(x, y),
        Coordinate(-x, -y),
        Coordinate(-x, y),
        Coordinate(x, -y),
    ]))
}

// This function takes an optional named tuple and returns an optional tuple by value.
pub fn cross_lines_from_quad_coordinates(
    maybe_coordinate: Option<Coordinate>,
) -> Option<(Line, Line)> {
    // Pattern match and extract the contents of the array if the Option uses the Some variant.
    // Return None if the Option uses the None variant.
    let [a, b, c, d] = *make_quad_coordinates(maybe_coordinate)?;

    // Form two lines from four coordinates and return them as a tuple.
    // The tuple is wrapped in a Some variant.
    Some((Line(a, b), Line(c, d)))
}

fn main() {
    println!("Hello, world!");
}
