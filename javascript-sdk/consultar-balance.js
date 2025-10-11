import pkg from '@stellar/stellar-sdk';
const { Horizon } = pkg;

const server = new Horizon.Server('https://horizon-testnet.stellar.org');

const cuentas = [
  'GDETARYVCCTO4RP4B2OXOBWZOQQPDMIEI7FKTJIBJGSW3VV4EII7OUEM',
  'GCIRCVVFWMLCMVGMSQIAXKGHKOOQWNREIUHPMPCZFQIBVU25GY7LFAPV',
  'GBMF5BXTP2JIIOJQOZXK4CXQ72ZQQEGPP3VZFWVNXIXHWMYYWLYBG62W'
];

async function consultarBalances() {
  console.log('=== MONITOR DE CUENTAS ===\n');
  
  for (const publicKey of cuentas) {
    try {
      const account = await server.loadAccount(publicKey);
      
      const xlmBalance = account.balances.find(b => b.asset_type === 'native');
      const trustlines = account.balances.filter(b => b.asset_type !== 'native').length;
      
      console.log(`Cuenta: ${publicKey.substring(0, 5)}...${publicKey.substring(publicKey.length - 3)}`);
      console.log(`  Balance: ${xlmBalance.balance} XLM`);
      console.log(`  Trustlines: ${trustlines}`);
      console.log(`  Sequence: ${account.sequenceNumber()}\n`);
      
    } catch (error) {
      console.error(`❌ Error consultando ${publicKey.substring(0, 8)}...:`, error.message);
    }
  }
  
  console.log('✅ Consulta completada\n');
}

consultarBalances();
