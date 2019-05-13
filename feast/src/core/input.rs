use crate::input::{Input, InputMarker, Requirement};
use crate::pass::{Pass, PassResult, PassSection, PassToken};

pub fn take_input<'i, P>(n: usize) -> impl Fn(P) -> PassResult<'i, P, PassSection<'i, P>>
where
    P: Pass<'i>,
{
    move |pass: P| {
        let input = pass.input();
        let ((taken, rest), pass) = pass.with_input_result(input.split_at(n))?;
        Ok((taken, pass.commit(rest)))
    }
}

pub fn take_input_until<'i, P, F>(pred: F) -> impl Fn(P) -> PassResult<'i, P, PassSection<'i, P>>
where
    P: Pass<'i>,
    F: Fn(&PassToken<'i, P>) -> bool,
{
    move |pass: P| {
        let input = pass.input();
        let mut marker = input.iter();
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
        Err(pass.with_input_error_incomplete(Requirement::Unknown))
    }
}
