use signal_mirror::*;

fn requests() -> Vec<(z2VVny, &'static str)> {
    vec![
        (
            z2VVny::z2VVjQ(z2VTq5 {
                field_0: z2Ve8p::new("fixture".to_owned()),
                field_1: Some(z2VcqM {
                    field_0: z2VSAK::new(7),
                    field_1: signal_standard::schema::lib::z2VSyM::new("blake3:aab".to_owned()),
                }),
                field_2: vec![z2VPuU {
                    field_0: z2VSAK::new(7),
                    field_1: Some(signal_standard::schema::lib::z2VSyM::new(
                        "blake3:aab".to_owned(),
                    )),
                    field_2: signal_standard::schema::lib::z2VSyM::new("blake3:aab".to_owned()),
                    field_3: z2VUwg::new(vec![7]),
                }],
            }),
            "Append",
        ),
        (
            z2VVny::z2VaYk(z2VZWt {
                field_0: z2Ve8p::new("fixture".to_owned()),
                field_1: z2VcqM {
                    field_0: z2VSAK::new(7),
                    field_1: signal_standard::schema::lib::z2VSyM::new("blake3:aab".to_owned()),
                },
                field_2: Some(signal_standard::schema::lib::z2VduW::z2VNCH(
                    signal_standard::schema::lib::z2VaVE {
                        field_0: signal_standard::schema::lib::z2VLyh::new("mirror-aab".to_owned()),
                        field_1: signal_standard::schema::lib::z2VQaE::new(7476),
                    },
                )),
            }),
            "NotifyObject",
        ),
        (
            z2VVny::z2VNu6(z2VTXE {
                field_0: z2Ve8p::new("fixture".to_owned()),
                field_1: z2VUKn::new(7),
                field_2: z2VSAK::new(7),
                field_3: signal_standard::schema::lib::z2VSyM::new("blake3:aab".to_owned()),
                field_4: z2VUxk::new(vec![7]),
            }),
            "PublishCheckpoint",
        ),
        (
            z2VVny::z2VZ8E(z2Vdqa::new(Some(z2Ve8p::new("fixture".to_owned())))),
            "ObserveHeads",
        ),
        (
            z2VVny::z2VdHF(z2VbvA::new(z2Ve8p::new("fixture".to_owned()))),
            "Restore",
        ),
    ]
}

fn replies() -> Vec<(z2VTqL, &'static str)> {
    vec![
        (
            z2VTqL::z2VLCz(z2VbBN {
                field_0: z2Ve8p::new("fixture".to_owned()),
                field_1: z2VPgu::z2VXM2,
            }),
            "RestoreRejected",
        ),
        (
            z2VTqL::z2VMR1(z2VY7x {
                field_0: vec![z2Vbm6 {
                    field_0: z2Ve8p::new("fixture".to_owned()),
                    field_1: Some(z2VcqM {
                        field_0: z2VSAK::new(7),
                        field_1: signal_standard::schema::lib::z2VSyM::new("blake3:aab".to_owned()),
                    }),
                }],
            }),
            "HeadsObserved",
        ),
        (
            z2VTqL::z2VWHb(z2VbP4 {
                field_0: z2Ve8p::new("fixture".to_owned()),
                field_1: z2Vcs2::z2VWLf,
            }),
            "PublishRejected",
        ),
        (
            z2VTqL::z2VR8x(z2VWFj {
                field_0: z2Ve8p::new("fixture".to_owned()),
                field_1: z2VcqM {
                    field_0: z2VSAK::new(7),
                    field_1: signal_standard::schema::lib::z2VSyM::new("blake3:aab".to_owned()),
                },
            }),
            "ObjectNoticeAccepted",
        ),
        (
            z2VTqL::z2VPpj(z2Vc3D::new(z2VMCw::new("fixture".to_owned()))),
            "MirrorFaulted",
        ),
        (
            z2VTqL::z2VSxB(z2VQTe {
                field_0: z2Ve8p::new("fixture".to_owned()),
                field_1: z2VdLR::z2VZJ4,
                field_2: Some(z2VcqM {
                    field_0: z2VSAK::new(7),
                    field_1: signal_standard::schema::lib::z2VSyM::new("blake3:aab".to_owned()),
                }),
            }),
            "ObjectNoticeRejected",
        ),
        (
            z2VTqL::z2VXSq(z2VaxY {
                field_0: z2Ve8p::new("fixture".to_owned()),
                field_1: z2VcqM {
                    field_0: z2VSAK::new(7),
                    field_1: signal_standard::schema::lib::z2VSyM::new("blake3:aab".to_owned()),
                },
            }),
            "Appended",
        ),
        (
            z2VTqL::z2VaSa(z2VLxP {
                field_0: z2Ve8p::new("fixture".to_owned()),
                field_1: z2VUKn::new(7),
                field_2: z2VSAK::new(7),
            }),
            "CheckpointPublished",
        ),
        (
            z2VTqL::z2VVve(z2VYSu {
                field_0: z2Ve8p::new("fixture".to_owned()),
                field_1: z2VTXE {
                    field_0: z2Ve8p::new("fixture".to_owned()),
                    field_1: z2VUKn::new(7),
                    field_2: z2VSAK::new(7),
                    field_3: signal_standard::schema::lib::z2VSyM::new("blake3:aab".to_owned()),
                    field_4: z2VUxk::new(vec![7]),
                },
                field_2: vec![z2VPuU {
                    field_0: z2VSAK::new(7),
                    field_1: Some(signal_standard::schema::lib::z2VSyM::new(
                        "blake3:aab".to_owned(),
                    )),
                    field_2: signal_standard::schema::lib::z2VSyM::new("blake3:aab".to_owned()),
                    field_3: z2VUwg::new(vec![7]),
                }],
            }),
            "Restored",
        ),
        (
            z2VTqL::z2VX2Y(z2VUTH {
                field_0: z2Ve8p::new("fixture".to_owned()),
                field_1: z2VcyE::z2VcFn,
                field_2: Some(z2VcqM {
                    field_0: z2VSAK::new(7),
                    field_1: signal_standard::schema::lib::z2VSyM::new("blake3:aab".to_owned()),
                }),
            }),
            "AppendRejected",
        ),
    ]
}

fn exchange(epoch: u64) -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(epoch),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::first(),
    )
}

#[test]
fn every_request_round_trips_through_the_bound_frame() {
    for (request, _head) in requests() {
        let expected = request.clone();
        let encoded = request
            .encode_request_frame(exchange(51))
            .expect("request frame encodes");
        let (decoded_exchange, decoded) =
            ContractMarker::decode_single_request(&encoded).expect("request frame decodes");
        assert_eq!(decoded_exchange, exchange(51));
        assert_eq!(decoded, expected);
    }
}

#[test]
fn every_reply_has_bound_frame_and_rkyv_behavior() {
    for (reply, _head) in replies() {
        let expected = reply.clone();
        let encoded = reply
            .clone()
            .encode_reply_frame(exchange(53))
            .expect("reply frame encodes");
        ContractMarker::decode_frame(&encoded).expect("reply frame decodes");
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).expect("reply archives");
        let recovered =
            rkyv::from_bytes::<z2VTqL, rkyv::rancor::Error>(&archive).expect("reply recovers");
        assert_eq!(recovered, expected);
    }
}

#[test]
fn opaque_payload_and_artifact_vectors_validate_as_octets() {
    let payload = z2VUwg::from_octets(&[0, 127, 255]);
    let artifact = z2VUxk::from_octets(&[1, 2, 3]);
    assert_eq!(payload.octets().expect("payload octets"), [0, 127, 255]);
    assert_eq!(artifact.octets().expect("artifact octets"), [1, 2, 3]);

    let invalid = z2VUwg::new(vec![256]);
    let error = invalid.octets().expect_err("integer is not an octet");
    assert_eq!(error.offset, 0);
    assert_eq!(error.value, 256);
}

#[cfg(feature = "dotos-text")]
#[test]
fn every_root_round_trips_through_dotos_with_visible_heads() {
    use dotos::{DotosEncode, DotosSource};
    for (request, head) in requests() {
        let text = request.to_dotos();
        assert!(text.starts_with(&format!("{head}.")), "{text}");
        assert_eq!(
            DotosSource::new(&text)
                .parse::<z2VVny>()
                .expect("request Dotos decodes"),
            request
        );
    }
    for (reply, head) in replies() {
        let text = reply.to_dotos();
        assert!(text.starts_with(&format!("{head}.")), "{text}");
        assert_eq!(
            DotosSource::new(&text)
                .parse::<z2VTqL>()
                .expect("reply Dotos decodes"),
            reply
        );
    }
}
