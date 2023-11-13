use cache_this::cache_this;

#[test]
fn test_cache_this() {
    fn some_function(execute_increment: &mut u8) -> Vec<i32> {
        *execute_increment += 1;
        vec![83947, 78239, 47384, 83439, 23213, 47384]
    }

    let mut execution_counter = 0;

    let first_execution_result = cache_this!(some_function(&mut execution_counter));

    assert_eq!(1, execution_counter);
    assert_eq!(vec![83947, 78239, 47384, 83439, 23213, 47384], first_execution_result);

    let second_execution_result = cache_this!(some_function(&mut execution_counter));

    // execution counter is still 1 as the functions body was not executed but the cached result is returned
    assert_eq!(1, execution_counter);
    assert_eq!(vec![83947, 78239, 47384, 83439, 23213, 47384], second_execution_result);

    // clean up cached files
    std::fs::remove_dir_all(".cache-this").unwrap();
}
