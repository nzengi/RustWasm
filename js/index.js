import init, { greet, connect_to_ethereum } from './pkg/rustwasm_eth.js';

async function run() {
    await init();

    // Greet fonksiyonunu çağırma
    greet();

    // Ethereum bağlantısını başlatma
    try {
        const ethMessage = connect_to_ethereum();
        console.log("Ethereum connection status:", ethMessage);
    } catch (e) {
        console.error("Error connecting to Ethereum:", e);
    }
}

run();
