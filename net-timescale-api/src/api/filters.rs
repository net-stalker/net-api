use ion_rs;
use ion_rs::element::reader::ElementReader;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug, PartialEq, Eq)]
pub struct Filters {
    filters: Vec<String>,
    include: bool
}

impl Filters {
    pub fn new (include: bool, filters: &[String]) -> Self {
        Filters {
            include,
            filters: filters.to_vec(),
        }
    }

    pub fn is_include (&self) -> bool {
        self.include
    }

    pub fn get_filters (&self) -> &[String] {
        &self.filters
    }
}

impl Encoder for Filters {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        #[cfg(feature = "ion-binary")]
            let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        #[cfg(feature = "ion-text")]
            let text_writer_builder = ion_rs::TextWriterBuilder::new(TextKind::Compact);

        #[cfg(feature = "ion-binary")]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        #[cfg(feature = "ion-text")]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let mut writer = text_writer_builder.build(buffer).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("include");
        writer.write_bool(self.include).unwrap();

        writer.set_field_name("filters");
        writer.step_in(ion_rs::IonType::List).unwrap();
        self.filters.iter().for_each(|filter| {
            writer.write_string(filter).unwrap();
        });
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for Filters {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let include = binary_user_reader.read_bool().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut filters = Vec::<String>::with_capacity(elements.len());
        elements.iter().for_each(|element| {
            let filter = element.as_string().unwrap();
            filters.push(filter.to_string());
        });
        // step out from the list
        binary_user_reader.step_out().unwrap();
        // step out from the root structure
        binary_user_reader.step_out().unwrap();

        Filters::new(
            include,
            filters.as_slice(),
        )
    }
}


#[cfg(test)]
mod tests {
    use ion_rs::element::reader::ElementReader;
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use net_proto_api::decoder_api::Decoder;
    use net_proto_api::encoder_api::Encoder;

    use crate::api::filters::Filters;

    #[test]
    fn reader_correctly_read_encoded_ng_request() {
        const INCLUDE: bool = true;

        let expected_filters: Vec<String> = ["f1", "f2", "f3"]
            .iter()
            .map(|filter| filter.to_string())
            .collect();

        let filters = Filters::new(
            INCLUDE,
            expected_filters.as_slice(),
        );

        let mut binary_user_reader = ReaderBuilder::new().build(filters.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Bool), binary_user_reader.next().unwrap());
        assert_eq!("include", binary_user_reader.field_name().unwrap());
        assert_eq!(INCLUDE,  binary_user_reader.read_bool().unwrap());

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("filters", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let filters: Vec<String> = binary_user_reader.read_all_elements().unwrap()
            .iter()
            .map(|element| element.as_string().unwrap().to_string())
            .collect();
        binary_user_reader.step_out().unwrap();
        assert_eq!(expected_filters, filters);
        binary_user_reader.step_out().unwrap();

    }

    #[test]
    fn endec_ng_request() {
        const INCLUDE: bool = true;

        let expected_filters: Vec<String> = ["f1", "f2", "f3"]
            .iter()
            .map(|filter| filter.to_string())
            .collect();

        let filters = Filters::new(
            INCLUDE,
            expected_filters.as_slice(),
        );
        assert_eq!(filters, Filters::decode(&filters.encode()));
    }
}