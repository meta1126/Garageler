<script lang="ts">
    import {onMount} from 'svelte';
    import {listen} from '@tauri-apps/api/event';
    import { event } from '@tauri-apps/api';

    let errorMessage: string | null = null;

    onMount(() => {
        listen('display_error', (event) => {
            errorMessage ='エラーが発生しました: ${event.payload}';
        });
    });
</script>

<main>
    {#if errorMessage}
    <div class="error-message">
        {errorMessage}
    </div>
    {/if}
</main>

<style>
    .error-message {
        color: red;
        font-weight: bold;
    }
</style>