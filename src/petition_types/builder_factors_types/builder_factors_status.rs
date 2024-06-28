/// The status of building using a certain list of factors, e.g. threshold or
/// override factors list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuilderFactorsStatus {
    /// In progress, still gathering output from factors (signatures or public keys).
    InProgress,

    /// Finished building with factors, either successfully or failed.
    Finished(BuilderFactorsStatusFinished),
}

/// Finished building with factors, either successfully or failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuilderFactorsStatusFinished {
    /// Successful completion of building with factors.
    Success,

    /// Failure building with factors, either a simulated status, as in what
    /// would happen if we skipped a factor source, or a real failure, as in,
    /// the user explicitly chose to skip a factor source even though she was
    /// advised it would result in some transaction failing. Or we failed to
    /// use a required factor source for what some reason.
    Fail,
}
