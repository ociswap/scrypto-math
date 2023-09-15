use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn test_advanced_math_demo() {
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (public_key, _private_key, account) = test_runner.new_allocated_account();
    let package_address = test_runner.compile_and_publish(this_package!());

    let manifest = ManifestBuilder::new()
        .call_function(
            package_address,
            "AdvancedMathDemo",
            "instantiate",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}\n", receipt);
    let component = receipt.expect_commit(true).new_component_addresses()[0];

    let manifest = ManifestBuilder::new()
        .call_method(
            component,
            "free_tokens",
            manifest_args!(dec!("3.14159265359")),
        )
        .deposit_batch(account)
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}\n", receipt);
    let commit_result = receipt.expect_commit_success();
    let (_, output_free_tokens): (Bucket, Decimal) = commit_result.output(1);

    assert_eq!(output_free_tokens, dec!("4.435924499291774560") + dec!("0.000000000000000001"));
}
