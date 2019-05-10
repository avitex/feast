use crate::pass::{Pass, PassResult};

pub fn hint<'p, P, F, O>(inner: F, _description: &'static str) -> impl Fn(P) -> PassResult<'p, P, O>
where
    P: Pass<'p>,
    F: Fn(P) -> PassResult<'p, P, O>,
{
    move |pass: P| {
        match inner(pass) {
            Err((err, pass)) => Err((err, pass)), // TODO: Wrap with hint description
            ok => ok,
        }
    }
}