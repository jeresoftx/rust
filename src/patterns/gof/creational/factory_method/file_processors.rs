#[derive(Debug, PartialEq, Eq)]
pub enum ProcessorError {
    MissingExtension,
    UnsupportedExtension(String),
}

trait FileProcessor {
    fn process(&self, content: &str) -> String;
}

struct CsvProcessor;

impl FileProcessor for CsvProcessor {
    fn process(&self, content: &str) -> String {
        let mut lines = content.lines();
        let columns = lines
            .next()
            .map(|header| header.split(',').count())
            .unwrap_or_default();
        let rows = lines.filter(|line| !line.trim().is_empty()).count();

        format!("csv rows={rows} columns={columns}")
    }
}

struct JsonProcessor;

impl FileProcessor for JsonProcessor {
    fn process(&self, content: &str) -> String {
        let objects = content.matches('{').count();

        format!("json objects={objects}")
    }
}

struct XmlProcessor;

impl FileProcessor for XmlProcessor {
    fn process(&self, content: &str) -> String {
        let elements = content.matches("<order").count() - content.matches("<orders").count();

        format!("xml elements={elements}")
    }
}

fn processor_for(file_name: &str) -> Result<Box<dyn FileProcessor>, ProcessorError> {
    match file_name.rsplit_once('.') {
        Some((_, "csv")) => Ok(Box::new(CsvProcessor)),
        Some((_, "json")) => Ok(Box::new(JsonProcessor)),
        Some((_, "xml")) => Ok(Box::new(XmlProcessor)),
        Some((_, extension)) => Err(ProcessorError::UnsupportedExtension(extension.to_string())),
        None => Err(ProcessorError::MissingExtension),
    }
}

pub fn process_file(file_name: &str, content: &str) -> Result<String, ProcessorError> {
    let processor = processor_for(file_name)?;

    Ok(processor.process(content))
}
