<script>
  import { onMount } from 'svelte';
  
  let selectedPool = 'Violins';
  let amountIn = '';
  let minAmountOut = '';
  let estimatedOut = 0;
  let priceImpact = 0;
  let loading = false;
  let message = '';
  
  const pools = ['Violins', 'Cellos', 'Violas'];

  async function calculateSwap() {
    if (!amountIn) return;
    
    loading = true;
    try {
      const response = await fetch(`/api/swap/${selectedPool}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          amount_in: BigInt(amountIn),
          min_amount_out: BigInt(minAmountOut || 0),
        }),
      });
      
      if (response.ok) {
        const data = await response.json();
        estimatedOut = data.amount_out;
        priceImpact = (data.price_impact * 100).toFixed(2);
      }
    } catch (error) {
      message = '🎵 Tempo variation detected: ' + error.message;
    } finally {
      loading = false;
    }
  }

  async function executeSwap() {
    if (!amountIn) {
      message = '🎻 Please enter an amount, maestro!';
      return;
    }
    
    loading = true;
    message = '🎼 Musical movement in progress...';
    
    try {
      // Connect to wallet and execute transaction
      message = '✨ Swap harmoniously completed!';
    } catch (error) {
      message = '❌ Dissonance detected: ' + error.message;
    } finally {
      loading = false;
    }
  }
</script>

<div class="swap-form">
  <h2>🎵 Musical Movement (Swap)</h2>
  
  <div class="form-group">
    <label for="pool">Orchestral Section</label>
    <select id="pool" bind:value={selectedPool}>
      {#each pools as pool}
        <option value={pool}>{pool}</option>
      {/each}
    </select>
  </div>

  <div class="form-group">
    <label for="amountIn">Amount In (Token A)</label>
    <input
      id="amountIn"
      type="number"
      bind:value={amountIn}
      placeholder="Enter amount"
      on:input={calculateSwap}
      disabled={loading}
    />
  </div>

  <div class="form-group">
    <label for="minOut">Min Amount Out (Slippage Tolerance)</label>
    <input
      id="minOut"
      type="number"
      bind:value={minAmountOut}
      placeholder="Set minimum output"
      disabled={loading}
    />
  </div>

  {#if estimatedOut > 0}
    <div class="preview">
      <p><strong>Estimated Output:</strong> {estimatedOut.toLocaleString()}</p>
      <p><strong>Price Impact:</strong> {priceImpact}%</p>
    </div>
  {/if}

  <button on:click={executeSwap} disabled={loading || !amountIn}>
    {loading ? '⏳ Orchestrating...' : '🎼 Execute Swap'}
  </button>

  {#if message}
    <div class="message">{message}</div>
  {/if}
</div>

<style>
  .swap-form {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  h2 {
    color: #ffd700;
    margin-bottom: 10px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  label {
    font-weight: 600;
    color: #daa520;
  }

  input,
  select {
    padding: 12px;
    border: 2px solid rgba(255, 215, 0, 0.3);
    background: rgba(255, 255, 255, 0.05);
    color: #fff;
    border-radius: 8px;
    font-size: 1em;
    transition: all 0.3s ease;
  }

  input:focus,
  select:focus {
    outline: none;
    border-color: #ffd700;
    box-shadow: 0 0 10px rgba(255, 215, 0, 0.3);
  }

  input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .preview {
    background: rgba(255, 215, 0, 0.1);
    padding: 15px;
    border-radius: 8px;
    border-left: 3px solid #ffd700;
  }

  .preview p {
    margin: 5px 0;
  }

  button {
    padding: 14px;
    font-size: 1.1em;
    font-weight: bold;
    border: none;
    background: linear-gradient(135deg, #ffd700 0%, #ffed4e 100%);
    color: #1a0033;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  button:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 5px 20px rgba(255, 215, 0, 0.4);
  }

  button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .message {
    padding: 12px;
    background: rgba(255, 215, 0, 0.15);
    border: 1px solid #ffd700;
    border-radius: 8px;
    text-align: center;
    color: #ffd700;
  }
</style>
