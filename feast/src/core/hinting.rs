use crate::pass::{Pass, PassResult};

pub fn hint<'i, P, F, O>(sub: F, _description: &'static str) -> impl Fn(P) -> PassResult<'i, P, O>
where
    P: Pass<'i>,
    F: Fn(P) -> PassResult<'i, P, O>,
{
    move |pass: P| {
        match sub(pass) {
            Err((err, pass)) => Err((err, pass)), // TODO: Wrap with hint description
            ok => ok,
        }
    }
}
