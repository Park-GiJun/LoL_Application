<script>
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  let teamOrder = [];
  let teamChaos = [];

  async function getLogFile() {
    try {
      const result = await invoke('get_log_file');
      teamOrder = result[0];
      teamChaos = result[1];
      console.log("Team Order:", teamOrder);
      console.log("Team Chaos:", teamChaos);
    } catch (error) {
      console.error('Error reading log file:', error);
      teamOrder = ['Error reading log file'];
      teamChaos = ['Error reading log file'];
    }
  }

  onMount(() => {
    getLogFile();
  });
</script>

<main class="min-h-screen bg-gray-100 flex flex-col items-center justify-center">
  <button on:click={getLogFile} class="px-4 py-2 bg-blue-500 text-white rounded">Read Log File</button>
  <div class="mt-4 p-4 bg-white rounded shadow-md w-3/4 overflow-x-auto">
    <h2 class="text-2xl font-bold">Team Order</h2>
    <ul>
      {#each teamOrder as player}
        <li>{player}</li>
      {/each}
    </ul>
    <h2 class="text-2xl font-bold mt-4">Team Chaos</h2>
    <ul>
      {#each teamChaos as player}
        <li>{player}</li>
      {/each}
    </ul>
  </div>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    margin: 0 auto;
  }

  button {
    margin: 20px;
    padding: 10px 20px;
    font-size: 16px;
  }

  ul {
    list-style: none;
    padding: 0;
  }

  li {
    padding: 5px 0;
    border-bottom: 1px solid #ddd;
  }
</style>
