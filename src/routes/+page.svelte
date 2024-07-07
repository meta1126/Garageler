<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import {emit, listen} from '@tauri-apps/api/event'
  import {appWindow} from "@tauri-apps/api/window"
  import { X, Minus, Maximize, AlignHorizontalJustifyCenter } from 'lucide-svelte'

  let name = "";
  let greetMsg = "";
  let number = "";
  let fname ="";
  let filesearchMsg =""

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }
async function test() {  
  emit('click', {theMessage: 'this is test'});
  }
  async function filesearch() {
    filesearchMsg =await invoke("filesearch", {fname});
  }

  function minimize(){
    appWindow.minimize()
  }
async function maximize() {
  let maximizeState = await appWindow.isMaximized()

  if(!maximizeState) {
    appWindow.maximize()
  }else{
    appWindow.unmaximize()
  }
}

function hide(){
  appWindow.hide()
}
</script>
<div>
  <header class="bg-zinc-800 h12 select-none bg-fixe">
    <nav data-tauri-drag-region class="mx-auto flex items-center justify-between" aria-label=":global">
      <div class="mx-1 p-3" >
        <AlignHorizontalJustifyCenter class="square-4" />
      </div>
      <ul class="Flex gap-x-1">
        <li>
          <button on:click={minimize} class="window-control-button">
            <minus size="16" />
          </button>
        </li>
          <li>
            <button on:click={maximize} class="window-control-button">
              <maximize size="16" />
            </button>
          </li>
        <li>
          <button on:click={hide} class="window-control-button">
            <X size="16" />
          </button>
        </li>
      </ul>
    </nav>
  </header>
  <div class= "bg-zinc-900 h-[1px]"></div>
  <main class= "bg-zinc-800 h-screen">
    <slot />
  </main>
</div>

<div class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://kit.svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>

  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>

  <p>{greetMsg}</p>
  <button on:click={test}>test</button>

  <form class="row2" on:submit|preventDefault={filesearch}>
    <input id="fname-input" placeholder="Enter a filename(filepath)" bind:value={fname} />
    <button type="submit">ファイルチェック</button>

    <p>{filesearchMsg}</p>
  </form>

</div>
<style>
  
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte-kit:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 5;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  #greet-input {
    margin-right: 5px;
  }

  input,
  button {
    border-radius: 5px;
    border: 1px solid transparent;
    padding: 0.5em 0.8em;
    font-size: 1em;
    font-weight: 300;
    font-family: inherit;
    color: #2f2f2f;
    background-color: #ffffff;
    transition: border-color 0.21s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #535bf2;
  }
  button:active {
    border-color: #2f2f2f;
    background-color: #ff3e00;
  }

  button {
    border-radius: 5px;
    border: 1px solid transparent;
    padding: 0.5em 0.8em;
    font-size: 1em;
    font-weight: 300;
    font-family: inherit;
    color: #685c5c;
    background-color: #151515;
    transition: border-color 0.21s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #535bf2;
  }
  button:active {
    border-color: #2f2f2f;
    background-color: #ff3e00;
  }


  input,
  button {
    outline: none;
  }

  #fname-input {
    margin-right: 3px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
