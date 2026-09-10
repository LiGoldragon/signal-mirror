use signal_mirror::{HeadQuery, Query, Response};

#[test]
fn query_and_response_archive_with_current_named_heads() {
    let query = Query::ObserveHeads(HeadQuery::None);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&query).expect("archive query");
    assert_eq!(
        rkyv::from_bytes::<Query, rkyv::rancor::Error>(&bytes).expect("restore query"),
        query
    );

    let response = Response::HeadsObserved(signal_mirror::HeadListing {
        store_head_vector: vec![],
    });
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&response).expect("archive response");
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&bytes).expect("restore response"),
        response
    );
}
