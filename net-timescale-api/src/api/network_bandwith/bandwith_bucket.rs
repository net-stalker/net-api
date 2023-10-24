use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BandwithBucketDTO {
    bucket_timestamp: i64,
    total_bytes: i64,
}

impl BandwithBucketDTO {
    pub fn new (bucket_timestamp: i64, total_bytes: i64) -> Self {
        BandwithBucketDTO {
            bucket_timestamp,
            total_bytes,
        }
    }

    pub fn get_bucket_timestamp (&self) -> i64 {
        self.bucket_timestamp
    }

    pub fn get_total_bytes (&self) -> i64 {
        self.total_bytes
    }
}

impl Encoder for BandwithBucketDTO {
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

        writer.set_field_name("bucket_timestamp");
        writer.write_i64(self.bucket_timestamp).unwrap();

        writer.set_field_name("total_bytes");
        writer.write_i64(self.total_bytes).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for BandwithBucketDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let bucket_timestamp = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        let total_bytes = binary_user_reader.read_i64().unwrap();

        BandwithBucketDTO::new(
            bucket_timestamp,
            total_bytes
        )
    }
}


#[cfg(test)]
mod tests {
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use net_proto_api::decoder_api::Decoder;
    use net_proto_api::encoder_api::Encoder;

    use crate::api::network_bandwith::bandwith_bucket::BandwithBucketDTO;


    #[test]
    fn reader_correctly_read_encoded_bandwith_bucket() {
        const BUCKET_TIMESTAMP: i64 = i64::MAX;
        const TOTAL_BYTES: i64 = i64::MAX;

        let bandwith_bucket = BandwithBucketDTO::new(
            BUCKET_TIMESTAMP,
            TOTAL_BYTES
        );
        let mut binary_user_reader = ReaderBuilder::new().build(bandwith_bucket.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("bucket_timestamp", binary_user_reader.field_name().unwrap());
        assert_eq!(BUCKET_TIMESTAMP,  binary_user_reader.read_i64().unwrap());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("total_bytes", binary_user_reader.field_name().unwrap());
        assert_eq!(TOTAL_BYTES,  binary_user_reader.read_i64().unwrap());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_bandwith_bucket() {
        const BUCKET_TIMESTAMP: i64 = i64::MAX;
        const TOTAL_BYTES: i64 = i64::MAX;

        let bandwith_bucket = BandwithBucketDTO::new(
            BUCKET_TIMESTAMP,
            TOTAL_BYTES
        );
        assert_eq!(bandwith_bucket, BandwithBucketDTO::decode(&bandwith_bucket.encode()));
    }
}
