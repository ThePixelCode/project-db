<script>
    import "../app.css";
</script>

<nav class="navbar">
    <ul class="flex gap-4 p-4 bg-gray-800 text-white">
        <li><a href="/book" class="hover:underline">Book</a></li>
        <li><a href="/music" class="hover:underline">Music</a></li>
        <li><a href="/videogame" class="hover:underline">Videogame</a></li>
        <li><a href="/author" class="hover:underline">Author</a></li>
        <li><a href="/country" class="hover:underline">Country</a></li>
        <li><a href="/language" class="hover:underline">Language</a></li>
        <li><a href="/identifier" class="hover:underline">Identifier</a></li>
        <li><a href="/pegi" class="hover:underline">Pegi</a></li>
        <li><a href="/collection" class="hover:underline">Collection</a></li>
        <li><a href="/studio" class="hover:underline">Estudio</a></li>
        <li><a href="/distributor" class="hover:underline">Distribuidor</a></li>
        <li><a href="/genre" class="hover:underline">Genero</a></li>
    </ul>
</nav>

<slot />
