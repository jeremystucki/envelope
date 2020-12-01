#[macro_use]
extern crate lopdf;

use lopdf::content::{Content, Operation};
use lopdf::{Document, Object, Stream};
use std::io::Write;

pub struct Envelope {
    paper_size: PaperSize,
    sender: Option<String>,
    recipient: Option<String>,
}

impl Envelope {
    pub fn new_with_size(paper_size: PaperSize) -> Self {
        Self {
            paper_size,
            sender: None,
            recipient: None,
        }
    }

    pub fn sender(mut self, sender: String) -> Self {
        self.sender = Option::Some(sender);
        self
    }

    pub fn recipient(mut self, recipient: String) -> Self {
        self.recipient = Option::Some(recipient);
        self
    }

    pub fn write<W: Write>(&self, destination: &mut W) {
        let mut document = Document::with_version("1.5");

        let pages_id = document.new_object_id();
        let font_id = document.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Courier",
        });
        let resources_id = document.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });

        let (width, height) = get_dimensions(&self.paper_size);

        let mut operations = self
            .sender
            .as_ref()
            .map(|text| Self::generate_text_operations(text, 5, (10, height - 10)))
            .unwrap_or_default();

        operations.append(
            &mut self
                .recipient
                .as_ref()
                .map(|text| Self::generate_text_operations(text, 7, (width / 2, height / 2)))
                .unwrap_or_default(),
        );

        let content = Content { operations };

        let content_id =
            document.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));

        let page_id = document.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
        });

        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
            "Resources" => resources_id,
            "MediaBox" => vec![0.into(), 0.into(), width.into(), height.into()],
        };

        document.objects.insert(pages_id, Object::Dictionary(pages));

        let catalog_id = document.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        document.trailer.set("Root", catalog_id);
        document.save_to(destination).unwrap()
    }

    fn generate_text_operations(
        text: &str,
        text_size: u8,
        (offset_left, offset_bottom): (u32, u32),
    ) -> Vec<Operation> {
        let lines = text.lines();
        let mut operations = Vec::new();

        operations.append(&mut vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), text_size.into()]),
            Operation::new("Td", vec![offset_left.into(), offset_bottom.into()]),
            Operation::new("TL", vec![text_size.into()]),
        ]);

        operations.append(
            &mut lines
                .map(|line| Operation::new("'", vec![Object::string_literal(line)]))
                .collect(),
        );

        operations.push(Operation::new("ET", vec![]));

        operations
    }
}

pub enum PaperSize {
    C5,
    Custom(u32, u32),
}

fn get_dimensions(paper_size: &PaperSize) -> (u32, u32) {
    match paper_size {
        PaperSize::C5 => (229, 162),
        PaperSize::Custom(width, height) => (*width, *height),
    }
}
