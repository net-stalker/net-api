use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;

use net_core_api::envelope::envelope::Envelope;


const DATA_TYPE: &str = "request_result";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RequestResultDTO {
    is_ok: bool,
    description: Option<String>,
    response: Option<Envelope>,
}
impl API for RequestResultDTO { }

impl RequestResultDTO {

    pub fn new(
        is_ok: bool,
        description: Option<&str>,
        response: Option<Envelope>
    ) -> Self {
        Self {
            is_ok,
            description: description.map(|id| id.into()),
            response
        }
    }
    
    pub fn is_ok(&self) -> bool {
        self.is_ok
    }

    pub fn get_description(&self) -> Result<&str, &str> {
        self.description
            .as_deref()
            .map_or_else(|| Err("There is no description provided"), Ok)
    }

    pub fn into_inner(self) -> Option<Envelope> {
        self.response
    }
}

impl Encoder for RequestResultDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("is_ok");
        writer.write_bool(self.is_ok).unwrap();

        writer.set_field_name("description");
        match &self.description {
            Some(description) => writer.write_string(description).unwrap(),
            None => writer.write_null(ion_rs::IonType::String).unwrap(),
        };

        writer.set_field_name("response");
        match &self.response {
            Some(response) => writer.write_blob(response.encode()).unwrap(),
            None => writer.write_null(ion_rs::IonType::Blob).unwrap(),
        };

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for RequestResultDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let is_ok = binary_user_reader.read_bool().unwrap();

        let binding;
        binary_user_reader.next().unwrap();
        let description = match binary_user_reader.current() {
            ion_rs::StreamItem::Value(_) => {
                binding = binary_user_reader.read_string().unwrap();
                Some(binding.text())
            },
            ion_rs::StreamItem::Null(_) => {
                None
            },
            //TODO: Return en error here in future
            ion_rs::StreamItem::Nothing => todo!(),
        };

        binary_user_reader.next().unwrap();
        let response = match binary_user_reader.current() {
            ion_rs::StreamItem::Value(_) => {
                Some(Envelope::decode(binary_user_reader.read_blob().unwrap().as_slice()))
            },
            ion_rs::StreamItem::Null(_) => {
                None
            },
            //TODO: Return en error here in future
            ion_rs::StreamItem::Nothing => todo!(),
        };

        binary_user_reader.step_out().unwrap();

        RequestResultDTO::new(
            is_ok,
            description,
            response
        )
    }
}

impl Typed for RequestResultDTO {
    fn get_data_type() -> &'static str {
        DATA_TYPE
    }

    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}


#[cfg(test)]
mod tests {
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use net_core_api::encoder_api::Encoder;
    use net_core_api::decoder_api::Decoder;
    use net_core_api::envelope::envelope::Envelope;
    use net_core_api::typed_api::Typed;

    use crate::api::request_result::request_result::RequestResultDTO;


    #[test]
    fn reader_correctly_read_encoded_request_result() {
        const IS_OK: bool = true;
        const DESCRIPTION: Option<&str> = Some("DESCRIPTION");

        const GROUP_ID: Option<&str> = Some("SOME_GROUP_ID");
        const AGENT_ID: Option<&str> = Some("SOME_AGENT_ID");
        const ENVELOPE_TYPE: &str = "ENVELOPE_TYPE";
        const ENVELOPE_DATA: &[u8] = "ENVELOPE_DATA".as_bytes();
        let  response: Option<Envelope> = Some(
            Envelope::new(
                GROUP_ID,
                AGENT_ID,
                ENVELOPE_TYPE,
                ENVELOPE_DATA
            )
        );

        let request_result = RequestResultDTO::new(
            IS_OK,
            DESCRIPTION,
            response.clone()
        );
        let mut binary_user_reader = ReaderBuilder::new().build(request_result.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::Bool), binary_user_reader.next().unwrap());
        assert_eq!("is_ok", binary_user_reader.field_name().unwrap());
        assert_eq!(IS_OK, binary_user_reader.read_bool().unwrap());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("description", binary_user_reader.field_name().unwrap());
        assert_eq!(DESCRIPTION.unwrap(), binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
        assert_eq!("response", binary_user_reader.field_name().unwrap());
        assert_eq!(response.unwrap(), Envelope::decode(binary_user_reader.read_blob().unwrap().as_slice()));

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_request_result() {
        const IS_OK: bool = true;
        const DESCRIPTION: Option<&str> = Some("DESCRIPTION");

        const GROUP_ID: Option<&str> = Some("SOME_GROUP_ID");
        const AGENT_ID: Option<&str> = Some("SOME_AGENT_ID");
        const ENVELOPE_TYPE: &str = "ENVELOPE_TYPE";
        const ENVELOPE_DATA: &[u8] = "ENVELOPE_DATA".as_bytes();
        let  response: Option<Envelope> = Some(
            Envelope::new(
                GROUP_ID,
                AGENT_ID,
                ENVELOPE_TYPE,
                ENVELOPE_DATA
            )
        );

        let request_result = RequestResultDTO::new(
            IS_OK,
            DESCRIPTION,
            response.clone()
        );
        assert_eq!(request_result, RequestResultDTO::decode(&request_result.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const IS_OK: bool = true;
        const DESCRIPTION: Option<&str> = Some("DESCRIPTION");

        const GROUP_ID: Option<&str> = Some("SOME_GROUP_ID");
        const AGENT_ID: Option<&str> = Some("SOME_AGENT_ID");
        const ENVELOPE_TYPE: &str = "ENVELOPE_TYPE";
        const ENVELOPE_DATA: &[u8] = "ENVELOPE_DATA".as_bytes();
        let  response: Option<Envelope> = Some(
            Envelope::new(
                GROUP_ID,
                AGENT_ID,
                ENVELOPE_TYPE,
                ENVELOPE_DATA
            )
        );

        let request_result = RequestResultDTO::new(
            IS_OK,
            DESCRIPTION,
            response.clone()
        );
        assert_eq!(request_result.get_type(), RequestResultDTO::get_data_type());
        assert_eq!(request_result.get_type(), super::DATA_TYPE);
    }
}

