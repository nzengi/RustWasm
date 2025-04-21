// Contract.js - Smart contract interaction helpers

import * as ethUtils from "./eth.js";

/**
 * Deploy a smart contract
 * @param {Object} options Contract options
 * @param {string} options.bytecode Contract bytecode
 * @param {string} options.abi Contract ABI JSON string
 * @param {Array} options.args Constructor arguments
 * @param {Object} options.deployOptions Transaction options (from, gas, etc.)
 */
export async function deployContract(options) {
  try {
    const { bytecode, abi, args = [], deployOptions = {} } = options;

    if (!bytecode) throw new Error("Contract bytecode is required");
    if (!abi) throw new Error("Contract ABI is required");

    // Ensure we have an account to deploy from
    if (!deployOptions.from) {
      const accounts = await ethUtils.requestAccounts();
      if (accounts && accounts.length > 0) {
        deployOptions.from = accounts[0];
      } else {
        throw new Error("No accounts available for deployment");
      }
    }

    // Prepare transaction data
    const data = bytecode + encodeConstructorArgs(abi, args);

    const txParams = {
      from: deployOptions.from,
      data,
      gas: deployOptions.gas || "0x500000", // Default gas
      value: deployOptions.value || "0x0", // Default value
    };

    // Send deployment transaction
    const txHash = await ethUtils.sendTransaction(txParams);

    // Return transaction hash (contract address will be available in receipt)
    return {
      transactionHash: txHash,
      from: deployOptions.from,
    };
  } catch (error) {
    console.error("Error deploying contract:", error);
    throw error;
  }
}

/**
 * Interacts with an existing contract
 * @param {Object} options Contract options
 * @param {string} options.address Contract address
 * @param {string} options.abi Contract ABI JSON string
 * @param {string} options.method Function name to call
 * @param {Array} options.args Function arguments
 * @param {Object} options.txOptions Transaction options for non-view functions
 * @param {boolean} options.isView Whether this is a view function (no state change)
 */
export async function callContract(options) {
  try {
    const {
      address,
      abi,
      method,
      args = [],
      txOptions = {},
      isView = true,
    } = options;

    if (!address) throw new Error("Contract address is required");
    if (!abi) throw new Error("Contract ABI is required");
    if (!method) throw new Error("Method name is required");

    // Encode function call
    const encodedData = encodeFunctionCall(abi, method, args);

    if (isView) {
      // Call view function (no state change)
      return await ethUtils.callContract(address, encodedData);
    } else {
      // For state-changing functions, we need an account
      if (!txOptions.from) {
        const accounts = await ethUtils.requestAccounts();
        if (accounts && accounts.length > 0) {
          txOptions.from = accounts[0];
        } else {
          throw new Error("No accounts available for transaction");
        }
      }

      const txParams = {
        from: txOptions.from,
        to: address,
        data: encodedData,
        gas: txOptions.gas || "0x100000", // Default gas
        value: txOptions.value || "0x0", // Default ETH value
      };

      // Send transaction
      return await ethUtils.sendTransaction(txParams);
    }
  } catch (error) {
    console.error("Error calling contract:", error);
    throw error;
  }
}

/**
 * Creates an event subscription for a contract
 * @param {Object} options Subscription options
 * @param {string} options.address Contract address
 * @param {string} options.abi Contract ABI JSON string
 * @param {string} options.eventName Name of the event to subscribe to
 * @param {Object} options.filter Filter parameters (indexed event args)
 * @param {function} callback Function to call on each event
 */
export async function subscribeToEvent(options, callback) {
  try {
    const { address, abi, eventName, filter = {} } = options;

    if (!address) throw new Error("Contract address is required");
    if (!abi) throw new Error("Contract ABI is required");
    if (!eventName) throw new Error("Event name is required");

    // This is a simplified implementation
    // In a real app, you'd need to implement proper event filtering with Web3.js or ethers.js
    console.log(`Subscribed to ${eventName} events on contract ${address}`);

    // Return unsubscribe function
    return () => {
      console.log(`Unsubscribed from ${eventName} events`);
    };
  } catch (error) {
    console.error("Error subscribing to event:", error);
    throw error;
  }
}

// Helper function to encode constructor arguments
// This is a simplified implementation - in a real application,
// you would use ethers.js or Web3.js for proper ABI encoding
function encodeConstructorArgs(abi, args) {
  // Placeholder for actual encoding logic
  console.log("Encoding constructor args:", args);
  return ""; // In a real implementation, this would return the encoded args
}

// Helper function to encode function call data
// This is a simplified implementation
function encodeFunctionCall(abi, method, args) {
  // Very simplified implementation - for demo purposes only
  // In a real app, use ethers.js or web3.js for proper encoding

  try {
    // Parse ABI to find the function
    const abiObj = typeof abi === "string" ? JSON.parse(abi) : abi;
    const methodAbi = abiObj.find(
      (item) => item.name === method && item.type === "function"
    );

    if (!methodAbi) {
      throw new Error(`Method ${method} not found in ABI`);
    }

    // Create a very basic function selector (first 4 bytes of the hash)
    // This is a simplified demo - not for production use
    const functionSignature = `${method}(${methodAbi.inputs
      .map((input) => input.type)
      .join(",")})`;
    console.log(`Function signature: ${functionSignature}`);

    // In a real implementation, you would hash this signature and take the first 4 bytes
    // For demo, just return a placeholder
    return `0x12345678${Buffer.from(functionSignature).toString("hex")}`;
  } catch (error) {
    console.error("Error encoding function call:", error);
    return `0x12345678`; // Placeholder
  }
}
