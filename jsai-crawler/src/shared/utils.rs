use anyhow::Result;
use charming::{
    Chart, ImageFormat, ImageRenderer,
    component::{DataView, Feature, Legend, Restore, SaveAsImage, Toolbox},
    element::ItemStyle,
    series::{Pie, PieRoseType},
};
use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress_bar(total: usize, msg: Option<String>) -> ProgressBar {
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{wide_bar}] {percent}% ({msg})")
            .unwrap()
            .progress_chars("█▓▒░")
            .tick_chars("⠋⠙⠚⠉"),
    );
    if let Some(m) = msg {
        pb.set_message(m);
    } else {
        pb.set_message("Processing".to_string());
    }
    pb
}

pub fn draw_pie(data: Vec<f64>, labels: Vec<String>, file_path: &str) -> Result<()> {
    let data = data.into_iter().zip(labels).collect::<Vec<_>>();
    let chart = Chart::new()
        .legend(Legend::new().top("bottom"))
        .toolbox(
            Toolbox::new().show(true).feature(
                Feature::new()
                    .data_view(DataView::new().show(true))
                    .restore(Restore::new().show(true))
                    .save_as_image(SaveAsImage::new().show(true)),
            ),
        )
        .series(
            Pie::new()
                .name("Nightingale Chart")
                .rose_type(PieRoseType::Radius)
                .radius(vec!["50", "150"])
                .center(vec!["50%", "50%"])
                .item_style(ItemStyle::new().border_radius(8))
                .data(data),
        );

    let mut renderer = ImageRenderer::new(1000, 800);
    renderer.save_format(ImageFormat::Png, &chart, file_path)?;
    Ok(())
}
