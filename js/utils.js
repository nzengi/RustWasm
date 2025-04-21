// Utils.js - General helper functions

// Convert hex value to decimal
export function hexToDecimal(hexValue) {
  if (!hexValue.startsWith("0x")) {
    throw new Error("Not a valid hex string, must start with 0x");
  }
  return parseInt(hexValue, 16).toString();
}

// Convert decimal value to hex
export function decimalToHex(decimalValue) {
  const number = parseInt(decimalValue, 10);
  if (isNaN(number)) {
    throw new Error("Not a valid decimal number");
  }
  return "0x" + number.toString(16);
}

// Check if an Ethereum address is valid
export function isValidEthAddress(address) {
  return /^0x[0-9a-fA-F]{40}$/.test(address);
}

// Convert UNIX timestamp to readable date
export function timestampToDate(timestamp) {
  const date = new Date(timestamp * 1000); // Ethereum timestamps are in seconds
  return date.toLocaleString();
}

// Parse JSON RPC error messages into user-friendly messages
export function parseRpcError(error) {
  if (!error) return "Unknown error";

  // MetaMask error messages are usually in error.message
  if (error.message) {
    if (error.message.includes("User denied")) {
      return "Transaction was rejected by the user.";
    }
    if (error.message.includes("insufficient funds")) {
      return "Insufficient balance. Not enough ETH for the transaction.";
    }
    if (error.message.includes("gas required exceeds")) {
      return "Gas limit exceeded. Transaction is too complex or gas limit is too low.";
    }
    return error.message;
  }

  // If there's a code, create an error message based on it
  if (error.code) {
    switch (error.code) {
      case 4001:
        return "Transaction rejected by the user.";
      case -32603:
        return "Internal JSON-RPC error.";
      case -32000:
        return "Invalid input.";
      default:
        return `RPC Error: Code ${error.code}`;
    }
  }

  return "An unknown error occurred.";
}

// Helper function for creating DOM elements
export function createElement(tag, attributes = {}, children = []) {
  const element = document.createElement(tag);

  // Set attributes
  Object.entries(attributes).forEach(([key, value]) => {
    if (key === "style" && typeof value === "object") {
      Object.entries(value).forEach(([styleKey, styleValue]) => {
        element.style[styleKey] = styleValue;
      });
    } else if (key.startsWith("on") && typeof value === "function") {
      element.addEventListener(key.substring(2).toLowerCase(), value);
    } else {
      element.setAttribute(key, value);
    }
  });

  // Add child elements
  children.forEach((child) => {
    if (typeof child === "string") {
      element.appendChild(document.createTextNode(child));
    } else if (child instanceof Node) {
      element.appendChild(child);
    }
  });

  return element;
}

// LocalStorage helpers for saving and retrieving data
export function saveToStorage(key, data) {
  try {
    localStorage.setItem(key, JSON.stringify(data));
    return true;
  } catch (error) {
    console.error("Error saving to localStorage:", error);
    return false;
  }
}

export function loadFromStorage(key) {
  try {
    const data = localStorage.getItem(key);
    return data ? JSON.parse(data) : null;
  } catch (error) {
    console.error("Error loading from localStorage:", error);
    return null;
  }
}

// Copy button functionality
export function setupCopyButton(buttonId, textToCopy) {
  const button = document.getElementById(buttonId);
  if (!button) return;

  button.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(textToCopy);
      const originalText = button.textContent;
      button.textContent = "Copied!";

      setTimeout(() => {
        button.textContent = originalText;
      }, 2000);
    } catch (error) {
      console.error("Copy failed:", error);
    }
  });
}
