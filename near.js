const nearAPI = require("near-api-js");
const { connect } = nearAPI;

// creates keyStore from a private key string
// you can define your key here or use an environment variable

const { keyStores, KeyPair } = nearAPI;
const myKeyStore = new keyStores.InMemoryKeyStore();
const PRIVATE_KEY = "by8kdJoJHu7uUkKfoaLd2J2Dp1q1TigeWMG123pHdu9UREqPcshCM223kWadm";

// creates a public / private key pair using the provided private key
const keyPair = KeyPair.fromString(PRIVATE_KEY);

// Wrap your code in an async function
(async () => {
  // adds the keyPair you created to keyStore
  await myKeyStore.setKey("testnet", "example-account.testnet", keyPair);

  console.log("Key added to keyStore successfully");

  const connectionConfig = {
    networkId: "testnet",
    keyStore: myKeyStore, // first create a key store
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://testnet.mynearwallet.com/",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://testnet.nearblocks.io",
  };
  const nearConnection = await connect(connectionConfig);
  console.log(nearConnection)
})().catch(console.error);




