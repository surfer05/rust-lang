use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn find_nth<T: Ord + Clone>(elems: &mut [T], n: usize) -> T {
    elems.sort();
    let t = &elems[n];
    return t.clone();
}

fn main() {
    println!("Hello World!");
}

struct TestResult {
    /// Student's scores on a test
    scores: Vec<usize>,

    /// A possible value to curve all scores
    curve: Option<usize>
}
impl TestResult {  
    pub fn get_curve(&self) -> &Option<usize> { 
        &self.curve 
    }

    /// If there is a curve, then increments all 
    /// scores by the curve
    pub fn apply_curve(&mut self) {
        if let Some(curve) = self.get_curve() {
            for score in self.scores.iter_mut() {
                *score += *curve;
            }
        }
    }
}