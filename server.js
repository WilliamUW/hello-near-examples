const express = require("express");
const nearAPI = require("near-api-js");
const { connect, Contract, keyStores, KeyPair } = nearAPI;

const PRIVATE_KEY =
  "ed25519:5vXPCRENf1oUHJFCK5Dt3o3hiswrPNuzPTBekD4s646beRp26yTpU4WmmETE81gv7bvRgcgeM1hZKkVCg3iStBf1";
const ACCOUNT_ID = "male-beginner.testnet";
const CONTRACT_ID = "neargoredacted.testnet";

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
    viewMethods: ["get_records"], // Your view methods here
    changeMethods: ["add_record"], // Your change methods here
  });

  return { contract, account };
}

app.get("/add_record", async (req, res) => {
  try {
    const { contract, account } = await getNearContract();

    // Call the contract's method with the appropriate account as the signer
    const result = await contract.add_record({
      args: {
        species: "King Penguin",
        location: "Sea Life Melbourne Aquarium",
        time_captured: "2024-09-10",
        description:
          "A beautiful King Penguin spotted at the Sea Life Melbourne Aquarium!",
        image_blob_id:
          "https://www.pedestrian.tv/wp-content/uploads/2024/09/Pesto-Gender-Reveal.jpg?quality=75&w=1024",
        latitude: "40.7468733",
        longitude: "-73.9947449",
      },
      signerAccount: account,
    });

    console.log("record added successfully:", result);
  } catch (error) {
    console.error("Error fetching artworks:", error);
    res.status(500).json({ error: "Failed to fetch artworks" });
  }
});

app.get("/get_records", async (req, res) => {
  try {
    const { contract } = await getNearContract();
    const artworks = await contract.get_records();
    res.json(artworks);
  } catch (error) {
    console.error("Error fetching artworks:", error);
    res.status(500).json({ error: "Failed to fetch artworks" });
  }
});

app.listen(PORT, () => {
  console.log(`Server is running on port ${PORT}`);
});
