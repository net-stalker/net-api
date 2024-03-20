use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "bucket";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpRequestsBucketDTO {
    bucket_timestamp: i64,
    total_requests: i64,
}
impl API for HttpRequestsBucketDTO { }

impl HttpRequestsBucketDTO {
    pub fn new(bucket_timestamp: i64, total_requests: i64) -> Self {
        HttpRequestsBucketDTO {
            bucket_timestamp,
            total_requests,
        }
    }

    pub fn get_bucket_timestamp(&self) -> i64 {
        self.bucket_timestamp
    }

    pub fn get_total_requests(&self) -> i64 {
        self.total_requests
    }
}

impl Encoder for HttpRequestsBucketDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new(); 
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("bucket_timestamp");
        writer.write_i64(self.bucket_timestamp).unwrap();

        writer.set_field_name("total_requests");
        writer.write_i64(self.total_requests).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for HttpRequestsBucketDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let bucket_timestamp = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        let total_requests = binary_user_reader.read_i64().unwrap();

        HttpRequestsBucketDTO::new(
            bucket_timestamp,
            total_requests
        )
    }
}

impl Typed for HttpRequestsBucketDTO {
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

    use net_core_api::core::encoder_api::Encoder;
    use net_core_api::core::decoder_api::Decoder;

    use crate::api::total_http_requests::http_requests_bucket::HttpRequestsBucketDTO;

    #[test]
    fn reader_correctly_read_encoded_requests_bucket() {
        const BUCKET_TIMESTAMP: i64 = i64::MAX;
        const TOTAL_REQUESTS: i64 = i64::MAX;

        let bandwidth_bucket = HttpRequestsBucketDTO::new(
            BUCKET_TIMESTAMP,
            TOTAL_REQUESTS
        );
        let mut binary_user_reader = ReaderBuilder::new().build(bandwidth_bucket.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("bucket_timestamp", binary_user_reader.field_name().unwrap());
        assert_eq!(BUCKET_TIMESTAMP,  binary_user_reader.read_i64().unwrap());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("total_requests", binary_user_reader.field_name().unwrap());
        assert_eq!(TOTAL_REQUESTS,  binary_user_reader.read_i64().unwrap());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_requests_bucket() {
        const BUCKET_TIMESTAMP: i64 = i64::MAX;
        const TOTAL_REQUESTS: i64 = i64::MAX;

        let bandwidth_bucket = HttpRequestsBucketDTO::new(
            BUCKET_TIMESTAMP,
            TOTAL_REQUESTS
        );
        assert_eq!(bandwidth_bucket, HttpRequestsBucketDTO::decode(&bandwidth_bucket.encode()));
    }
}

