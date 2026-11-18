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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RepositoryPath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Repository(pub RepositoryPath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct WorkspacePath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Workspace(pub WorkspacePath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RelativePath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SourceRevision(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PublicationRevision(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct GenerationId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Harness {
    Claude,
    Codex,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Owner {
    Login(translator::UserId),
    Peer(translator::PeerId),
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
pub struct Found(pub std::vec::Vec<ModuleName>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Missing(pub std::vec::Vec<ModuleName>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Resolution {
    pub found: Found,
    pub missing: Missing,
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
pub enum Placement {
    SystemPrompt,
    Skills,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SystemPromptPath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SkillsPath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FurnishedPaths {
    pub system_prompt_path: SystemPromptPath,
    pub skills_path: SkillsPath,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FurnishedWorkspace {
    pub workspace: Workspace,
    pub generation_id: GenerationId,
    pub publication_revision: PublicationRevision,
    pub furnished_paths: FurnishedPaths,
    pub found: Found,
    pub missing: Missing,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ExecutablePath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PeerList(pub std::vec::Vec<translator::PeerBinding>);
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
pub enum InvalidPlacement {
    Unknown(String),
    NonScalar,
    Duplicate,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InvalidModule {
    pub module_name: ModuleName,
    pub relative_path: RelativePath,
    pub invalid_placement: InvalidPlacement,
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
    InvalidModule(InvalidModule),
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
pub enum RefusalKind {
    Source,
    File,
    Memory,
    NotAuthorized,
    SubscriberCapacityReached,
    NotReady,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RefurnishFailure {
    pub workspace: Workspace,
    pub refusal_kind: RefusalKind,
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
pub struct Workspaces(pub std::vec::Vec<FurnishedWorkspace>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Furnishing {
    Furnished(FurnishedWorkspace),
    RefurnishFailed(RefurnishFailure),
    CleanupFailed(CleanupFailure),
    MemoryAfterPublication(MemoryAfterPublication),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Refusal {
    Source(SourceFailure),
    File(FileFailure),
    Memory(MemoryFailure),
    NotAuthorized(translator::Caller),
    SubscriberCapacityReached(i64),
    NotReady,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Input {
    Register(Repository),
    Resolve(std::vec::Vec<ModuleName>),
    Furnish(FurnishRequest),
    Subscribe,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Output {
    Registered(RepositoryState),
    Resolved(Resolution),
    Furnished(FurnishedWorkspace),
    Subscribed(signal::Frame<Workspaces, Furnishing>),
    MemoryAfterPublication(MemoryAfterPublication),
    Refused(Refusal),
}
#[rustfmt::skip]
#[allow(clippy::large_enum_variant)]
pub enum Answer {
    Once(Output),
    Subscribed(signal::Stream<Workspaces, Furnishing>),
}
#[rustfmt::skip]
#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
pub trait Answering {
    fn register(
        &self,
        repository: &Repository,
    ) -> std::result::Result<RepositoryState, Output>;
    fn resolve(
        &self,
        module_name_vector: &std::vec::Vec<ModuleName>,
    ) -> std::result::Result<Resolution, Output>;
    fn furnish(
        &self,
        furnish_request: &FurnishRequest,
    ) -> std::result::Result<FurnishedWorkspace, Output>;
    fn subscribe(
        &self,
    ) -> std::result::Result<signal::Stream<Workspaces, Furnishing>, Output>;
}
#[rustfmt::skip]
pub trait Answerable {
    type Answer;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Self::Answer;
}
#[rustfmt::skip]
impl Answerable for Input {
    type Answer = Answer;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Answer {
        match self {
            Input::Register(carried) => {
                Answer::Once(
                    nexus
                        .register(carried)
                        .map_or_else(|refusal| refusal, Output::Registered),
                )
            }
            Input::Resolve(carried) => {
                Answer::Once(
                    nexus
                        .resolve(carried)
                        .map_or_else(|refusal| refusal, Output::Resolved),
                )
            }
            Input::Furnish(carried) => {
                Answer::Once(
                    nexus
                        .furnish(carried)
                        .map_or_else(|refusal| refusal, Output::Furnished),
                )
            }
            Input::Subscribe => {
                nexus.subscribe().map_or_else(Answer::Once, Answer::Subscribed)
            }
        }
    }
}
const ETHOS: &str = include_str!("../signal.ethos");

impl signal::Contracted for Input {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}

/// A subscription's state follows each furnishing: a workspace furnished anew
/// replaces the one held for it, or is taken in when none is held; a refused
/// refurnish, a failed cleanup and a memory failure after publication leave
/// the furnished workspaces as they are.
impl signal::Following for Workspaces {
    type Change = Furnishing;

    fn after(workspaces: Self, furnishing: Furnishing) -> Self {
        let Workspaces(mut furnished) = workspaces;
        if let Furnishing::Furnished(workspace) = furnishing {
            match furnished
                .iter()
                .position(|held| held.workspace == workspace.workspace)
            {
                Some(at) => furnished[at] = workspace,
                None => {
                    furnished.push(workspace);
                    furnished.sort_by_key(|held| held.workspace.0 .0.clone());
                }
            }
        }
        Workspaces(furnished)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal::{ByteViewable, Following, Frame, Restorable, Signal, Signalizable};

    fn furnished(path: &str, revision: i64) -> FurnishedWorkspace {
        FurnishedWorkspace {
            workspace: Workspace(WorkspacePath(path.into())),
            generation_id: GenerationId("sha256:one".into()),
            publication_revision: PublicationRevision(revision),
            furnished_paths: FurnishedPaths {
                system_prompt_path: SystemPromptPath(format!(
                    "{path}/.belief/current/model-instructions.md"
                )),
                skills_path: SkillsPath(format!("{path}/.agents/skills")),
            },
            found: Found(vec![ModuleName {
                kind: Kind::Vision,
                subject: Subject("nexus".into()),
            }]),
            missing: Missing(vec![ModuleName {
                kind: Kind::Knowledge,
                subject: Subject("missing".into()),
            }]),
        }
    }

    #[test]
    fn inputs_and_outputs_restore_through_validated_signal() {
        let input = Input::Furnish(FurnishRequest {
            workspace: Workspace(WorkspacePath("/tmp/work".into())),
            harness: Harness::Codex,
            module_name_vector: vec![ModuleName {
                kind: Kind::Vision,
                subject: Subject("nexus".into()),
            }],
        });
        let archived = input.signalize().expect("archive public input");
        assert_eq!(
            Signal::<Input>::from(archived.bytes().to_vec())
                .restore()
                .unwrap(),
            input
        );

        let output = Output::Furnished(furnished("/tmp/work", 7));
        let archived = output.signalize().expect("archive public output");
        assert_eq!(
            Signal::<Output>::from(archived.bytes().to_vec())
                .restore()
                .unwrap(),
            output
        );
    }

    #[test]
    fn every_subscription_frame_restores_through_validated_signal() {
        for frame in [
            Frame::Opened(Workspaces(vec![furnished("/tmp/work", 7)])),
            Frame::Changed(Furnishing::Furnished(furnished("/tmp/work", 8))),
            Frame::Changed(Furnishing::RefurnishFailed(RefurnishFailure {
                workspace: Workspace(WorkspacePath("/tmp/work".into())),
                refusal_kind: RefusalKind::Source,
            })),
            Frame::Lagged(Workspaces(Vec::new())),
        ] {
            let output = Output::Subscribed(frame);
            let archived = output.signalize().expect("archive subscription frame");
            assert_eq!(
                Signal::<Output>::from(archived.bytes().to_vec())
                    .restore()
                    .unwrap(),
                output
            );
        }
    }

    #[test]
    fn a_workspace_furnished_anew_replaces_the_one_held() {
        let state = Workspaces(vec![furnished("/tmp/a", 1), furnished("/tmp/b", 1)]);
        let state = Workspaces::after(state, Furnishing::Furnished(furnished("/tmp/b", 2)));
        assert_eq!(
            state,
            Workspaces(vec![furnished("/tmp/a", 1), furnished("/tmp/b", 2)])
        );
    }

    #[test]
    fn a_workspace_not_yet_held_is_taken_in_in_path_order() {
        let state = Workspaces(vec![furnished("/tmp/c", 1)]);
        let state = Workspaces::after(state, Furnishing::Furnished(furnished("/tmp/a", 1)));
        assert_eq!(
            state,
            Workspaces(vec![furnished("/tmp/a", 1), furnished("/tmp/c", 1)])
        );
    }

    #[test]
    fn a_failure_leaves_the_furnished_workspaces_as_they_are() {
        let state = Workspaces(vec![furnished("/tmp/a", 1)]);
        let after = Workspaces::after(
            state.clone(),
            Furnishing::RefurnishFailed(RefurnishFailure {
                workspace: Workspace(WorkspacePath("/tmp/a".into())),
                refusal_kind: RefusalKind::File,
            }),
        );
        assert_eq!(after, state);
    }

    #[test]
    fn configured_peer_binding_carries_shared_identity() {
        let binding = translator::PeerBinding {
            peer_id: translator::PeerId("flow".into()),
            peer_identity: translator::PeerIdentity {
                user_id: translator::UserId(1000),
                process_id: translator::ProcessId(42),
            },
        };
        let caller = translator::Caller {
            process_id: binding.peer_identity.process_id,
            user_id: binding.peer_identity.user_id,
        };
        assert_eq!(binding.peer_id, translator::PeerId("flow".into()));
        assert_eq!(caller.process_id, translator::ProcessId(42));
    }
}
