use fia::prelude::*;

struct FiaSign {
    fia: FiaTransactionSigning,
}

impl FiaSign {
    pub fn just(transaction: TransactionIntent) -> Self {
        Self::new([transaction])
    }
    pub fn new(transactions: impl IntoIterator<Item = TransactionIntent>) -> Self {
        let driver: Box<dyn SignWithFactorSourceDriver> = Box::new(TestSignDriver);
        let fia = FiaTransactionSigning::new_batch_sign_by_analyzing_transactions(
            transactions.into_iter().collect_vec(),
            FactorSource::all(),
            [driver],
        )
        .unwrap();
        Self { fia }
    }

    async fn run(&self) -> Result<FiaOutput<IntentHash, HDSignature>> {
        self.fia.accumulate().await
    }
}

#[actix_rt::test]
async fn trivial() {
    let sut = FiaSign::just(TransactionIntent::just(Entity::unsecurified_anonymous(
        fs_id_with_kind(FactorSourceKind::Device),
    )));
    let result = sut.run().await;
    assert!(matches!(result, Ok(_)))
}
