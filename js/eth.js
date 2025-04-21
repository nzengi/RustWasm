// Eth.js - Helper functions for Ethereum interaction

// Check for the presence of MetaMask or other Ethereum wallets
export function checkEthereumProvider() {
  console.log("Checking for Ethereum provider...");
  if (window.ethereum) {
    console.log("Ethereum provider found:", window.ethereum);
    return {
      available: true,
      provider: window.ethereum,
    };
  }
  console.warn("No Ethereum provider found");
  return {
    available: false,
    provider: null,
  };
}

// Request Ethereum accounts
export async function requestAccounts() {
  try {
    if (!window.ethereum) {
      throw new Error(
        "No Ethereum provider found. Please install MetaMask or a similar wallet."
      );
    }

    const accounts = await window.ethereum.request({
      method: "eth_requestAccounts",
    });
    return accounts;
  } catch (error) {
    console.error("Error requesting accounts:", error);
    throw error;
  }
}

// Get network information
export async function getNetworkInfo() {
  try {
    if (!window.ethereum) {
      throw new Error("No Ethereum provider found");
    }

    const chainId = await window.ethereum.request({ method: "eth_chainId" });

    // Known networks
    const networks = {
      "0x1": "Ethereum Mainnet",
      "0x3": "Ropsten Test Network",
      "0x4": "Rinkeby Test Network",
      "0x5": "Goerli Test Network",
      "0x2a": "Kovan Test Network",
      "0x38": "Binance Smart Chain",
      "0x89": "Polygon",
      "0xa86a": "Avalanche",
    };

    return {
      chainId,
      networkName: networks[chainId] || "Unknown Network",
    };
  } catch (error) {
    console.error("Error getting network info:", error);
    throw error;
  }
}

// Send transaction
export async function sendTransaction(transaction) {
  try {
    if (!window.ethereum) {
      throw new Error("No Ethereum provider found");
    }

    // Send the transaction
    const txHash = await window.ethereum.request({
      method: "eth_sendTransaction",
      params: [transaction],
    });

    return txHash;
  } catch (error) {
    console.error("Error sending transaction:", error);
    throw error;
  }
}

// Call a smart contract (for read-only operations)
export async function callContract(contractAddress, encodedABI) {
  try {
    if (!window.ethereum) {
      throw new Error("No Ethereum provider found");
    }

    const result = await window.ethereum.request({
      method: "eth_call",
      params: [
        {
          to: contractAddress,
          data: encodedABI,
        },
        "latest",
      ],
    });

    return result;
  } catch (error) {
    console.error("Error calling contract:", error);
    throw error;
  }
}

// Convert Wei to Ether
export function weiToEther(wei) {
  return wei / 1000000000000000000; // 10^18
}

// Convert Ether to Wei
export function etherToWei(ether) {
  return ether * 1000000000000000000; // 10^18
}

// Set up event listener
export function setupEventListener(eventName, callback) {
  if (!window.ethereum) {
    throw new Error("No Ethereum provider found");
  }

  window.ethereum.on(eventName, callback);

  // Cleanup function
  return () => {
    window.ethereum.removeListener(eventName, callback);
  };
}

// Sample ERC-20 ABI for easy testing
export const SAMPLE_ERC20_ABI = `[
  {
    "constant": true,
    "inputs": [],
    "name": "name",
    "outputs": [{"name": "", "type": "string"}],
    "payable": false,
    "stateMutability": "view",
    "type": "function"
  },
  {
    "constant": true,
    "inputs": [],
    "name": "symbol",
    "outputs": [{"name": "", "type": "string"}],
    "payable": false,
    "stateMutability": "view",
    "type": "function"
  },
  {
    "constant": true,
    "inputs": [],
    "name": "decimals",
    "outputs": [{"name": "", "type": "uint8"}],
    "payable": false,
    "stateMutability": "view",
    "type": "function"
  },
  {
    "constant": true,
    "inputs": [],
    "name": "totalSupply",
    "outputs": [{"name": "", "type": "uint256"}],
    "payable": false,
    "stateMutability": "view",
    "type": "function"
  },
  {
    "constant": true,
    "inputs": [{"name": "owner", "type": "address"}],
    "name": "balanceOf",
    "outputs": [{"name": "", "type": "uint256"}],
    "payable": false,
    "stateMutability": "view",
    "type": "function"
  },
  {
    "constant": false,
    "inputs": [
      {"name": "to", "type": "address"},
      {"name": "value", "type": "uint256"}
    ],
    "name": "transfer",
    "outputs": [{"name": "", "type": "bool"}],
    "payable": false,
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "constant": true,
    "inputs": [
      {"name": "owner", "type": "address"},
      {"name": "spender", "type": "address"}
    ],
    "name": "allowance",
    "outputs": [{"name": "", "type": "uint256"}],
    "payable": false,
    "stateMutability": "view",
    "type": "function"
  },
  {
    "constant": false,
    "inputs": [
      {"name": "spender", "type": "address"},
      {"name": "value", "type": "uint256"}
    ],
    "name": "approve",
    "outputs": [{"name": "", "type": "bool"}],
    "payable": false,
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "constant": false,
    "inputs": [
      {"name": "from", "type": "address"},
      {"name": "to", "type": "address"},
      {"name": "value", "type": "uint256"}
    ],
    "name": "transferFrom",
    "outputs": [{"name": "", "type": "bool"}],
    "payable": false,
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "anonymous": false,
    "inputs": [
      {"indexed": true, "name": "from", "type": "address"},
      {"indexed": true, "name": "to", "type": "address"},
      {"indexed": false, "name": "value", "type": "uint256"}
    ],
    "name": "Transfer",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {"indexed": true, "name": "owner", "type": "address"},
      {"indexed": true, "name": "spender", "type": "address"},
      {"indexed": false, "name": "value", "type": "uint256"}
    ],
    "name": "Approval",
    "type": "event"
  }
]`;
