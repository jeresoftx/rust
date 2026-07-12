use design_patterns_rust::patterns::architecture::pipeline_architecture::image_processing::{
    ImageJob, ImagePipeline, ProcessedImage, ProcessingError,
};

#[test]
fn pipeline_processes_image_through_resize_watermark_and_optimization() {
    let pipeline = ImagePipeline::thumbnail_pipeline("ACME");
    let job = ImageJob::new("hero.png", 4_000, 2_000, 8_000);

    let image = pipeline
        .process(job)
        .expect("valid image should be processed");

    assert_eq!(
        image,
        ProcessedImage::new(
            "hero.png",
            1_200,
            600,
            4_000,
            vec![
                "resize:1200x600".to_string(),
                "watermark:ACME".to_string(),
                "optimize:50%".to_string()
            ]
        )
    );
}

#[test]
fn pipeline_keeps_small_images_at_original_dimensions() {
    let pipeline = ImagePipeline::thumbnail_pipeline("ACME");
    let job = ImageJob::new("avatar.png", 400, 400, 1_000);

    let image = pipeline
        .process(job)
        .expect("small image should be processed");

    assert_eq!(
        image,
        ProcessedImage::new(
            "avatar.png",
            400,
            400,
            500,
            vec![
                "resize:400x400".to_string(),
                "watermark:ACME".to_string(),
                "optimize:50%".to_string()
            ]
        )
    );
}

#[test]
fn pipeline_rejects_invalid_image_dimensions_before_running_steps() {
    let pipeline = ImagePipeline::thumbnail_pipeline("ACME");
    let job = ImageJob::new("broken.png", 0, 400, 1_000);

    let result = pipeline.process(job);

    assert_eq!(result, Err(ProcessingError::InvalidDimensions));
}
