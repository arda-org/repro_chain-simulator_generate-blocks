Scenario that fails:
- User on shard 0 calls contract on shard 1
- Contract on shard 1 calls another contract on shard 0 and in the callback transfers token to user on shard 0
- Fails with error:

```
Error: Unsuccessful proxy request. Response: {
  "data": null,
  "error": "cannot generate blocks: something went wrong, transaction is still in pending",
  "code": "internal_issue"
}
```

## How to reproduce

```
npm install

npm run build

npm run test
```

## The contract

The contract ([./contract.rs](./contract.rs)):

```
pub trait Contract {
    ...

    #[proxy]
    fn self_proxy(&self, remote_address: ManagedAddress) -> self::Proxy<Self::Api>;

    #[payable("*")]
    #[endpoint]
    fn call_remote(&self, remote_address: ManagedAddress) {
        let payment = self.call_value().egld_or_single_esdt();
        let caller = self.blockchain().get_caller();
        self.self_proxy(remote_address)
            .do_nothing()
            .callback(self.callbacks().call_remote_callback(caller, payment))
            .call_and_exit();
    }

    #[callback]
    fn call_remote_callback(
        &self,
        caller: ManagedAddress,
        payment: EgldOrEsdtTokenPayment<Self::Api>,
    ) {
        self.send().direct(&caller, &payment.token_identifier, payment.token_nonce, &payment.amount);
    }

    #[endpoint]
    fn do_nothing(&self) {}
}
```

## The test

The test ([./contract.test.rs](./contract.test.ts)):

```
test("Test", async () => {
  using world = await FSWorld.start();
  const userShard0 = await world.createWallet({
    address: { shard: 0 },
    balance: 10n ** 18n,
  });
  const [contractShard0, contractShard1] = await world.createContracts([
    {
      address: { shard: 0 },
      code: "file:output/contract.wasm",
    },
    {
      address: { shard: 1 },
      code: "file:output/contract.wasm",
    },
  ]);
  await userShard0.callContract({
    callee: contractShard1,
    funcName: "call_remote",
    funcArgs: [contractShard0],
    gasLimit: 50_000_000,
    value: 1,
  });
});
```
