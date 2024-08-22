[![codecov](https://codecov.io/github/Sajjon/one-does-not-simply-sign/branch/main/graph/badge.svg?token=PTFupnAjyZ)](https://codecov.io/github/Sajjon/one-does-not-simply-sign)

"Coordinators" for collecting and accumulating signatures and public keys from multiple `FactorSources` in one "session" or one "process".

This repo contains seperate solutions for both "processes" (signature collecting and public key collecting respectively), since it was too hard to make a generic solution. Why? Because of the nature of differences in input, for signing we have a HashMap as input, many transactions ID to be signed by many derivation paths, per factor source, whereas for public key derivation we only have derivation paths. This makes it hard to come up with a suitable generic "shape".