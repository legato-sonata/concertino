<script>
  import { onMount } from 'svelte';
  
  let selectedPool = 'Violins';
  let amountA = '';
  let amountB = '';
  let lpTokens = 0;
  let loading = false;
  let message = '';
  let poolInfo = null;
  
  const pools = ['Violins', 'Cellos', 'Violas'];

  onMount(async () => {
    await loadPoolInfo();
  });

  async function loadPoolInfo() {
    try {
      const response = await fetch(`/api/pool/${selectedPool}`);
      if (response.ok) {
        poolInfo = await response.json();
      }
    } catch (error) {
      message = '⚠️ Could not load pool info';
    }
  }

  async function addLiquidity() {
    if (!amountA || !amountB) {
      message = '🎻 Enter both token amounts, maestro!';
      return;
    }

    loading = true;
    message = '🎼 Joining the orchestra...';

    try {
      const response = await fetch(`/api/liquidity/${selectedPool}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          amount_a: BigInt(amountA),
          amount_b: BigInt(amountB),
        }),
      });

      if (response.ok) {
        const data = await response.json();
        lpTokens = data.lp_tokens;
        message = '✨ ' + data.message;
        amountA = '';
        amountB = '';
        await loadPoolInfo();
      }
    } catch (error) {
      message = '❌ Dissonance: ' + error.message;
    } finally {
      loading = false;
    }
  }

  async function handlePoolChange() {
    await loadPoolInfo();
  }
</script>

<div class="pool-manager">
  <h2>🎼 Maestro Liquidity (Add Liquidity)</h2>

  <div class="form-group">
    <label for="pool">Orchestral Section</label>
    <select id="pool" bind:value={selectedPool} on:change={handlePoolChange}>
      {#each pools as pool}
        <option value={pool}>{pool}</option>
      {/each}
    </select>
  </div>

  {#if poolInfo}
    <div class="pool-stats">
      <div class="stat">
        <span>Reserve A</span>
        <strong>{poolInfo.reserve_a.toLocaleString()}</strong>
      </div>
      <div class="stat">
        <span>Reserve B</span>
        <strong>{poolInfo.reserve_b.toLocaleString()}</strong>
      </div>
      <div class="stat">
        <span>Fee</span>
        <strong>{(poolInfo.fee_basis_points / 100).toFixed(2)}%</strong>
      </div>
    </div>
  {/if}

  <div class="form-group">
    <label for="amountA">Token A Amount</label>
    <input
      id="amountA"
      type="number"
      bind:value={amountA}
      placeholder="Enter amount"
      disabled={loading}
    />
  </div>

  <div class="form-group">
    <label for="amountB">Token B Amount</label>
    <input
      id="amountB"
      type="number"
      bind:value={amountB}
      placeholder="Enter amount"
      disabled={loading}
    />
  </div>

  {#if lpTokens > 0}
    <div class="reward">
      <p>🎻 LP Tokens Minted: <strong>{lpTokens.toLocaleString()}</strong></p>
    </div>
  {/if}

  <button on:click={addLiquidity} disabled={loading || !amountA || !amountB}>
    {loading ? '🎵 Harmonizing...' : '🎼 Add Liquidity'}
  </button>

  {#if message}
    <div class="message">{message}</div>
  {/if}
</div>

<style>
  .pool-manager {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  h2 {
    color: #ffd700;
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
  }

  input:focus,
  select:focus {
    outline: none;
    border-color: #ffd700;
    box-shadow: 0 0 10px rgba(255, 215, 0, 0.3);
  }

  .pool-stats {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 15px;
    margin: 15px 0;
  }

  .stat {
    background: rgba(255, 215, 0, 0.1);
    padding: 15px;
    border-radius: 8px;
    border: 1px solid rgba(255, 215, 0, 0.2);
    text-align: center;
  }

  .stat span {
    display: block;
    font-size: 0.9em;
    color: #daa520;
    margin-bottom: 5px;
  }

  .stat strong {
    color: #ffd700;
    font-size: 1.2em;
  }

  .reward {
    background: rgba(0, 255, 0, 0.1);
    padding: 15px;
    border-radius: 8px;
    border-left: 3px solid #00ff00;
    color: #00ff00;
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
