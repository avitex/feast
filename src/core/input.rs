use crate::input::{Input, InputMarker, Requirement};
use crate::pass::{Pass, PassResult, PassSection, PassToken};

pub fn take_input<'p, P>(n: usize) -> impl Fn(P) -> PassResult<'p, P, PassSection<'p, P>>
where
    P: Pass<'p>,
{
    move |pass: P| {
        let input = pass.input();
        let ((taken, rest), pass) = pass.with_input_result(input.split_at(n))?;
        Ok((taken, pass.commit(rest)))
    }
}

pub fn take_input_until<'p, P, F>(pred: F) -> impl Fn(P) -> PassResult<'p, P, PassSection<'p, P>>
where
    P: Pass<'p>,
    F: Fn(&PassToken<'p, P>) -> bool,
{
    move |pass: P| {
        let input = pass.input();
        let mut marker = input.marker();
        loop {
            match marker.next() {
                Some(ref token) if pred(token) => {
                    let mark = marker.mark();
                    let ((taken, rest), pass) = pass.with_input_result(input.split_mark(mark))?;
                    return Ok((taken, pass.commit(rest)));
                }
                Some(_) => continue,
                None => break,
            }
        }
        pass.with_input_error_incomplete(Requirement::Unknown)
    }
}
