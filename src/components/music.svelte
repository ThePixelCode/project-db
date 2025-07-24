<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { Optional, type music } from "../lib/types";

    let { musicId }: { musicId: number } = $props();

    let music_state = $state<Optional<music>>(Optional.empty());

    invoke<music>("get_music", { id: musicId }).then((returned_music) => {
        music_state = Optional.of(returned_music);
    });
</script>

{#if music_state.is_present()}
    <div>
        <h2>{music_state.unwrap().name}</h2>
        <p><strong>Language:</strong> {music_state.unwrap().lang}</p>
        <p><strong>Category:</strong> {music_state.unwrap().category}</p>
        <p><strong>Published:</strong> {music_state.unwrap().published_year}</p>
        <p><strong>Duration:</strong> {music_state.unwrap().duration}</p>
    </div>
{:else}
    <div>loading...</div>
{/if}
