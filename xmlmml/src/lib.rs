#[allow(unused, dead_code)]
mod files;
#[allow(unused, dead_code)]
mod parsers;
#[allow(unused, dead_code)]
mod structure;
#[allow(unused, dead_code)]
mod xslt;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn xslt() {
        xslt::xslt2html();
    }
}
