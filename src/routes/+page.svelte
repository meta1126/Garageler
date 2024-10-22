<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import {emit, listen} from '@tauri-apps/api/event'
  import type {FileList} from './types';
    import { onMount } from "svelte";

  interface FileList {
    directories: string[];
    files: string[];
  }

  let name = "";
  let greetMsg = "";
  let fname = ""
  let filesearchMsg = "";
  let path: string ='';
  let loading: boolean = false;
  let error: string | null = null;
  let directories: string[]= [];
  let files: string[]=[];

  onMount(async () => {
    const unlisten = await listen('error', (event) => {
      error = event.payload as string;
    });

    return () => {
      unlisten();
    };
  })
  

  async function listFilesanddirs() {
    if (!path) {
      error = 'パスを入力してください。';
      return;
    }

    loading = true;
    error = null;
     
    try{
      const result = await invoke<FileList>('list_files_and_dirs', {path: path,
        maxEntries: 500
      });
      directories = result.directories;
      files = result.files;
    }catch(err){
      console.error(`詳細なエラー：`, err);
      error = 'エラーが発生しました: ${err instanceof Error ? err.message : String(err)}';
    }finally{
      loading =false;
    }
    }
    
  
  
  async function test() {
    emit('click', {theMessage: 'this is test'});
  }
  async function filesearch() {
    filesearchMsg = await invoke("filesearch",{fname});
  }
</script>

{#if error}
<div class="error">{error}</div>
{/if}

<div class="container">
  <h1>Welcome to Tauri!</h1>

  
<main>
  <h1>フォルダーとファイル一覧</h1>
  <input bind:value={path} placeholder="ディレクトリパスを入力" />
  <button on:click={listFilesanddirs} disabled={loading}>ファイル一覧を取得</button>

  {#if loading}
  <p>読み込み中…</p>
  {:else if error}
  <p style="color: red;">{error}</p>
  {:else}
  <h2>{path}のフォルダー一覧:</h2>
  {#if directories.length >0}
  <ul>
    {#each directories as directoey}
    <list>{directories}</list>
   {/each}
  </ul>
  {:else}
  <p>フォルダーなし</p>
  {/if}

  <h2>{path}のファイル一覧</h2>
  {#if files.length > 0}
  <ul>
    {#each  files as file}
    <li>{file}</li>
    {/each}
  </ul>
  {:else}
  <p>ファイルはありません。</p>
  {/if}
  {/if}
</main>


  
  <button on:click={test}>test</button>

  <form class="row2" on:submit|preventDefault={filesearch}>
    <input id="fname-input" placeholder="Enter a filename(filepath)" bind:value={fname} />
    <button type="submit">ファイルチェック</button>

    <p>{filesearchMsg}</p>
  </form>

  <p>{greetMsg}</p>
</div>

<style>
  .error{
    color: red;
    font-weight: bold;
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
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
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

  main {
    padding: 1em;
    max-width: 800px;
    margin: 0 auto;
  }
  input, button {
    margin-bottom: 1em;
  }
  ul {
    list-style-type: none;
    padding: 0;
  }
  li {
    margin-bottom: 0.5em;
  }

  input,
  button {
    outline: none;
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

{#if error}
  <p style="color: red;">
    エラー: {error}
    {#if error.includes('Failed to list files')}
    <br>
    ファイルの一覧取得に失敗しました。パスが正しいか確認してください。
    {/if}
    </p>
   {/if}