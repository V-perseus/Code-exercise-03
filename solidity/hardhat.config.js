require("@nomicfoundation/hardhat-toolbox");
require("@nomiclabs/hardhat-ganache");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  networks: {
    hardhat: {
      host: "localhost",
      port: 7545,
      network_id: "5777"
    }
  },
  solidity: "0.8.17",
};
