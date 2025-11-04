import { useState } from 'react'
import './App.css'

function App() {
  const [publicKey, setPublicKey] = useState<string>('')
  const [connected, setConnected] = useState<boolean>(false)

  const connectWallet = async () => {
    try {
      const freighter = (window as any).freighter
      
      if (!freighter) {
        alert('Instalá Freighter: https://freighter.app')
        return
      }

      const key = await freighter.getPublicKey()
      
      setPublicKey(key)
      setConnected(true)
      
      console.log('Wallet conectada:', key)
      
    } catch (error: any) {
      console.error('Error:', error)
      alert('Error: ' + error.message)
    }
  }

  return (
    <div style={{ padding: '40px', maxWidth: '600px', margin: '0 auto' }}>
      <h1>🦈 Token BDB</h1>
      <p>by Jenny Tejedor - Equipo 14</p>
      
      {!connected ? (
        <div>
          <p>Conectá tu Freighter wallet</p>
          <button 
            onClick={connectWallet}
            style={{
              padding: '12px 24px',
              fontSize: '16px',
              backgroundColor: '#0088cc',
              color: 'white',
              border: 'none',
              borderRadius: '8px',
              cursor: 'pointer'
            }}
          >
            🔗 Conectar Wallet
          </button>
        </div>
      ) : (
        <div>
          <p>✅ Wallet Conectada</p>
          <div style={{ 
            background: '#f5f5f5', 
            padding: '15px', 
            borderRadius: '8px',
            wordBreak: 'break-all',
            fontSize: '12px',
            fontFamily: 'monospace'
          }}>
            {publicKey}
          </div>
          <button 
            onClick={() => {
              setConnected(false)
              setPublicKey('')
            }}
            style={{ 
              marginTop: '15px',
              padding: '8px 16px',
              backgroundColor: '#ff6b6b',
              color: 'white',
              border: 'none',
              borderRadius: '6px',
              cursor: 'pointer'
            }}
          >
            Desconectar
          </button>
        </div>
      )}
    </div>
  )
}

export default App
