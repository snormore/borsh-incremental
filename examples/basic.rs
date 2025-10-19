use borsh::{to_vec, BorshSerialize};
use borsh_incremental::BorshDeserializeIncremental;

#[derive(BorshSerialize, BorshDeserializeIncremental, Debug, PartialEq)]
struct ExampleV1 {
    #[incremental(default = "Unnamed".to_string())]
    name: String,
    count: u32,
}

#[derive(BorshDeserializeIncremental, Debug, PartialEq)]
struct ExampleV2 {
    #[incremental(default = "Unnamed".to_string())]
    name: String,
    count: u32,
    #[incremental(default = "active".to_string())]
    status: String, // new field added in the next version
}

fn main() {
    // Encode data using the old V1 struct (no 'status' field)
    let old = ExampleV1 {
        name: "Alice".into(),
        count: 5,
    };
    let old_bytes = to_vec(&old).unwrap();

    // Decode using the new V2 struct — the missing field gets its default
    let parsed_v2 = ExampleV2::try_from(&old_bytes[..]).unwrap();
    println!("Backward-compatible decode: {:?}", parsed_v2);

    // Simulate truncated input — only part of the serialized name
    let truncated = &old_bytes[..4];
    let partial = ExampleV2::try_from(truncated).unwrap();
    println!("Partial decode with defaults: {:?}", partial);
}
