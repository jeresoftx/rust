#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ImageJob` usado por el ejemplo para expresar el dominio del patron.
pub struct ImageJob {
    file_name: String,
    width: u32,
    height: u32,
    size_kb: u32,
}

impl ImageJob {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(file_name: impl Into<String>, width: u32, height: u32, size_kb: u32) -> Self {
        Self {
            file_name: file_name.into(),
            width,
            height,
            size_kb,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ProcessedImage` usado por el ejemplo para expresar el dominio del patron.
pub struct ProcessedImage {
    file_name: String,
    width: u32,
    height: u32,
    size_kb: u32,
    operations: Vec<String>,
}

impl ProcessedImage {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        file_name: impl Into<String>,
        width: u32,
        height: u32,
        size_kb: u32,
        operations: Vec<String>,
    ) -> Self {
        Self {
            file_name: file_name.into(),
            width,
            height,
            size_kb,
            operations,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `ProcessingError` dentro del ejemplo.
pub enum ProcessingError {
    /// Variante `InvalidDimensions` del estado o error del ejemplo.
    InvalidDimensions,
}

#[derive(Debug)]
/// Tipo publico `ImagePipeline` usado por el ejemplo para expresar el dominio del patron.
pub struct ImagePipeline {
    max_width: u32,
    watermark: String,
    optimization_percent: u32,
}

impl ImagePipeline {
    /// Modela la operacion `thumbnail pipeline` dentro del ejemplo del patron.
    pub fn thumbnail_pipeline(watermark: impl Into<String>) -> Self {
        Self {
            max_width: 1_200,
            watermark: watermark.into(),
            optimization_percent: 50,
        }
    }

    /// Modela la operacion `process` dentro del ejemplo del patron.
    pub fn process(&self, job: ImageJob) -> Result<ProcessedImage, ProcessingError> {
        let image = validate_dimensions(job)?;
        let image = resize_to_max_width(image, self.max_width);
        let image = apply_watermark(image, &self.watermark);
        Ok(optimize(image, self.optimization_percent))
    }
}

/// Operacion `validate dimensions` definida por la abstraccion del ejemplo.
fn validate_dimensions(job: ImageJob) -> Result<ProcessedImage, ProcessingError> {
    if job.width == 0 || job.height == 0 {
        return Err(ProcessingError::InvalidDimensions);
    }

    Ok(ProcessedImage::new(
        job.file_name,
        job.width,
        job.height,
        job.size_kb,
        Vec::new(),
    ))
}

/// Operacion `resize to max width` definida por la abstraccion del ejemplo.
fn resize_to_max_width(mut image: ProcessedImage, max_width: u32) -> ProcessedImage {
    if image.width > max_width {
        image.height = image.height * max_width / image.width;
        image.width = max_width;
    }

    image
        .operations
        .push(format!("resize:{}x{}", image.width, image.height));
    image
}

/// Operacion `apply watermark` definida por la abstraccion del ejemplo.
fn apply_watermark(mut image: ProcessedImage, watermark: &str) -> ProcessedImage {
    image.operations.push(format!("watermark:{watermark}"));
    image
}

/// Operacion `optimize` definida por la abstraccion del ejemplo.
fn optimize(mut image: ProcessedImage, optimization_percent: u32) -> ProcessedImage {
    image.size_kb = image.size_kb * optimization_percent / 100;
    image
        .operations
        .push(format!("optimize:{optimization_percent}%"));
    image
}
