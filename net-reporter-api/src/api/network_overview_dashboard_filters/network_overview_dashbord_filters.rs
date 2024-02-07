use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;
use ion_rs::element::reader::ElementReader;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;

use super::filter_entry::FilterEntryDTO;


const DATA_TYPE: &str = "network-overview-dashboard-filters";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NetworkOverviewDashboardFiltersDTO {
    entries: Vec<FilterEntryDTO>,
}
impl API for NetworkOverviewDashboardFiltersDTO { }

impl NetworkOverviewDashboardFiltersDTO {
    pub fn new(entries: &[FilterEntryDTO]) -> Self {
        NetworkOverviewDashboardFiltersDTO {
            entries: entries.to_vec(),
        }
    }

    pub fn get_entries(&self) -> &[FilterEntryDTO] {
        &self.entries
    }
}

impl Encoder for NetworkOverviewDashboardFiltersDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("entries");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        for entry in &self.entries {
            writer.write_blob(entry.encode()).unwrap();
        }
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkOverviewDashboardFiltersDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut entries = Vec::<FilterEntryDTO>::with_capacity(elements.len());
        for element in elements {
            entries.push(FilterEntryDTO::decode(element.as_blob().unwrap()));
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();

        NetworkOverviewDashboardFiltersDTO::new(
            entries.as_slice(),
        )
    }
}

impl Typed for NetworkOverviewDashboardFiltersDTO {
    fn get_data_type() -> &'static str {
        DATA_TYPE
    }
    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}


#[cfg(test)]
mod tests {
    use ion_rs::element::reader::ElementReader;
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use net_core_api::encoder_api::Encoder;
    use net_core_api::decoder_api::Decoder;
    use net_core_api::typed_api::Typed;

    use crate::api::network_overview_dashboard_filters::filter_entry::FilterEntryDTO;
    use crate::api::network_overview_dashboard_filters::network_overview_dashbord_filters::NetworkOverviewDashboardFiltersDTO;

    fn get_filters() -> NetworkOverviewDashboardFiltersDTO {
        const ENDPOINT_1: &str = "0.0.0.0:0000";
        let protocols_1: Vec<String> = vec!["fac1_1".to_string(), "fac2_1".to_string(), "fac3_1".to_string()];
        let total_bytes_1 = 1000;
        
        const ENDPOINT_2: &str = "1.1.1.1:1111";
        let protocols_2: Vec<String> = vec!["fac1_2".to_string(), "fac2_2".to_string(), "fac3_2".to_string()];
        let total_bytes_2 = 2000;
        
        const ENDPOINT_3: &str = "2.2.2.2:2222";
        let protocols_3: Vec<String> = vec!["fac1_3".to_string(), "fac2_3".to_string(), "fac3_3".to_string()];
        let total_bytes_3 = 3000;
        
        let filter_entries = vec![
            FilterEntryDTO::new(ENDPOINT_1, protocols_1.as_slice(), total_bytes_1),
            FilterEntryDTO::new(ENDPOINT_2, protocols_2.as_slice(), total_bytes_2),
            FilterEntryDTO::new(ENDPOINT_3, protocols_3.as_slice(), total_bytes_3),
        ];

        NetworkOverviewDashboardFiltersDTO::new(filter_entries.as_slice())
    }

    #[test]
    fn reader_correctly_read_encoded_filters() {

        let filters = get_filters();
        let mut binary_user_reader = ReaderBuilder::new().build(filters.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("entries", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();

        let endeced_entries: Vec<FilterEntryDTO> = binary_user_reader.read_all_elements().unwrap().iter().map(|element| {
            FilterEntryDTO::decode(element.as_blob().unwrap())
        }).collect();

        assert_eq!(filters.get_entries(), endeced_entries);

        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_filters() {
        let filters = get_filters();
        assert_eq!(filters, NetworkOverviewDashboardFiltersDTO::decode(&filters.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        let filters = get_filters();
        assert_eq!(filters.get_type(), NetworkOverviewDashboardFiltersDTO::get_data_type());
        assert_eq!(filters.get_type(), super::DATA_TYPE);
    }
}