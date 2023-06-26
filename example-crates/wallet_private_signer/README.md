# Privacy preserving multi-sig signing service example

This example demonstrates how to use `bdk` to create a privacy preserving multi-sig signing service. It is only a simple sketch of the code needed.

### Problem

Create a signing service that when properly authorized will add valid signatures to a client's PSBT without knowing the client's full wallet history. The signing service must be able to determine the transaction spending amount. The client must be able to sign and finalize a transactions independently of the service.

### Solution

1. For each client the signing service creates a signing bip39 xprv key and shares the corresponding xpub with the client's software. The signing service does not need to share any root key information with the client except the key fingerprint. Example:
    ```
    [24bff216/84'/1'/100']tpubDCQkJg1DDPPaTQg9KmdZjmq9FSJffLLy5vx74cyvfWqj78wwnWMeGAY2i8hHVHsMcf6PAWi5tQ1oQ3UFuxPc6pQZMAbPoggvUS78vgdH7oX/0/*
    ```
2. The client software generates it's own bip39 xprv key and corresponding xpub key.
3. The client software collects the client's hardware signer's xpub.
4. The client software uses the client's xpubs and the signing service provided xpub to create a multi-sig taproot wallet descriptor of the form `tr(<unspendable>, { and_v(v:pk(APP),pk(SVR)), { and_v(v:pk(APP),pk(HWS)), and_v(v:pk(HWS),pk(SVR)) }})`. The client should NOT share this information with the service or anyone else. Example:
  ```
  tr(020000000000000000000000000000000000000000000000000000000000000001,{and_v(v:pk([6fd38f0d/86'/1'/0']tprv8fxz3mLgqbq5G4rsjkRqSzfXNXz2ADyTyBLUai6zKatPDqJHxLXL9CkwVyksbssPPtF3DxtcnmzBhbeEKu9cpVifsFQq8hy7LupSGjRCRp3/0/*),pk([24bff216/86'/1'/100']tpubDCmfBxaNgzLocQBGfGGTrHUnjtZN9gBzJU5N7zrxJC6RyN3b5rGdpzehvNspRyJx96Nkv1pVpdNnbi221WmXQp5wxxgv4AdRjjRuth8YmtY/0/*)),{and_v(v:pk([6fd38f0d/86'/1'/0']tprv8fxz3mLgqbq5G4rsjkRqSzfXNXz2ADyTyBLUai6zKatPDqJHxLXL9CkwVyksbssPPtF3DxtcnmzBhbeEKu9cpVifsFQq8hy7LupSGjRCRp3/0/*),pk([e6be9672/86'/1'/0']tpubDDQhhz5Rh23DHRFcEivwP6G77vuUHoecD1NMUsCavnUZokRAFJudWNrM8A3Xja9rJyzi3dPjP1XWfASEpvireK5yXSpcsRHSWNwKvZwhfh5/0/*)),and_v(v:pk([e6be9672/86'/1'/0']tpubDDQhhz5Rh23DHRFcEivwP6G77vuUHoecD1NMUsCavnUZokRAFJudWNrM8A3Xja9rJyzi3dPjP1XWfASEpvireK5yXSpcsRHSWNwKvZwhfh5/0/*),pk([24bff216/86'/1'/100']tpubDCmfBxaNgzLocQBGfGGTrHUnjtZN9gBzJU5N7zrxJC6RyN3b5rGdpzehvNspRyJx96Nkv1pVpdNnbi221WmXQp5wxxgv4AdRjjRuth8YmtY/0/*))}})#jvjsn2tg
  ```
5. The client's software monitors the blockchain for relevant descriptor wallet transactions.
6. When the client wants to make a spending transaction their client software constructs a valid PSBT and adds their wallet's signature.
7. The client's software redacts the client's tap key origin information from the PSBT.
8. The client authenticates themselves to the signing service.
9. The client's software sends the redacted PSBT to the signing service.
10. The signing service adds their signature to the PSBT based on the tap key origin indicated by the client's software in the PSBT and sends the fully signed PSBT back to the client's software.
11. The client's software finalizes the PSBT and broadcasts it.

## Caveats

1. This only works for taproot wallets (thanks to @jesseposner for pointing out that other script types always reveal the server's pubkey so are trivial to track by the server).
2. The client must be able to securely backup and restore their wallet descriptor.
3. The signing service must trust the client's software to correctly construct a valid PSBT.
4. The signing service must trust the client's software to correctly indicate which output is for change.
5. The signing service will be able to associate the client to any transactions that include inputs or change outputs of a PSBT it is given to sign.

Note: Caveat 4 could be fixed by having the client software create a signed message that proves to the server which output has a tap script spending path that requires the client and server keys to spend.

## Privacy improvements

- The client software should track which change outputs it has shared with the signing service and allow the client to create spendings transactions that do not include these UTXOs.
- When possible the client software should construct payjoin PSBTs with their payment recipients so the singing service won't have full knowledge of which inputs they are associated with.
- Make sure when generating the internal key that it is provably unspendable. See: ["Provably unspendable internal key"](https://github.com/Coldcard/firmware/blob/edge/docs/taproot.md#provably-unspendable-internal-key)