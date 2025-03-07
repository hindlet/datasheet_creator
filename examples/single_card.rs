use datasheet_creator::render_card;

fn main() {
    render_card("examples/example_unit.ron".to_string(), false, Some("examples/single_unit".to_string()), true);
}