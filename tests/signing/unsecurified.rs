use fia::prelude::*;

struct FiaSign {
    fia: FiaTransactionSigning,
}

impl FiaSign {
    pub fn new(
        transactions: impl IntoIterator<Item = TransactionIntent>,
        entities: impl IntoIterator<Item = Entity>,
    ) -> Self {
        let driver: Box<dyn SignWithFactorSourceDriver> = Box::new(TestSignDriver);
        let fia = FiaTransactionSigning::new_batch_sign_by_analyzing_transactions(
            transactions.into_iter().collect_vec(),
            entities.into_iter().collect_vec(),
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
    let sut = FiaSign::new([], []);
    let result = sut.run().await;
    assert!(matches!(result, Ok(_)))
}
