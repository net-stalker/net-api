schema_header::{}

type::{
    name: bandwidth_bucket,
    type: struct,
    fields: {
        bucket_timestamp: int,
        total_bytes: int,
    },
}

type::{
    name: network_bandwidth,
    type: struct,
    fields: {
        bandwidth_buckets: {
            type: list,
            element: {
                type: bandwidth_bucket
            },
        },
    },
}

schema_footer::{}