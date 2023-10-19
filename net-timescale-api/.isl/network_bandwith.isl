schema_header::{}

type::{
    name: bandwith_bucket,
    type: struct,
    fields: {
        bucket_timestamp: int,
        total_bytes: int,
    },
}

type::{
    name: network_bandwith,
    type: struct,
    fields: {
        bandwith_buckets: {
            type: list,
            element: {
                type: bandwith_bucket
            },
        },
    },
}

schema_footer::{}