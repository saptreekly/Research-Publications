use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlockKind {
    Brief {
        id: String,
        title: Option<String>,
        body_md: String,
    },
    Probe {
        id: String,
        params: Vec<ProbeParam>,
    },
    Blueprint {
        id: String,
        language: String,
        code: String,
    },
    Verify {
        id: String,
        cases: Vec<VerifyCase>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProbeParam {
    pub name: String,
    pub value: i64,
    pub min: i64,
    pub max: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifyCase {
    pub expression: String,
    pub function: String,
    pub args: Vec<VerifyArg>,
    pub expected: VerifyExpectation,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VerifyArg {
    Literal(i64),
    Probe(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VerifyExpectation {
    Value(i64),
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Idle,
    Running,
    Success,
    Error,
}

impl Default for ExecutionStatus {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifyResult {
    pub expression: String,
    pub passed: bool,
    pub expected: String,
    pub got: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BlockExecution {
    pub status: ExecutionStatus,
    pub stdout: Vec<String>,
    pub result: Option<String>,
    pub error: Option<String>,
    pub verify_results: Vec<VerifyResult>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabBlock {
    pub kind: BlockKind,
    pub execution: BlockExecution,
}

impl LabBlock {
    pub fn id(&self) -> &str {
        match &self.kind {
            BlockKind::Brief { id, .. } => id,
            BlockKind::Probe { id, .. } => id,
            BlockKind::Blueprint { id, .. } => id,
            BlockKind::Verify { id, .. } => id,
        }
    }

    pub fn type_label(&self) -> &'static str {
        match &self.kind {
            BlockKind::Brief { .. } => "BRIEF",
            BlockKind::Probe { .. } => "PROBE",
            BlockKind::Blueprint { .. } => "BLUEPRINT",
            BlockKind::Verify { .. } => "VERIFY",
        }
    }

    pub fn is_runnable(&self) -> bool {
        matches!(self.kind, BlockKind::Blueprint { .. })
    }

    pub fn is_verifiable(&self) -> bool {
        matches!(self.kind, BlockKind::Verify { .. })
    }
}

pub fn collect_probe_values(blocks: &[LabBlock]) -> Vec<(String, i64)> {
    let mut values = Vec::new();
    for block in blocks {
        if let BlockKind::Probe { params, .. } = &block.kind {
            for param in params {
                values.push((param.name.clone(), param.value));
            }
        }
    }
    values
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct KernelSession {
    pub variables: Vec<(String, String)>,
    pub kernel_label: String,
    pub last_run_ms: Option<u32>,
}

impl KernelSession {
    pub fn local_stub() -> Self {
        Self {
            kernel_label: "LOCAL // STUB".to_string(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabModule {
    pub id: String,
    pub title: String,
    pub blocks: Vec<LabBlock>,
}
