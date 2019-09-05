use super::{IntoStage, Stage, Pass, Capture, Error};

#[derive(Debug)]
pub struct With<A, B>(A, B)
where
    A: Stage,
    B: Stage;

impl<A, B> With<A, B>
where
    A: Stage,
    B: Stage,
{
    pub fn new<S>(a: A, s: S) -> Self
    where
        S: IntoStage<Stage = B>,
    {
        Self(a, s.into_stage())
    }
}

impl<A, B> Stage for With<A, B>
where
    A: Stage<P,
    B: Stage<Error = A::Error>,
{
    type

    fn run(&self, pass: Pass) -> Result<(((), ()), Pass), Error> {
        self.0.run(pass)
            .and_then(move |(cap_a, pass)| {
                self.1.run(pass)
                    .map(move |(cap_b, pass)| {
                        ((cap_a, cap_b), pass)
                    })
            })
    }
}
