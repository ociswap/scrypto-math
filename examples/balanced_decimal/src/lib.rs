use scrypto::prelude::*;
use scrypto_math::*;

#[blueprint]
mod balanced_decimal_demo {
    struct BalancedDecimalDemo {
        oci_vault: Vault,
    }

    impl BalancedDecimalDemo {
        pub fn instantiate() -> Global<BalancedDecimalDemo> {
            let oci_bucket: Bucket =
                ResourceBuilder::new_fungible(OwnerRole::None).mint_initial_supply(1000);
            Self {
                oci_vault: Vault::with_bucket(oci_bucket),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn free_tokens_decimal(&mut self, amount: Decimal) -> (Bucket, Decimal) {
            let amount = BalancedDecimal::try_from(amount).expect("Value too large.");
            let output = self.oci_vault.take(amount.floor_to_decimal());
            (output, amount.into())
        }

        pub fn free_tokens_precise_decimal(
            &mut self,
            amount: PreciseDecimal,
        ) -> (Bucket, PreciseDecimal) {
            let amount = BalancedDecimal::try_from(amount).expect("Value too large.");
            let output = self.oci_vault.take(amount.ceil_to_decimal());
            (output, amount.into())
        }
    }
}
