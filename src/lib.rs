#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Kind {
    Spirit,
    Intent,
    Vision,
    Knowledge,
    Role,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Subject(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ModuleName {
    pub kind: Kind,
    pub subject: Subject,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RepositoryPath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Repository(pub RepositoryPath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct WorkspacePath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Workspace(pub WorkspacePath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RelativePath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SourceRevision(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PublicationRevision(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct GenerationId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Harness {
    Codex,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RepositoryState {
    pub repository: Repository,
    pub source_revision: SourceRevision,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Found(pub ModuleName);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Resolution {
    pub found_vector: std::vec::Vec<Found>,
    pub module_name_vector: std::vec::Vec<ModuleName>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FurnishRequest {
    pub workspace: Workspace,
    pub harness: Harness,
    pub module_name_vector: std::vec::Vec<ModuleName>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SystemPrompt(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Skills(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Placement {
    pub system_prompt: SystemPrompt,
    pub skills: Skills,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FurnishedWorkspace {
    pub workspace: Workspace,
    pub generation_id: GenerationId,
    pub publication_revision: PublicationRevision,
    pub placement: Placement,
    pub found_vector: std::vec::Vec<Found>,
    pub module_name_vector: std::vec::Vec<ModuleName>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BeliefState {
    pub repository_state_vector: std::vec::Vec<RepositoryState>,
    pub furnished_workspace_vector: std::vec::Vec<FurnishedWorkspace>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Change {
    pub repository: Repository,
    pub source_revision: SourceRevision,
    pub module_name_vector: std::vec::Vec<ModuleName>,
    pub furnished_workspace_vector: std::vec::Vec<FurnishedWorkspace>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UserId(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct GroupId(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ProcessId(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ExecutablePath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ContractDigest(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PeerName(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Caller {
    pub user_id: UserId,
    pub group_id: GroupId,
    pub process_id: ProcessId,
    pub executable_path: ExecutablePath,
    pub contract_digest: ContractDigest,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PeerIdentity {
    pub peer_name: PeerName,
    pub user_id: UserId,
    pub group_id: GroupId,
    pub executable_path: ExecutablePath,
    pub contract_digest: ContractDigest,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PeerList(pub std::vec::Vec<PeerIdentity>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Call {
    pub caller: Caller,
    pub query: Query,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SourceOperation {
    Registering,
    Snapshotting,
    Resolving,
    Watching,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum FileOperation {
    Reading,
    Staging,
    Linking,
    Replacing,
    Inspecting,
    Cleaning,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MemoryOperation {
    Loading,
    Recording,
    BeginningPublication,
    CompletingPublication,
    AfterPublication,
    Recovering,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ExitStatus(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Diagnostic {
    pub executable_path: ExecutablePath,
    pub exit_status_option: std::option::Option<ExitStatus>,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Ambiguous_Data {
    pub module_name: ModuleName,
    pub repository_vector: std::vec::Vec<Repository>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SourceEvidence {
    NotRepository,
    MainUnavailable,
    RevisionUnavailable,
    Ambiguous(Ambiguous_Data),
    Translator(Diagnostic),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SourceFailure {
    pub repository: Repository,
    pub source_operation: SourceOperation,
    pub source_evidence: SourceEvidence,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OutsideBoundary_Data {
    pub first_relative_path: RelativePath,
    pub second_relative_path: RelativePath,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Conflict_Data {
    pub first_publication_revision_option: std::option::Option<PublicationRevision>,
    pub second_publication_revision_option: std::option::Option<PublicationRevision>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum FileEvidence {
    OutsideBoundary(OutsideBoundary_Data),
    PermissionDenied,
    MissingRequired(RelativePath),
    Conflict(Conflict_Data),
    Translator(Diagnostic),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FileFailure {
    pub workspace: Workspace,
    pub file_operation: FileOperation,
    pub file_evidence: FileEvidence,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MemoryEvidence {
    Unavailable,
    Corrupt,
    Conflict,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MemoryFailure {
    pub memory_operation: MemoryOperation,
    pub memory_evidence: MemoryEvidence,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct WatchFailure {
    pub repository: Repository,
    pub source_revision_option: std::option::Option<SourceRevision>,
    pub source_failure: SourceFailure,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RefurnishFailure {
    pub workspace: Workspace,
    pub repository: Repository,
    pub source_revision: SourceRevision,
    pub refusal: Refusal,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CleanupFailure {
    pub workspace: Workspace,
    pub generation_id: GenerationId,
    pub publication_revision: PublicationRevision,
    pub file_failure: FileFailure,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MemoryAfterPublication {
    pub workspace: Workspace,
    pub generation_id: GenerationId,
    pub publication_revision: PublicationRevision,
    pub memory_failure: MemoryFailure,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Refusal {
    Source(SourceFailure),
    File(FileFailure),
    Memory(MemoryFailure),
    NotAuthorized(Caller),
    SubscriberCapacityReached(i64),
    NotReady,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Register(Repository),
    Resolve(std::vec::Vec<ModuleName>),
    Furnish(FurnishRequest),
    Subscribe,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Registered(RepositoryState),
    Resolved(Resolution),
    Furnished(FurnishedWorkspace),
    Subscribed(BeliefState),
    Changed(Change),
    WatchFailed(WatchFailure),
    RefurnishFailed(RefurnishFailure),
    CleanupFailed(CleanupFailure),
    MemoryAfterPublication(MemoryAfterPublication),
    Refused(Refusal),
}
pub const ETHOS: &str = include_str!("../signal.ethos");

impl signal::Contracted for Query {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal::{ByteViewable, Restorable, Signal, Signalizable};

    #[test]
    fn calls_and_subscription_state_restore_through_validated_signal() {
        let query = Query::Furnish(FurnishRequest {
            workspace: Workspace(WorkspacePath("/tmp/work".into())),
            harness: Harness::Codex,
            module_name_vector: vec![ModuleName {
                kind: Kind::Vision,
                subject: Subject("nexus".into()),
            }],
        });
        let archived = query.signalize().expect("archive public query");
        assert_eq!(Signal::<Query>::from(archived.bytes().to_vec()).restore().unwrap(), query);

        let response = Response::Subscribed(BeliefState {
            repository_state_vector: vec![RepositoryState {
                repository: Repository(RepositoryPath("/src/belief".into())),
                source_revision: SourceRevision("abc123".into()),
            }],
            furnished_workspace_vector: vec![FurnishedWorkspace {
                workspace: Workspace(WorkspacePath("/tmp/work".into())),
                generation_id: GenerationId("sha256:one".into()),
                publication_revision: PublicationRevision(7),
                placement: Placement {
                    system_prompt: SystemPrompt("/tmp/work/.belief/current/model-instructions.md".into()),
                    skills: Skills("/tmp/work/.agents/skills".into()),
                },
                found_vector: vec![Found(ModuleName { kind: Kind::Vision, subject: Subject("nexus".into()) })],
                module_name_vector: vec![ModuleName { kind: Kind::Knowledge, subject: Subject("missing".into()) }],
            }],
        });
        let archived = response.signalize().expect("archive public response");
        assert_eq!(Signal::<Response>::from(archived.bytes().to_vec()).restore().unwrap(), response);
    }

    #[test]
    fn configured_peer_identity_excludes_observed_process_id() {
        let peer = PeerIdentity {
            peer_name: PeerName("flow".into()),
            user_id: UserId(1000),
            group_id: GroupId(1000),
            executable_path: ExecutablePath("/nix/store/flow/bin/flow".into()),
            contract_digest: ContractDigest("sha256:contract".into()),
        };
        let caller = Caller {
            user_id: peer.user_id.clone(),
            group_id: peer.group_id.clone(),
            process_id: ProcessId(42),
            executable_path: peer.executable_path.clone(),
            contract_digest: peer.contract_digest.clone(),
        };
        assert_eq!(peer.peer_name, PeerName("flow".into()));
        assert_eq!(caller.process_id, ProcessId(42));
    }
}
