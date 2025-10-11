import { Keypair } from '@stellar/stellar-sdk';

async function crearCuentas() {
  const cuentas = [];
  
  for (let i = 1; i <= 5; i++) {
    console.log(`\n🔐 Generando cuenta ${i}...\n`);
    
    const pair = Keypair.random();
    
    console.log('✅ ¡Cuenta creada!\n');
    console.log('📧 PUBLIC KEY (puedes compartir):');
    console.log(pair.publicKey());
    console.log('\n🔑 SECRET KEY (NUNCA COMPARTIR):');
    console.log(pair.secret());
    
    console.log('\n💰 Fondeando con Friendbot...');
    
    try {
      const response = await fetch(
        `https://friendbot.stellar.org/?addr=${pair.publicKey()}`
      );
      
      const result = await response.json();
      
      if (result.successful || response.ok) {
        console.log('✅ ¡Cuenta fondeada con 10,000 XLM!\n');
        console.log('🔗 Transaction hash:', result.hash);
        
        cuentas.push({
          publicKey: pair.publicKey(),
          secretKey: pair.secret(),
          balance: '10000'
        });
      }
    } catch (error) {
      console.error('❌ Error al fondear:', error.message);
    }
    
    console.log('\n⚠️  IMPORTANTE: Guarda estas llaves en un lugar seguro\n');
  }
  
  console.log('\n=== 📦 ARRAY DE CUENTAS CREADAS ===');
  console.log(cuentas);
  console.log(`\n✅ Total de cuentas creadas: ${cuentas.length}`);
}

crearCuentas();
