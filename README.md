[![codecov](https://codecov.io/github/Sajjon/one-does-not-simply-sign/branch/main/graph/badge.svg?token=PTFupnAjyZ)](https://codecov.io/github/Sajjon/one-does-not-simply-sign)

# Concepts

`FactorInstanceAccumulator` a.k.a. `FIA` is an accumulator which iterates through
all factor sources and dispatches requests to collect their output - `FactorInstance`s -
and builds up - `accumulates` - a final result. The type of `FactorInstance` is
either a `Signature` or a `PublicKey`. FIA support batch operations, meaning it will
collect signatures for MANY transactions, or it will derive public keys for many
Security Shields. In both for both operation kinds, or rather for both processes,
it will need to derive a private key at some derivation path for some FactorSource
and either sign some hashes with it, or simply derive its public key.

# Design Decision

> [!IMPORTANT]
> FIA does not handle retries of any FactorSource operation. The concept of
> retry does not exist in FIA.
> If using a FactorSource, e.g. signing with a Ledger fails, it is up to the
> host to prompt for retry.
> If a `UseFactorSourceDriver` returns `Err`, using that FactorSource will
> have failed.
