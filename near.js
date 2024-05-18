const nearAPI = require("near-api-js");
const { connect, Contract, keyStores, KeyPair } = nearAPI;

const PRIVATE_KEY = "ed25519:5rfmAZX62xHZjq1HCRSmEgt6c1i63VzY7k77Riq4Kmb2pVSb9Y7GpPSkYV2TQJX3sN8NCKpBfRXZs2CBbrxA2bze";
const ACCOUNT_ID = "delirious-whip.testnet";
const CONTRACT_ID = "delirious-whip.testnet";

const myKeyStore = new keyStores.InMemoryKeyStore();
const keyPair = KeyPair.fromString(PRIVATE_KEY);

async function main() {
  // Adds the keyPair to the keyStore
  await myKeyStore.setKey("testnet", ACCOUNT_ID, keyPair);
  console.log("Key added to keyStore successfully");

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
  console.log("Connected to NEAR successfully");

  // Get the account object
  const account = await nearConnection.account(ACCOUNT_ID);

  // Set up the contract object
  const contract = new Contract(account, CONTRACT_ID, {
    viewMethods: ["get_artworks"], // Specify view methods
  });

  // Call the view method
  const artworks = await contract.get_artworks();
  console.log("Artworks:", artworks);
}

main().catch(console.error);
