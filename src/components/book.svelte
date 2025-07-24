<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { Optional, type book } from "../lib/types";

    let { bookId }: { bookId: number } = $props();

    let book_state = $state<Optional<book>>(Optional.empty());

    invoke<book>("get_book", { id: bookId }).then((returned_book) => {
        book_state = Optional.of(returned_book);
    });
</script>

{#if book_state.is_present()}
    <div>
        <h2>{book_state.unwrap().name}</h2>
        <p><strong>Language:</strong> {book_state.unwrap().lang}</p>
        <p><strong>Category:</strong> {book_state.unwrap().category}</p>
        <p><strong>Published:</strong> {book_state.unwrap().published_year}</p>
        <p><strong>Pages:</strong> {book_state.unwrap().pages}</p>
    </div>
{:else}
    <div>loading...</div>
{/if}
