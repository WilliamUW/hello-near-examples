const express = require("express");
const nearAPI = require("near-api-js");
const { connect, Contract, keyStores, KeyPair } = nearAPI;

const PRIVATE_KEY =
  "ed25519:5vXPCRENf1oUHJFCK5Dt3o3hiswrPNuzPTBekD4s646beRp26yTpU4WmmETE81gv7bvRgcgeM1hZKkVCg3iStBf1";
const ACCOUNT_ID = "male-beginner.testnet";
const CONTRACT_ID = "male-beginner.testnet";

const myKeyStore = new keyStores.InMemoryKeyStore();
const keyPair = KeyPair.fromString(PRIVATE_KEY);

const app = express();
const PORT = 3000;

async function getNearContract() {
  // Adds the keyPair to the keyStore
  await myKeyStore.setKey("testnet", ACCOUNT_ID, keyPair);

  // Set up the connection configuration
  const connectionConfig = {
    networkId: "testnet",
    keyStore: myKeyStore,
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://testnet.mynearwallet.com/",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://testnet.nearblocks.io",
  };

  // Connect to NEAR
  const nearConnection = await connect(connectionConfig);

  // Get the account object
  const account = await nearConnection.account(ACCOUNT_ID);

  // Get the contract object
  const contract = new nearAPI.Contract(account, CONTRACT_ID, {
    viewMethods: ["get_messages"], // Your view methods here
    changeMethods: ["add_message"], // Your change methods here
  });

  return { contract, account };
}

app.get("/add_message", async (req, res) => {
  try {
    const { contract, account } = await getNearContract();

    // Call the contract's method with the appropriate account as the signer
    const result = await contract.add_message({
      args: { text: "hi" },
      signerAccount: account,
    });

    console.log("Message added successfully:", result);
  } catch (error) {
    console.error("Error fetching artworks:", error);
    res.status(500).json({ error: "Failed to fetch artworks" });
  }
});

app.get("/get_messages", async (req, res) => {
  try {
    const { contract, account } = await getNearContract();
    const artworks = await contract.get_messages();
    res.json(artworks);
  } catch (error) {
    console.error("Error fetching artworks:", error);
    res.status(500).json({ error: "Failed to fetch artworks" });
  }
});

app.listen(PORT, () => {
  console.log(`Server is running on port ${PORT}`);
});
