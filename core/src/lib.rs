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
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Courier",
        });
        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });

        let sender_text = self.sender.clone().unwrap();
        let sender = sender_text.lines();
        let mut content_operations = Vec::new();
        content_operations.append(&mut vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 12.into()]),
            Operation::new("Td", vec![50.into(), 50.into()]),
            Operation::new("TL", vec![12.into()]),
        ]);

        content_operations.push(Operation::new("Tj", vec![Object::string_literal("1")]));
        content_operations.push(Operation::new("'", vec![Object::string_literal("2")]));
        content_operations.push(Operation::new("'", vec![Object::string_literal("3")]));

        content_operations.push(Operation::new("ET", vec![]));

        let content = Content {
            operations: content_operations,
        };
        let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
        });
        let (width, height) = get_dimensions(&self.paper_size);
        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
            "Resources" => resources_id,
            "MediaBox" => vec![0.into(), 0.into(), width.into(), height.into()],
        };
        doc.objects.insert(pages_id, Object::Dictionary(pages));
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);
        doc.save_to(destination).unwrap()
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
