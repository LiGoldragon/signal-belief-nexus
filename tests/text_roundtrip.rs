//! The ordinary contract written as Datom text and read back, through its public API.
#![cfg(feature = "datom")]

use datom_codec::{Actualizing, Budget, Composing, Datomizable, Potential};
use protos::{Protosizable, ReaderBudget, Textualizable};
use signal_belief_nexus::{
    FurnishRequest, GenerationId, Harness, Kind, ModuleName, Owner, PublicationRevision, Query,
    Refusal, Response, Subject, Workspace, WorkspacePath,
};

fn text<T: Datomizable>(value: &T) -> String {
    value.datomize(Vec::new()).protosize().textualize()
}

fn read<T: Composing>(text: &str) -> T {
    let mut budget = Budget {
        remaining: 4096,
        reader: ReaderBudget::try_from(4096usize).unwrap(),
        depth: 0,
        maximum_depth: 64,
    };
    Potential::<T>::from(text).actualize(&mut budget).expect("read the datom text")
}

fn module(subject: &str) -> ModuleName {
    ModuleName { kind: Kind::Vision, subject: Subject(subject.into()) }
}

#[test]
fn furnish_query_with_scalar_wrappers_round_trips_through_text() {
    let query = Query::Furnish(FurnishRequest {
        workspace: Workspace(WorkspacePath("/tmp/work".into())),
        harness: Harness::Codex,
        module_name_vector: vec![module("nexus"), module("signal")],
    });
    let written = text(&query);
    assert_eq!(read::<Query>(&written), query);
    assert_eq!(text(&read::<Query>(&written)), written);
}

#[test]
fn integer_wrapper_and_string_wrapper_round_trip_through_text() {
    let revision = PublicationRevision(7);
    let generation = GenerationId("sha256:one".into());
    assert_eq!(read::<PublicationRevision>(&text(&revision)), revision);
    assert_eq!(read::<GenerationId>(&text(&generation)), generation);
    assert!(generation < GenerationId("sha256:two".into()));
}

#[test]
fn translator_identity_in_the_contract_round_trips_through_text() {
    let caller = translator::Caller { process_id: translator::ProcessId(42), user_id: translator::UserId(1000) };
    let refusal = Response::Refused(Refusal::NotAuthorized(caller));
    assert_eq!(read::<Response>(&text(&refusal)), refusal);
    let owner = Owner::Peer(translator::PeerId("flow".into()));
    assert_eq!(read::<Owner>(&text(&owner)), owner);
}
