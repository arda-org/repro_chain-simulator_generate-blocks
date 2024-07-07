import { test } from "vitest";
import { FSWorld } from "xsuite";

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
