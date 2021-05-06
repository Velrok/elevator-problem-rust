

pub fn render_graph(progress: i32, max: i32) -> String {
    const BAR : &str = "##############################";
    let bar_width = (progress as f32 / max as f32 * 30.0) as usize;
    return BAR[0..bar_width].into()
}

mod tests {
    #[test]
    fn test_render_graph() {
        assert_eq!("#", crate::printer::render_graph(1, 30));
        assert_eq!("##############################", crate::printer::render_graph(30, 30));
    }
}