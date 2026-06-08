<script>
  import SwapForm from './components/SwapForm.svelte';
  import PoolManager from './components/PoolManager.svelte';
  
  let activeTab = 'swap';
</script>

<main>
  <header>
    <h1>🎻 Concertino</h1>
    <p>Solana DeFi AMM - Classical Orchestral Movements</p>
  </header>

  <nav class="tabs">
    <button
      class:active={activeTab === 'swap'}
      on:click={() => (activeTab = 'swap')}
    >
      🎵 Swap (Musical Movement)
    </button>
    <button
      class:active={activeTab === 'liquidity'}
      on:click={() => (activeTab = 'liquidity')}
    >
      🎼 Liquidity (Maestro Section)
    </button>
  </nav>

  <div class="container">
    {#if activeTab === 'swap'}
      <SwapForm />
    {:else if activeTab === 'liquidity'}
      <PoolManager />
    {/if}
  </div>

  <footer>
    <p>🎻 Made with Classical Music & Web3 Magic</p>
    <p>Vatican Orchestral Movements • Solana Devnet</p>
  </footer>
</main>

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(body) {
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    background: linear-gradient(135deg, #1a0033 0%, #330066 100%);
    color: #fff;
    min-height: 100vh;
  }

  main {
    max-width: 900px;
    margin: 0 auto;
    padding: 20px;
  }

  header {
    text-align: center;
    margin-bottom: 40px;
    padding: 40px 20px;
    background: rgba(255, 215, 0, 0.1);
    border-radius: 10px;
    border: 2px solid rgba(255, 215, 0, 0.3);
  }

  h1 {
    font-size: 3em;
    margin-bottom: 10px;
    color: #ffd700;
    text-shadow: 0 0 20px rgba(255, 215, 0, 0.5);
  }

  header p {
    font-size: 1.1em;
    color: #daa520;
  }

  .tabs {
    display: flex;
    gap: 10px;
    margin-bottom: 30px;
    justify-content: center;
  }

  button {
    padding: 12px 24px;
    font-size: 1em;
    border: 2px solid #ffd700;
    background: transparent;
    color: #ffd700;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  button:hover {
    background: rgba(255, 215, 0, 0.1);
    transform: translateY(-2px);
  }

  button.active {
    background: #ffd700;
    color: #1a0033;
    font-weight: bold;
  }

  .container {
    background: rgba(51, 0, 102, 0.5);
    padding: 40px;
    border-radius: 15px;
    border: 2px solid rgba(255, 215, 0, 0.2);
    backdrop-filter: blur(10px);
  }

  footer {
    text-align: center;
    margin-top: 40px;
    padding: 20px;
    border-top: 2px solid rgba(255, 215, 0, 0.2);
    font-size: 0.9em;
    color: #daa520;
  }
</style>
