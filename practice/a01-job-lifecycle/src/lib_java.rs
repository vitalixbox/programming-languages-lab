//! A deliberately non-idiomatic, object-oriented version of the job lifecycle.
//!
//! This module keeps state and state-specific data in separate fields. Its
//! methods must therefore maintain relationships that the type system does not
//! express.

// Keeping the original ownership-recovery API exposes the larger size of this
// deliberately sparse representation in the error type as well.
#![allow(clippy::result_large_err)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Queued,
    Running,
    Succeeded,
    Failed,
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
    queued_at: Option<u64>,
    started_at: Option<u64>,
    finished_at: Option<u64>,
    worker: Option<String>,
    artifact: Option<String>,
    reason: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TransitionError {
    pub attempted: TransitionAction,
    pub current: JobState,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RejectedTransition {
    pub job: Job,
    pub error: TransitionError,
}

impl Job {
    pub fn new(id: u64, queued_at: u64) -> Self {
        Self {
            id,
            state: JobState::Queued,
            queued_at: Some(queued_at),
            started_at: None,
            finished_at: None,
            worker: None,
            artifact: None,
            reason: None,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn state(&self) -> JobState {
        self.state
    }

    pub fn queued_at(&self) -> Option<u64> {
        self.queued_at
    }

    pub fn started_at(&self) -> Option<u64> {
        self.started_at
    }

    pub fn finished_at(&self) -> Option<u64> {
        self.finished_at
    }

    pub fn worker(&self) -> Option<&str> {
        self.worker.as_deref()
    }

    pub fn artifact(&self) -> Option<&str> {
        self.artifact.as_deref()
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn start(mut self, worker: String, started_at: u64) -> Result<Self, RejectedTransition> {
        if self.state != JobState::Queued {
            return Err(self.reject(TransitionAction::Start));
        }

        self.state = JobState::Running;
        self.started_at = Some(started_at);
        self.worker = Some(worker);

        Ok(self)
    }

    pub fn succeed(
        mut self,
        artifact: String,
        finished_at: u64,
    ) -> Result<Self, RejectedTransition> {
        if self.state != JobState::Running {
            return Err(self.reject(TransitionAction::Succeed));
        }

        self.state = JobState::Succeeded;
        self.finished_at = Some(finished_at);
        self.artifact = Some(artifact);
        self.reason = None;

        Ok(self)
    }

    pub fn fail(mut self, reason: String, finished_at: u64) -> Result<Self, RejectedTransition> {
        if self.state != JobState::Running {
            return Err(self.reject(TransitionAction::Fail));
        }

        self.state = JobState::Failed;
        self.finished_at = Some(finished_at);
        self.reason = Some(reason);
        self.artifact = None;

        Ok(self)
    }

    fn reject(self, attempted: TransitionAction) -> RejectedTransition {
        let current = self.state;

        RejectedTransition {
            job: self,
            error: TransitionError { attempted, current },
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
        assert_eq!(job.state(), JobState::Succeeded);
        assert_eq!(job.queued_at(), Some(10));
        assert_eq!(job.started_at(), Some(20));
        assert_eq!(job.finished_at(), Some(30));
        assert_eq!(job.worker(), Some("worker-a"));
        assert_eq!(job.artifact(), Some("build.tar"));
        assert_eq!(job.reason(), None);
    }

    #[test]
    fn running_job_can_fail() {
        let job = Job::new(8, 100)
            .start("worker-b".to_owned(), 110)
            .unwrap()
            .fail("compiler crashed".to_owned(), 120)
            .unwrap();

        assert_eq!(job.state(), JobState::Failed);
        assert_eq!(job.queued_at(), Some(100));
        assert_eq!(job.started_at(), Some(110));
        assert_eq!(job.finished_at(), Some(120));
        assert_eq!(job.worker(), Some("worker-b"));
        assert_eq!(job.artifact(), None);
        assert_eq!(job.reason(), Some("compiler crashed"));
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
                current: JobState::Queued,
            }
        );
        assert_eq!(rejection.job.id(), 9);
        assert_eq!(rejection.job.state(), JobState::Queued);
        assert_eq!(rejection.job.queued_at(), Some(200));
        assert_eq!(rejection.job.started_at(), None);
        assert_eq!(rejection.job.finished_at(), None);
        assert_eq!(rejection.job.worker(), None);
        assert_eq!(rejection.job.artifact(), None);
        assert_eq!(rejection.job.reason(), None);
    }

    #[test]
    fn succeeded_job_cannot_start_again() {
        let job = Job::new(10, 300)
            .start("worker-c".to_owned(), 310)
            .unwrap()
            .succeed("release.tar".to_owned(), 320)
            .unwrap();

        let rejection = job.start("worker-d".to_owned(), 330).unwrap_err();

        assert_eq!(rejection.error.attempted, TransitionAction::Start);
        assert_eq!(rejection.error.current, JobState::Succeeded);
        assert_eq!(rejection.job.id(), 10);
        assert_eq!(rejection.job.state(), JobState::Succeeded);
        assert_eq!(rejection.job.worker(), Some("worker-c"));
        assert_eq!(rejection.job.artifact(), Some("release.tar"));
    }
}
