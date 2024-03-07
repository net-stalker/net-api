use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;


const DATA_TYPE: &str = "http_responses_bucket";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpResponsesBucketDTO {
    bucket_timestamp: i64,
    response_code: i64,
    amount: i64,
}
impl API for HttpResponsesBucketDTO { }

impl HttpResponsesBucketDTO {
    pub fn new(bucket_timestamp: i64, response_code: i64, amount: i64) -> Self {
        HttpResponsesBucketDTO {
            bucket_timestamp,
            response_code,
            amount,
        }
    }

    pub fn get_bucket_timestamp(&self) -> i64 {
        self.bucket_timestamp
    }

    pub fn get_resnpose_code(&self) -> i64 {
        self.response_code
    }

    pub fn get_response_amount(&self) -> i64 {
        self.amount
    }
}

impl Encoder for HttpResponsesBucketDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new(); 
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("bucket_timestamp");
        writer.write_i64(self.bucket_timestamp).unwrap();

        writer.set_field_name("response_code");
        writer.write_i64(self.response_code).unwrap();

        writer.set_field_name("amount");
        writer.write_i64(self.amount).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for HttpResponsesBucketDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let bucket_timestamp = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        let response_code = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        let amount = binary_user_reader.read_i64().unwrap();

        HttpResponsesBucketDTO::new(
            bucket_timestamp,
            response_code,
            amount,
        )
    }
}

impl Typed for HttpResponsesBucketDTO {
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

    use crate::api::http_responses_dist::http_responses_bucket::HttpResponsesBucketDTO;

    #[test]
    fn reader_correctly_read_encoded_http_responses_bucket() {
        const BUCKET_TIMESTAMP: i64 = i64::MAX;
        const RESPONSE_CODE: i64 = i64::MIN;
        const AMOUNT: i64 = 0;

        let http_response_bucket = HttpResponsesBucketDTO::new(
            BUCKET_TIMESTAMP,
            RESPONSE_CODE,
            AMOUNT,
        );
        let mut binary_user_reader = ReaderBuilder::new().build(http_response_bucket.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("bucket_timestamp", binary_user_reader.field_name().unwrap());
        assert_eq!(BUCKET_TIMESTAMP,  binary_user_reader.read_i64().unwrap());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("response_code", binary_user_reader.field_name().unwrap());
        assert_eq!(RESPONSE_CODE,  binary_user_reader.read_i64().unwrap());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("amount", binary_user_reader.field_name().unwrap());
        assert_eq!(AMOUNT,  binary_user_reader.read_i64().unwrap());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_http_responses_bucket() {
        const BUCKET_TIMESTAMP: i64 = i64::MAX;
        const RESPONSE_CODE: i64 = i64::MIN;
        const AMOUNT: i64 = 0;

        let http_response_bucket = HttpResponsesBucketDTO::new(
            BUCKET_TIMESTAMP,
            RESPONSE_CODE,
            AMOUNT,
        );
        assert_eq!(http_response_bucket, HttpResponsesBucketDTO::decode(&http_response_bucket.encode()));
    }
}

