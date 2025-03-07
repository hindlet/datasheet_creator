use datasheet_creator::render_folder;

fn main() {
    render_folder("examples/example_folder".to_string(), false, "examples/folder_output".to_string(), true);
}