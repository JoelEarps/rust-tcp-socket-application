

async fn main () {
    let ip = "0.0.0.0";
    test_borrow(ip);
        


}

fn test_borrow(ip_address : &str) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use crate::test_borrow;

    #[test]
    fn my_test() {
        assert!(test_borrow("1.2.3.4"));
    }    
}
