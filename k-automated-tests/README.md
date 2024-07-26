### Automated Tests

Tests are pieces of code testing other pieces of code. The tests usually performs three actions:
* Setup data and/or state;
* Run the code to be tested;
* Assert the results

A test function is annotated with `#[test]` attribute right before the function signature. The command `cargo test` will run all functions identified with the thest annotation and report the errors.

#### Unit Tests

Unit tests are used to test small pieces of the code isolated from the rest. This way, is easier to determinate where's the failing point. The unit test codes are put in the same file as the src code, that's why the annotation `#[cfg(test)]` needs to be used.

#### Integration Tests

These tests must use only the interfaces, that means, the way any other will use the process/code to produce required output. For Integration Tests, a *tests* directory must be used.

The *test* directory is created at the top level of the project directory, as *src* sibling.

Inside the integration test code file, there's no need to define mod with `#[cfg(test)]` annotation. It will work, like said before, as another programmer using the library, so the module should be imported. Then, the functions are defined and annotated with `#[test]`.

In order to create commom functions to prepare the tests, a file can be created to group all this functions in a file inside `$project_root/tests/commom/mod.rs`. Doing this, the mod file won't run and show at test logs.

###### cargo test parameters
In order to user test parameters, two hyphens needs to be used
>> $ cargo test -- --test-threads=1
* --test-threads: This controls the amount of threads to run the tests. It will affect the parallelism. When set to 1, the tests won't use parallelism.
* --show-output: This shows all the outputs from the tests: failed and successed.
* --ignore: The annotation `#[ignore]` will ignore the test when running `cargo test`, but using this parameter will run ONLY the ignored annotated tests.
* --include-ignore: This parameters will run every test, even if annotated with the `#[ignore]`.

##### Run test by name
To run a single test, or a couple of them, instead of running all and search for the one, the name of the function can be used as a parameter to run specific tests.
Let's suppose there is a teste called `my_test` and using the `#[test]` annotation. To test it individually, the following command should be used
>> $ cargo test my_test

This command will evaluate all tests that match the pattern informed. So, if there another test called `my_test_test`, it will run it as well.

##### Ignoring teste
