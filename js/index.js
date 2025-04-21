// Simple approach to load modules
console.log("JavaScript module loading...");

// Check if WebAssembly is supported
console.log("Is WebAssembly supported:", typeof WebAssembly === "object");

// Set a loading status
document.getElementById("web3-status").textContent = "JavaScript loading...";

// Global variables for modules
let wasmModule, ethUtilsModule, utilsModule, contractUtilsModule;

// Helper function to call WASM functions with better error handling
async function callWasmFunction(name, ...args) {
  console.log(`Calling WASM function: ${name} with args:`, args);

  if (!wasmModule) {
    throw new Error("WASM module not loaded yet");
  }

  const func = wasmModule[name];

  if (typeof func !== "function") {
    console.error(
      `Function ${name} not found in module. Available functions:`,
      Object.keys(wasmModule).filter((k) => typeof wasmModule[k] === "function")
    );
    throw new Error(`Function ${name} not found in WebAssembly module`);
  }

  try {
    const result = await func(...args);
    console.log(`Function ${name} result:`, result);
    return result;
  } catch (error) {
    console.error(`Error calling ${name}:`, error);
    throw error;
  }
}

// Load modules one by one
import("../pkg/rustwasm_eth.js")
  .then((wasm) => {
    wasmModule = wasm;
    console.log("WASM module loaded:", wasmModule);
    console.log(
      "Available functions:",
      Object.keys(wasmModule).filter((k) => typeof wasmModule[k] === "function")
    );

    // Initialize WASM module
    return wasmModule.default();
  })
  .then(() => {
    console.log("WASM initialized successfully");
    document.getElementById("web3-status").textContent =
      "WASM loaded, other modules loading...";

    // Load other modules
    return Promise.all([
      import("./eth.js").then((mod) => (ethUtilsModule = mod)),
      import("./utils.js").then((mod) => (utilsModule = mod)),
      import("./contract.js").then((mod) => (contractUtilsModule = mod)),
    ]);
  })
  .then(() => {
    console.log("All modules loaded successfully");
    document.getElementById("web3-status").textContent = "All modules loaded!";

    // Initialize UI
    initializeUI();
  })
  .catch((error) => {
    console.error("Module loading error:", error);
    document.getElementById(
      "web3-status"
    ).textContent = `Error: ${error.message}`;
    document.getElementById("web3-status").className = "status-error";
  });

// Main function to initialize UI
function initializeUI() {
  // Check Ethereum provider
  const ethereum = window.ethereum;
  const web3StatusElement = document.getElementById("web3-status");

  if (ethereum) {
    web3StatusElement.textContent = "MetaMask or compatible wallet found";
    web3StatusElement.className = "status-ok";
    console.log("Ethereum provider details:", ethereum);
  } else {
    web3StatusElement.textContent =
      "No Web3 wallet found. Please install MetaMask.";
    web3StatusElement.className = "status-error";
    console.warn("No Ethereum provider detected");
  }

  // Select UI elements
  const greetButton = document.getElementById("greet-button");
  const connectButton = document.createElement("button");
  connectButton.textContent = "Connect to Ethereum";
  connectButton.className = "eth-button";

  const accountsButton = document.createElement("button");
  accountsButton.textContent = "Get Accounts";
  accountsButton.className = "eth-button";
  accountsButton.style.display = "none";

  const networkButton = document.createElement("button");
  networkButton.textContent = "Get Network Info";
  networkButton.className = "eth-button";
  networkButton.style.display = "none";

  const resultDiv = document.createElement("div");
  resultDiv.id = "result";
  resultDiv.className = "result-container";

  // Add UI elements to the page
  const main = document.querySelector("main");
  main.appendChild(connectButton);
  main.appendChild(accountsButton);
  main.appendChild(networkButton);
  main.appendChild(resultDiv);

  // Contract section element
  const contractSection = document.getElementById("contract-section");

  // Add functionality to the basic greeting button
  greetButton.addEventListener("click", async () => {
    try {
      await callWasmFunction("greet");
    } catch (error) {
      console.error("Greet error:", error);
      alert("Greet function error: " + error.message);
    }
  });

  // Add functionality to Connect to Ethereum button
  connectButton.addEventListener("click", async () => {
    try {
      resultDiv.innerHTML = "<p>Connecting to Ethereum...</p>";

      // Alternative approach if the direct call fails
      let connectResult;
      try {
        // Try direct WebAssembly function call
        connectResult = await callWasmFunction("connect_to_ethereum");
      } catch (error) {
        // Fallback to JavaScript implementation
        console.warn(
          "WebAssembly connect_to_ethereum failed, using JS fallback:",
          error
        );

        // JavaScript fallback implementation
        if (!ethereum) {
          throw new Error("No Ethereum provider found");
        }

        try {
          // Request accounts from MetaMask
          const accounts = await ethereum.request({
            method: "eth_requestAccounts",
          });

          if (accounts && accounts.length > 0) {
            connectResult = "Connected to Ethereum!";
          } else {
            connectResult = "Connected to Ethereum but no accounts available";
          }
        } catch (ethError) {
          throw new Error("MetaMask connection failed: " + ethError.message);
        }
      }

      console.log("Ethereum connection result:", connectResult);

      // Show other buttons if connection is successful
      accountsButton.style.display = "inline-block";
      networkButton.style.display = "inline-block";

      // Show contract section
      contractSection.style.display = "block";

      resultDiv.innerHTML = `
        <p class="success">✅ ${connectResult}</p>
        <p>Now you can interact with Ethereum blockchain.</p>
      `;
    } catch (error) {
      // Display error message
      const errorMessage = error.message || "Unknown error";
      console.error("Connection error:", errorMessage, error);

      resultDiv.innerHTML = `
        <p class="error">❌ Connection failed: ${errorMessage}</p>
        <p>Please make sure you have MetaMask or another Ethereum wallet installed.</p>
      `;
    }
  });

  // Add functionality to Get Accounts button
  accountsButton.addEventListener("click", async () => {
    try {
      resultDiv.innerHTML = "<p>Requesting accounts...</p>";

      // Get accounts using either WASM or JavaScript fallback
      let accounts;
      try {
        // Try WASM function
        const accountsValue = await callWasmFunction("get_ethereum_accounts");

        // Process the result
        try {
          if (typeof accountsValue === "string") {
            accounts = JSON.parse(accountsValue);
          } else {
            accounts = accountsValue;
          }
        } catch (parseError) {
          console.error("Error parsing accounts:", parseError);
          accounts = [accountsValue.toString()];
        }
      } catch (wasmError) {
        // JavaScript fallback
        console.warn(
          "WebAssembly get_ethereum_accounts failed, using JS fallback",
          wasmError
        );

        if (!ethereum) {
          throw new Error("No Ethereum provider found");
        }

        accounts = await ethereum.request({
          method: "eth_requestAccounts",
        });
      }

      console.log("Accounts:", accounts);

      if (
        accounts &&
        (Array.isArray(accounts) ? accounts.length > 0 : accounts)
      ) {
        // Convert single account to array if not already
        const accountsList = Array.isArray(accounts) ? accounts : [accounts];

        resultDiv.innerHTML = `
          <h3>Your Ethereum Accounts:</h3>
          <ul class="accounts-list">
            ${accountsList
              .map(
                (account) => `
              <li>
                <code>${account}</code>
                <button class="copy-btn" data-address="${account}">Copy</button>
              </li>
            `
              )
              .join("")}
          </ul>
        `;

        // Set up copy buttons
        document.querySelectorAll(".copy-btn").forEach((button) => {
          const address = button.getAttribute("data-address");
          button.addEventListener("click", async () => {
            try {
              await navigator.clipboard.writeText(address);
              button.textContent = "Copied!";
              setTimeout(() => {
                button.textContent = "Copy";
              }, 2000);
            } catch (err) {
              console.error("Copy failed:", err);
            }
          });
        });
      } else {
        resultDiv.innerHTML = `
          <p class="warning">⚠️ No accounts found.</p>
          <p>Please unlock your Ethereum wallet.</p>
        `;
      }
    } catch (error) {
      const errorMessage = utilsModule.parseRpcError(error);
      resultDiv.innerHTML = `
        <p class="error">❌ Error: ${errorMessage}</p>
      `;
    }
  });

  // Add functionality to Get Network Info button
  networkButton.addEventListener("click", async () => {
    try {
      resultDiv.innerHTML = "<p>Getting network information...</p>";

      // Get network info using JavaScript fallback
      let networkInfo;

      try {
        // Try to use the WebAssembly function first
        const networkInfoValue = await callWasmFunction("get_network_info");

        try {
          // Parse as JSON if it's a string
          if (typeof networkInfoValue === "string") {
            networkInfo = JSON.parse(networkInfoValue);
          } else {
            // Use directly if it's already an object
            networkInfo = networkInfoValue;
          }
        } catch (parseError) {
          console.error("Error parsing network info:", parseError);
          // Fallback network info
          networkInfo = { chainId: "0x0" };
        }
      } catch (wasmError) {
        // JavaScript fallback
        console.warn(
          "WebAssembly get_network_info failed, using JS fallback",
          wasmError
        );

        if (!ethereum) {
          throw new Error("No Ethereum provider found");
        }

        const chainId = await ethereum.request({ method: "eth_chainId" });
        networkInfo = { chainId };
      }

      // Convert chain ID to decimal format
      const chainIdHex = networkInfo.chainId || "0x0";
      let chainIdDec = parseInt(chainIdHex, 16).toString();

      // Get network name
      let networkName = "Unknown Network";
      try {
        const networkData = await ethUtilsModule.getNetworkInfo();
        networkName = networkData.networkName;
      } catch (err) {
        console.error("Error getting network name:", err);
      }

      resultDiv.innerHTML = `
        <h3>Network Information:</h3>
        <div class="network-info">
          <p><strong>Chain ID (Hex):</strong> ${chainIdHex}</p>
          <p><strong>Chain ID (Decimal):</strong> ${chainIdDec}</p>
          <p><strong>Network:</strong> ${networkName}</p>
        </div>
      `;
    } catch (error) {
      const errorMessage = utilsModule.parseRpcError(error);
      resultDiv.innerHTML = `
        <p class="error">❌ Error: ${errorMessage}</p>
      `;
    }
  });

  // Add contract interaction functionality if contractSection exists
  if (contractSection) {
    setupContractInteraction(
      resultDiv,
      contractUtilsModule,
      utilsModule,
      ethUtilsModule
    );
  }
}

// Setup contract interaction UI functionality
function setupContractInteraction(resultDiv, contractUtils, utils, ethUtils) {
  // Add ERC-20 template button
  const contractAbiInput = document.getElementById("contract-abi");
  if (!contractAbiInput) {
    console.error("contractAbiInput element not found");
    return;
  }

  const abiTemplateButton = document.createElement("button");
  abiTemplateButton.textContent = "Load ERC-20 ABI";
  abiTemplateButton.className = "template-btn";
  abiTemplateButton.style.fontSize = "0.8rem";
  abiTemplateButton.style.padding = "0.25rem 0.5rem";
  abiTemplateButton.style.marginTop = "0.25rem";

  // Insert after the ABI textarea
  contractAbiInput.parentNode.insertBefore(
    abiTemplateButton,
    contractAbiInput.nextSibling
  );

  // Add click event
  abiTemplateButton.addEventListener("click", () => {
    contractAbiInput.value = ethUtils.SAMPLE_ERC20_ABI;
    resultDiv.innerHTML = `<p class="success">✅ Loaded ERC-20 token ABI template</p>`;
  });

  // Deploy Contract
  const deployContractBtn = document.getElementById("deploy-contract-btn");
  deployContractBtn.addEventListener("click", async () => {
    try {
      const bytecode = document
        .getElementById("contract-bytecode")
        .value.trim();
      const abi = document.getElementById("contract-abi").value.trim();
      const constructorArgsString = document
        .getElementById("constructor-args")
        .value.trim();

      // Basic validation
      if (!bytecode) {
        resultDiv.innerHTML =
          '<p class="error">❌ Contract bytecode is required</p>';
        return;
      }

      if (!abi) {
        resultDiv.innerHTML =
          '<p class="error">❌ Contract ABI is required</p>';
        return;
      }

      // Parse constructor args (this is simplified)
      const args = constructorArgsString
        ? constructorArgsString.split(",").map((arg) => arg.trim())
        : [];

      resultDiv.innerHTML = "<p>Deploying contract...</p>";

      // Deploy the contract
      const deployResult = await contractUtils.deployContract({
        bytecode,
        abi,
        args,
        deployOptions: {},
      });

      resultDiv.innerHTML = `
        <h3>Contract Deployment</h3>
        <div class="success">✅ Contract deployment transaction sent!</div>
        <div class="network-info">
          <p><strong>Transaction Hash:</strong> <code>${deployResult.transactionHash}</code></p>
          <p><strong>From:</strong> <code>${deployResult.from}</code></p>
          <p><em>Note: Contract address will be available once the transaction is mined.</em></p>
        </div>
      `;
    } catch (error) {
      const errorMessage = utils.parseRpcError(error);
      resultDiv.innerHTML = `
        <p class="error">❌ Deployment Error: ${errorMessage}</p>
      `;
    }
  });

  // Contract Function Call
  const callFunctionBtn = document.getElementById("call-function-btn");
  callFunctionBtn.addEventListener("click", async () => {
    try {
      const address = document.getElementById("contract-address").value.trim();
      const functionName = document
        .getElementById("function-name")
        .value.trim();
      const functionArgsString = document
        .getElementById("function-args")
        .value.trim();
      const functionType = document.getElementById("function-type").value;

      // Basic validation
      if (!address || !utils.isValidEthAddress(address)) {
        resultDiv.innerHTML =
          '<p class="error">❌ Valid contract address is required</p>';
        return;
      }

      if (!functionName) {
        resultDiv.innerHTML =
          '<p class="error">❌ Function name is required</p>';
        return;
      }

      // Get ABI from previous form
      const abi = document.getElementById("contract-abi").value.trim();
      if (!abi) {
        resultDiv.innerHTML =
          '<p class="error">❌ Contract ABI is required</p>';
        return;
      }

      // Parse function args (this is simplified)
      const args = functionArgsString
        ? functionArgsString.split(",").map((arg) => arg.trim())
        : [];

      resultDiv.innerHTML = "<p>Calling contract function...</p>";

      // Determine if this is a view function
      const isView = functionType === "view";

      // Call the contract function
      const callResult = await contractUtils.callContract({
        address,
        abi,
        method: functionName,
        args,
        isView,
      });

      if (isView) {
        resultDiv.innerHTML = `
          <h3>Function Result</h3>
          <div class="success">✅ Call successful!</div>
          <div class="network-info">
            <p><strong>Result:</strong> <code>${callResult}</code></p>
          </div>
        `;
      } else {
        resultDiv.innerHTML = `
          <h3>Transaction Sent</h3>
          <div class="success">✅ Transaction sent!</div>
          <div class="network-info">
            <p><strong>Transaction Hash:</strong> <code>${callResult}</code></p>
          </div>
        `;
      }
    } catch (error) {
      const errorMessage = utils.parseRpcError(error);
      resultDiv.innerHTML = `
        <p class="error">❌ Function Call Error: ${errorMessage}</p>
      `;
    }
  });

  // Event Listening
  const listenEventsBtn = document.getElementById("listen-events-btn");
  const stopEventsBtn = document.getElementById("stop-events-btn");
  const eventsLog = document.getElementById("events-log");
  const eventsList = document.getElementById("events-list");

  let eventUnsubscribe = null;

  listenEventsBtn.addEventListener("click", async () => {
    try {
      const address = document
        .getElementById("event-contract-address")
        .value.trim();
      const eventName = document.getElementById("event-name").value.trim();

      // Basic validation
      if (!address || !utils.isValidEthAddress(address)) {
        resultDiv.innerHTML =
          '<p class="error">❌ Valid contract address is required</p>';
        return;
      }

      if (!eventName) {
        resultDiv.innerHTML = '<p class="error">❌ Event name is required</p>';
        return;
      }

      // Get ABI from previous form
      const abi = document.getElementById("contract-abi").value.trim();
      if (!abi) {
        resultDiv.innerHTML =
          '<p class="error">❌ Contract ABI is required</p>';
        return;
      }

      // Show events log and stop button
      eventsLog.style.display = "block";
      stopEventsBtn.style.display = "inline-block";
      listenEventsBtn.style.display = "none";

      // Clear previous events
      eventsList.innerHTML =
        '<div class="event-item">Listening for events...</div>';

      // Subscribe to events
      eventUnsubscribe = await contractUtils.subscribeToEvent(
        {
          address,
          abi,
          eventName,
        },
        (event) => {
          // This is a placeholder as we don't have real event subscription in our simplified demo
          const eventItem = document.createElement("div");
          eventItem.className = "event-item";
          eventItem.innerHTML = `
          <div class="event-name">${eventName}</div>
          <div class="event-data">${JSON.stringify(event)}</div>
        `;
          eventsList.appendChild(eventItem);
        }
      );

      resultDiv.innerHTML = `
        <div class="success">✅ Listening for ${eventName} events on contract ${address}</div>
        <p>Event logs will appear in the events log section below.</p>
      `;
    } catch (error) {
      const errorMessage = utils.parseRpcError(error);
      resultDiv.innerHTML = `
        <p class="error">❌ Event Subscription Error: ${errorMessage}</p>
      `;

      // Reset UI
      eventsLog.style.display = "none";
      stopEventsBtn.style.display = "none";
      listenEventsBtn.style.display = "inline-block";
    }
  });

  stopEventsBtn.addEventListener("click", () => {
    if (eventUnsubscribe) {
      eventUnsubscribe();
      eventUnsubscribe = null;
    }

    // Update UI
    eventsLog.style.display = "none";
    stopEventsBtn.style.display = "none";
    listenEventsBtn.style.display = "inline-block";

    resultDiv.innerHTML = `
      <div class="success">✅ Stopped listening for events</div>
    `;
  });
}

// Error handling
window.addEventListener("error", function (e) {
  console.error("Global error caught:", e.error || e.message);
  const statusEl = document.getElementById("web3-status");
  if (statusEl) {
    statusEl.textContent = "Error: " + (e.error ? e.error.message : e.message);
    statusEl.className = "status-error";
  }
});

// Page load check
window.addEventListener("load", function () {
  console.log("Page fully loaded");
  console.log("window.ethereum:", window.ethereum);

  // If status doesn't change within 5 seconds, there's an error
  setTimeout(function () {
    const statusEl = document.getElementById("web3-status");
    if (statusEl && statusEl.textContent === "Checking...") {
      statusEl.textContent =
        "Error: WebAssembly couldn't be loaded. Check the console.";
      statusEl.className = "status-error";
    }
  }, 5000);
});
