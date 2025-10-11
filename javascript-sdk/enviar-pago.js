import pkg from '@stellar/stellar-sdk';
const { Keypair, Horizon, TransactionBuilder, Networks, Operation, Asset, BASE_FEE, Memo } = pkg;

const server = new Horizon.Server('https://horizon-testnet.stellar.org');
const networkPassphrase = Networks.TESTNET;

// ⚠️ IMPORTANTE: Reemplaza con tu SECRET KEY real antes de ejecutar
// Esta es una cuenta de testnet, nunca uses secret keys reales aquí
const SECRET_KEY = 'TU_SECRET_KEY_AQUI';

const destinatarios = [
  { publicKey: "GDETARYVCCTO4RP4B2OXOBWZOQQPDMIEI7FKTJIBJGSW3VV4EII7OUEM", memo: "Tiburoncin Uh Ha Ha 1" },
  { publicKey: "GCIRCVVFWMLCMVGMSQIAXKGHKOOQWNREIUHPMPCZFQIBVU25GY7LFAPV", memo: "Tiburoncin Uh Ha Ha 2" },
  { publicKey: "GBMF5BXTP2JIIOJQOZXK4CXQ72ZQQEGPP3VZFWVNXIXHWMYYWLYBG62W", memo: "Tiburoncin Uh Ha Ha 3" }
];

async function enviarPagos() {
  if (SECRET_KEY === 'TU_SECRET_KEY_AQUI') {
    console.error('❌ ERROR: Debes configurar tu SECRET_KEY en el código antes de ejecutar');
    console.log('💡 Edita el archivo y reemplaza TU_SECRET_KEY_AQUI con tu secret key real\n');
    return;
  }

  const sourceKeys = Keypair.fromSecret(SECRET_KEY);
  
  console.log(`\n💰 Cuenta origen: ${sourceKeys.publicKey()}\n`);
  
  for (let i = 0; i < destinatarios.length; i++) {
    const destino = destinatarios[i];
    
    try {
      console.log(`🦈 Enviando pago ${i + 1} de 3...`);
      console.log(`   Destino: ${destino.publicKey.substring(0, 8)}...`);
      console.log(`   Memo: ${destino.memo}`);
      
      const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
      
      const transaction = new TransactionBuilder(sourceAccount, {
        fee: BASE_FEE,
        networkPassphrase: networkPassphrase
      })
        .addOperation(Operation.payment({
          destination: destino.publicKey,
          asset: Asset.native(),
          amount: '2'
        }))
        .addMemo(Memo.text(destino.memo))
        .setTimeout(30)
        .build();
      
      transaction.sign(sourceKeys);
      
      const result = await server.submitTransaction(transaction);
      
      console.log(`✅ ¡Transacción exitosa!`);
      console.log(`🔗 Hash: ${result.hash}\n`);
      
    } catch (error) {
      console.error(`❌ Error en pago ${i + 1}:`, error.message);
    }
  }
  
  console.log('🎉 ¡Todos los pagos de Tiburoncín completados!\n');
}

enviarPagos();
