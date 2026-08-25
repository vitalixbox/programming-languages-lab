pub mod lib_java;

#[derive(Debug, PartialEq, Eq)]
pub enum JobState {
    Queued {
        queued_at: u64,
    },
    Running {
        queued_at: u64,
        started_at: u64,
        worker: String,
    },
    Succeeded {
        queued_at: u64,
        started_at: u64,
        finished_at: u64,
        worker: String,
        artifact: String,
    },
    Failed {
        queued_at: u64,
        started_at: u64,
        finished_at: u64,
        worker: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStateKind {
    Queued,
    Running,
    Succeeded,
    Failed,
}

impl JobState {
    fn kind(&self) -> JobStateKind {
        match self {
            JobState::Queued { .. } => JobStateKind::Queued,
            JobState::Running { .. } => JobStateKind::Running,
            JobState::Succeeded { .. } => JobStateKind::Succeeded,
            JobState::Failed { .. } => JobStateKind::Failed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionAction {
    Start,
    Succeed,
    Fail,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Job {
    id: u64,
    state: JobState,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TransitionError {
    pub attempted: TransitionAction,
    pub current: JobStateKind,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RejectedTransition {
    pub job: Job,
    pub error: TransitionError,
}

fn reject(id: u64, state: JobState, attempted: TransitionAction) -> RejectedTransition {
    let current = state.kind();

    RejectedTransition {
        job: Job { id, state },
        error: TransitionError { attempted, current },
    }
}

impl Job {
    pub fn new(id: u64, queued_at: u64) -> Self {
        Self {
            id,
            state: JobState::Queued { queued_at },
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn state(&self) -> &JobState {
        &self.state
    }

    pub fn start(self, worker: String, started_at: u64) -> Result<Self, RejectedTransition> {
        let Job { id, state } = self;

        match state {
            JobState::Queued { queued_at } => Ok(Job {
                id,
                state: JobState::Running {
                    queued_at,
                    started_at,
                    worker,
                },
            }),
            state => Err(reject(id, state, TransitionAction::Start)),
        }
    }

    pub fn succeed(self, artifact: String, finished_at: u64) -> Result<Self, RejectedTransition> {
        let Job { id, state } = self;

        match state {
            JobState::Running {
                queued_at,
                started_at,
                worker,
            } => Ok(Job {
                id,
                state: JobState::Succeeded {
                    queued_at,
                    started_at,
                    finished_at,
                    worker,
                    artifact,
                },
            }),
            state => Err(reject(id, state, TransitionAction::Succeed)),
        }
    }

    pub fn fail(self, reason: String, finished_at: u64) -> Result<Self, RejectedTransition> {
        let Job { id, state } = self;

        match state {
            JobState::Running {
                queued_at,
                started_at,
                worker,
            } => Ok(Job {
                id,
                state: JobState::Failed {
                    queued_at,
                    started_at,
                    finished_at,
                    worker,
                    reason,
                },
            }),
            state => Err(reject(id, state, TransitionAction::Fail)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queued_job_can_succeed() {
        let job = Job::new(7, 10)
            .start("worker-a".to_owned(), 20)
            .unwrap()
            .succeed("build.tar".to_owned(), 30)
            .unwrap();

        assert_eq!(job.id(), 7);

        match job.state() {
            JobState::Succeeded {
                queued_at,
                started_at,
                finished_at,
                worker,
                artifact,
            } => {
                assert_eq!(*queued_at, 10);
                assert_eq!(*started_at, 20);
                assert_eq!(*finished_at, 30);
                assert_eq!(worker, "worker-a");
                assert_eq!(artifact, "build.tar");
            }
            state => panic!("expected succeeded, got {state:?}"),
        }
    }

    #[test]
    fn runnig_job_can_fail() {
        let job = Job::new(8, 100)
            .start("worker-b".to_owned(), 110)
            .unwrap()
            .fail("compiller crashed".to_owned(), 120)
            .unwrap();

        match job.state() {
            JobState::Failed { reason, .. } => {
                assert_eq!(reason, "compiller crashed");
            }
            state => panic!("expected failed, got {state:?}"),
        }
    }

    #[test]
    fn queued_job_cannot_succeed() {
        let rejection = Job::new(9, 200)
            .succeed("impossible.tar".to_owned(), 210)
            .unwrap_err();

        assert_eq!(
            rejection.error,
            TransitionError {
                attempted: TransitionAction::Succeed,
                current: JobStateKind::Queued,
            }
        );
        assert_eq!(rejection.job.id(), 9);
        assert!(matches!(
            rejection.job.state(),
            JobState::Queued { queued_at: 200 }
        ));
    }

    #[test]
    fn succeeded_job_connot_start_again() {
        let job = Job::new(10, 300)
            .start("worker-c".to_owned(), 310)
            .unwrap()
            .succeed("release.tar".to_owned(), 320)
            .unwrap();

        let rejection = job.start("worker-d".to_owned(), 330).unwrap_err();

        assert_eq!(rejection.error.attempted, TransitionAction::Start);
        assert_eq!(rejection.error.current, JobStateKind::Succeeded);
        assert_eq!(rejection.job.id(), 10);
        assert!(matches!(rejection.job.state(), JobState::Succeeded { .. }));
    }
}
