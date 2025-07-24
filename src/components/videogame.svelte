<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { Optional, type videogame } from "../lib/types";

    let { videogameId }: { videogameId: number } = $props();

    let videogame_state = $state<Optional<videogame>>(Optional.empty());

    invoke<videogame>("get_videogame", { id: videogameId }).then(
        (returned_videogame) => {
            videogame_state = Optional.of(returned_videogame);
        },
    );
</script>

{#if videogame_state.is_present()}
    <div>
        <h2>{videogame_state.unwrap().name}</h2>
        <p><strong>Language:</strong> {videogame_state.unwrap().lang}</p>
        <p><strong>Category:</strong> {videogame_state.unwrap().category}</p>
        <p>
            <strong>Published:</strong>
            {videogame_state.unwrap().published_year}
        </p>
        <p>
            <strong>Supports Console Controller:</strong>
            {videogame_state.unwrap().supports_console_controller
                ? "Yes"
                : "No"}
        </p>
        <p>
            <strong>PEGI:</strong>
            {videogame_state.unwrap().pegi}
        </p>
    </div>
{:else}
    <div>loading...</div>
{/if}
