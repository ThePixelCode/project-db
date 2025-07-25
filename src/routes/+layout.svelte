<script>
    import "../app.css";
</script>

<nav class="navbar">
    <ul class="flex gap-4 p-4 bg-gray-800 text-white">
        <li><a href="/book" class="hover:underline">Book</a></li>
        <li><a href="/music" class="hover:underline">Music</a></li>
        <li><a href="/videogame" class="hover:underline">Videogame</a></li>
    </ul>
</nav>

<slot />
