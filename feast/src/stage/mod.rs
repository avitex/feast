//mod with;

//use crate::pass::Pass;
// Fn(Pass) -> Result<(Capture, Pass),  Error>
//pub use self::with::With;

pub trait Error {
}

pub trait Input {
}

pub trait Capture {}
pub trait CaptureValue {}

pub trait Capture {
    type Value;

    fn build<V, C>(val: V) -> C
        where C: Capture<Value = V>;
}

pub trait Pass {
    type Error: Error;
    type Input: Input;
    type Capture: Capture;
}

type PassError<P> = <P as Pass>::Error;
type PassCapture<P, V> = <P as Pass>::Capture<Value = V>;

type StageResult<P, V> = Result<(PassCapture<P, V>, P), PassError<P>>;


/// Stages take a pass
#[must_use = "stages do nothing unless run"]
pub trait Stage: Sized {

    type Pass: Pass;
    type Value;

    fn run(&self, pass: Self::Pass) -> StageResult<Self::Pass, Self::Value>;

    // ///
    // fn with<S>(self, s: S) -> With<Self, S>
    // where
    //     S: Stage,
    // {
    //     With::new(self, s)
    // }
}

impl<P, V> Stage for fn(P) -> StageResult<P, V>
where
    P: Pass,
{
    type Pass = P;
    type Value = V;

    fn run(&self, pass: Self::Pass) -> StageResult<Self::Pass, Self::Value> {
        self(pass)
    }
}

pub trait IntoStage: Sized {
    type Stage: Stage;

    fn into_stage(self) -> Self::Stage;
}

impl<S> IntoStage for S
where
    S: Stage,
{
    type Stage = S;

    fn into_stage(self) -> Self::Stage {
        self
    }
}

// #[test]
// fn test_stage() {
//     let a = |()| {
//         Ok(((), ()))
//     };
//     let b = |()| {
//         Ok(((), ()))
//     };
//     a.with(b);
// }
