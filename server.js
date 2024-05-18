const express = require("express");
const nearAPI = require("near-api-js");
const { connect, Contract, keyStores, KeyPair } = nearAPI;

const PRIVATE_KEY = "ed25519:5rfmAZX62xHZjq1HCRSmEgt6c1i63VzY7k77Riq4Kmb2pVSb9Y7GpPSkYV2TQJX3sN8NCKpBfRXZs2CBbrxA2bze";
const ACCOUNT_ID = "delirious-whip.testnet";
const CONTRACT_ID = "delirious-whip.testnet";

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

  // Set up the contract object
  return new Contract(account, CONTRACT_ID, {
    viewMethods: ["get_artworks"], // Specify view methods
  });
}

app.get("/artworks", async (req, res) => {
  try {
    const contract = await getNearContract();
    const artworks = await contract.get_artworks();
    res.json(artworks);
  } catch (error) {
    console.error("Error fetching artworks:", error);
    res.status(500).json({ error: "Failed to fetch artworks" });
  }
});

app.listen(PORT, () => {
  console.log(`Server is running on port ${PORT}`);
});
