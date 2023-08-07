use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn test_balanced_decimal_demo() {
    let mut test_runner = TestRunner::builder().without_trace().build();
    let (public_key, _private_key, account) = test_runner.new_allocated_account();
    let package_address = test_runner.compile_and_publish(this_package!());

    let manifest = ManifestBuilder::new()
        .call_function(
            package_address,
            "BalancedDecimalDemo",
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
            "free_tokens_decimal",
            manifest_args!(dec!("29.393993999102121881")),
        )
        .call_method(
            component,
            "free_tokens_precise_decimal",
            manifest_args!(pdec!("29.3939939991021218813939939991021218815400001")),
        )
        .deposit_batch(account)
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}\n", receipt);
    let commit_result = receipt.expect_commit_success();
    let (_, output_free_tokens_decimal): (Bucket, Decimal) = commit_result.output(1);
    let (_, output_free_tokens_precise_decimal): (Bucket, PreciseDecimal) = commit_result.output(2);

    assert_eq!(output_free_tokens_decimal, dec!("29.393993999102121881"));
    assert_eq!(output_free_tokens_precise_decimal, pdec!("29.39399399910212188139399399910212188154"));
}
