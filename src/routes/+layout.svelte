<script lang="ts">
  import { HouseIcon } from "lucide-svelte";
  import SearchForm from "$lib/SearchForm.svelte";
</script>

<div class="flex min-h-screen">
  <!-- 左カラム (検索フォームと登録ボタン) -->
  <aside class="w-1/4 xl:w-1/5 p-4 bg-gray-200 flex flex-col justify-between">
    <div>
      <a href="/" class="ml-2 inline-block text-gray-400 hover:text-gray-700">
        <HouseIcon />
      </a>

      <SearchForm />

      <a href="/new">
        <button class="w-full bg-amber-500 text-white py-2 rounded mt-4"
          >Register</button
        >
      </a>
    </div>

    <div class="mt-4 text-sm text-gray-600">
      <p>© 2025 Jasmify</p>
      <a href="/license" class="hover:text-blue-500 hover:underline"
        >License Information</a
      >
    </div>
  </aside>

  <!-- 右カラム (検索結果または登録画面) -->
  <main class="w-3/4 xl:w-4/5 px-4 py-2 bg-gray-100">
    <slot />
  </main>
</div>
