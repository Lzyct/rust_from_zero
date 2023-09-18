#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works_1() -> Result<(),String>{
        if 2 + 2 == 4 {
            Ok(())
        }else{
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
       // Test takes an hour to run
    }

    // #[test]
    // fn failing_test(){
    //     panic!("Make this test fail");
    // }
}